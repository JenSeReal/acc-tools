use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timing {
  #[serde(rename = "lastLap", default)]
  pub last_lap: u32,
  #[serde(rename = "lastSplits", default)]
  pub last_splits: Vec<u32>,
  #[serde(rename = "bestLap", default)]
  pub best_lap: u32,
  #[serde(rename = "bestSplits", default)]
  pub best_splits: Vec<u32>,
  #[serde(rename = "totalTime", default)]
  pub total_time: u64,
  #[serde(rename = "lapCount", default)]
  pub lap_count: u32,
  #[serde(rename = "lastSplitId", default)]
  pub last_split_id: u32,
  /// Average lap time (used in qualifying results from PDF)
  #[serde(
    rename = "averageLap",
    default,
    skip_serializing_if = "Option::is_none"
  )]
  pub average_lap: Option<u32>,
  /// Result lap time (used in qualifying results from PDF)
  #[serde(rename = "resultLap", default, skip_serializing_if = "Option::is_none")]
  pub result_lap: Option<u32>,
}

impl Timing {
  pub fn average_lap_time(&self) -> f64 {
    if self.lap_count == 0 {
      return 0.0;
    }
    self.total_time as f64 / self.lap_count as f64
  }

  pub fn format_lap_time(milliseconds: u32) -> String {
    let total_seconds = milliseconds / 1000;
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let millis = milliseconds % 1000;
    format!("{:02}:{:02}.{:03}", minutes, seconds, millis)
  }

  pub fn best_lap_formatted(&self) -> String {
    Self::format_lap_time(self.best_lap)
  }

  pub fn last_lap_formatted(&self) -> String {
    Self::format_lap_time(self.last_lap)
  }

  pub fn best_split_sum(&self) -> u32 {
    self.best_splits.iter().sum()
  }

  pub fn last_split_sum(&self) -> u32 {
    self.last_splits.iter().sum()
  }

  pub fn gap_to_best(&self, best_lap: u32) -> i32 {
    self.best_lap as i32 - best_lap as i32
  }

  pub fn is_improving(&self) -> bool {
    self.last_lap < self.best_lap
  }
}
