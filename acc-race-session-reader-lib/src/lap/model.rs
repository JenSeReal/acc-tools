use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lap {
  #[serde(rename = "carId")]
  pub car_id: u32,
  #[serde(rename = "driverIndex")]
  pub driver_index: u32,
  #[serde(rename = "laptime")]
  pub laptime: u32,
  #[serde(rename = "isValidForBest")]
  pub is_valid_for_best: bool,
  pub splits: Vec<u32>,
}
