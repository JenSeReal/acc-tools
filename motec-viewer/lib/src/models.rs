/// Parsed MoTeC telemetry session from a .ld file.
#[derive(Debug, Clone)]
pub struct LdSession {
  pub driver: String,
  pub vehicle_id: String,
  pub venue: String,
  pub date: String,
  pub time_of_day: String,
  pub short_comment: String,
  pub device_type: String,
  pub event_name: String,
  pub event_session: String,
  pub channels: Vec<LdChannel>,
}

/// A single telemetry channel with pre-computed statistics and full sample data.
#[derive(Debug, Clone)]
pub struct LdChannel {
  pub name: String,
  pub short_name: String,
  pub unit: String,
  /// Recording frequency in Hz.
  pub frequency: u16,
  /// Total number of samples in the file.
  pub sample_count: u32,
  pub min: f64,
  pub max: f64,
  pub avg: f64,
  /// Full decoded sample array (physical engineering values, one per recorded sample).
  pub samples: Vec<f64>,
}

impl LdChannel {
  pub fn duration_s(&self) -> f64 {
    if self.frequency == 0 {
      0.0
    } else {
      self.sample_count as f64 / self.frequency as f64
    }
  }

  /// Return the decoded sample at `lap_start_s + cursor_s` into the session.
  pub fn sample_at_lap(&self, lap_start_s: f64, cursor_s: f64) -> Option<f64> {
    let si = ((lap_start_s + cursor_s) * self.frequency as f64) as usize;
    self.samples.get(si).copied()
  }
}

impl LdSession {
  /// Find the first channel whose name matches `name` exactly.
  pub fn find_channel(&self, name: &str) -> Option<&LdChannel> {
    self.channels.iter().find(|c| c.name == name)
  }
}

/// Return the global `(min, max)` of `ch` across all given `(start_s, dur_s)` windows.
pub fn channel_range(ch: &LdChannel, windows: &[(f64, f64)]) -> (f64, f64) {
  let mut mn = f64::INFINITY;
  let mut mx = f64::NEG_INFINITY;
  for &(start_s, dur_s) in windows {
    let si = (start_s * ch.frequency as f64) as usize;
    let ei = ((start_s + dur_s) * ch.frequency as f64) as usize;
    let si = si.min(ch.samples.len());
    let ei = ei.min(ch.samples.len());
    for &v in &ch.samples[si..ei] {
      if v.is_finite() {
        mn = mn.min(v);
        mx = mx.max(v);
      }
    }
  }
  (mn, mx)
}

/// A lap boundary extracted from the .ldx XML sidecar.
#[derive(Debug, Clone)]
pub struct LapInfo {
  pub lap_number: usize,
  /// Cumulative time from session start (seconds).
  pub start_time_s: f64,
  /// Duration of this lap (None for the last lap if no end marker follows).
  pub duration_s: Option<f64>,
}

/// Returns `(start_offset_s, duration_s)` for the pure racing portion of a lap.
///
/// - Normal racing lap  → `(0.0, actual_dur)`
/// - In-lap (pits at end) → `(0.0, time_to_pit_entry)`
/// - Out-lap (pits at start) → `(time_of_pit_exit, remaining_dur)`
/// - Combined in+out    → trimmed on both sides
///
/// Detection uses the SPEED channel: sustained speed below `80 km/h` for ≥ 4 s
/// is treated as pit-lane driving. `median_racing_s` is used to avoid searching
/// for pit entry too early in the lap.
pub fn racing_window(lap: &LapInfo, speed_ch: &LdChannel, median_racing_s: f64) -> (f64, f64) {
  let actual_dur = match lap.duration_s {
    Some(d) => d,
    None => return (0.0, median_racing_s),
  };

  let freq = speed_ch.frequency as f64;
  if freq == 0.0 || speed_ch.samples.is_empty() {
    return (0.0, actual_dur);
  }

  // Only attempt trimming for laps that are significantly longer than median.
  if actual_dur <= median_racing_s * 1.4 {
    return (0.0, actual_dur);
  }

  let start_i = (lap.start_time_s * freq) as usize;
  let end_i = (start_i + (actual_dur * freq) as usize).min(speed_ch.samples.len());
  if start_i >= end_i {
    return (0.0, actual_dur);
  }

  let samples = &speed_ch.samples[start_i..end_i];
  let pit_threshold = 80.0_f64 / 3.6; // 80 km/h in m/s
  let min_low = ((4.0 * freq) as usize).max(1);

  // ── Out-lap: pit-lane period at the very start ──────────────────────────────
  let start_offset_s = {
    // Count how many samples at the start are below threshold
    let initial_low: usize = samples.iter().take_while(|&&v| v < pit_threshold).count();
    if initial_low >= min_low {
      // Find where sustained high speed begins after the initial slow period
      let mut high_count = 0usize;
      let mut exit_i = initial_low;
      for (i, &v) in samples.iter().enumerate().skip(initial_low) {
        if v >= pit_threshold {
          high_count += 1;
          if high_count >= min_low {
            exit_i = i + 1 - high_count;
            break;
          }
        } else {
          high_count = 0;
        }
      }
      exit_i as f64 / freq
    } else {
      0.0
    }
  };

  // ── In-lap: pit-lane period near the end ────────────────────────────────────
  // Only search after start_offset + 70 % of median to avoid false positives.
  let search_start_s = start_offset_s + median_racing_s * 0.7;
  let search_i = (search_start_s * freq) as usize;
  let end_offset_s = if search_i < samples.len() {
    let mut low_count = 0usize;
    let mut low_start = search_i;
    let mut found: Option<f64> = None;
    for (i, &v) in samples.iter().enumerate().skip(search_i) {
      if v < pit_threshold {
        if low_count == 0 {
          low_start = i;
        }
        low_count += 1;
        if low_count >= min_low {
          found = Some(low_start as f64 / freq);
          break;
        }
      } else {
        low_count = 0;
      }
    }
    found.unwrap_or(actual_dur)
  } else {
    actual_dur
  };

  let duration = (end_offset_s - start_offset_s).max(0.0);
  (start_offset_s, duration)
}

/// Filter `laps` to only racing laps, removing pit-stop laps and reset/abort laps.
///
/// Strategy: use the median complete-lap duration as a reference and keep only
/// laps within [median × 0.5, median × 1.5]. This tolerates yellow-flag laps
/// (~20 % slower) while reliably discarding in-laps (typically 2× or more) and
/// ESC-reset laps (typically < 50 % of a normal lap). Laps with no recorded
/// duration (last incomplete lap) are always excluded.
pub fn filter_racing_laps(laps: &[LapInfo]) -> Vec<LapInfo> {
  let mut durations: Vec<f64> = laps.iter().filter_map(|l| l.duration_s).collect();
  if durations.is_empty() {
    return vec![];
  }
  durations.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
  let median = durations[durations.len() / 2];
  let lo = median * 0.5;
  let hi = median * 1.5;
  laps
    .iter()
    .filter(|l| l.duration_s.map(|d| d >= lo && d <= hi).unwrap_or(false))
    .cloned()
    .collect()
}
