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
