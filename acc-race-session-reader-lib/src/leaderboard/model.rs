use crate::{car::model::Car, driver::model::Driver, timing::model::Timing};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderBoardLine {
  pub car: Car,
  #[serde(rename = "currentDriver")]
  pub current_driver: Driver,
  #[serde(rename = "currentDriverIndex")]
  pub current_driver_index: u32,
  pub timing: Timing,
  #[serde(rename = "missingMandatoryPitstop")]
  pub missing_mandatory_pitstop: i32,
  #[serde(rename = "driverTotalTimes")]
  pub driver_total_times: Vec<u64>,
}

impl LeaderBoardLine {
  pub fn is_current_driver(&self, player_id: &str) -> bool {
    self.current_driver.player_id == player_id
  }

  pub fn gap_to_best(&self, best_lap: u32) -> i32 {
    self.timing.best_lap as i32 - best_lap as i32
  }

  pub fn has_mandatory_pitstop(&self) -> bool {
    self.missing_mandatory_pitstop == 0
  }

  pub fn needs_pitstop(&self) -> bool {
    self.missing_mandatory_pitstop > 0
  }

  pub fn driver_stint_time(&self, driver_index: usize) -> Option<u64> {
    self.driver_total_times.get(driver_index).copied()
  }

  pub fn total_driver_stint_time(&self) -> u64 {
    self.driver_total_times.iter().sum()
  }
}
