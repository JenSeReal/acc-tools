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
