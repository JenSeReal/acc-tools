use crate::driver::model::Driver;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Car {
  #[serde(rename = "carId")]
  pub car_id: u32,
  #[serde(rename = "raceNumber")]
  pub race_number: u32,
  #[serde(rename = "carModel")]
  pub car_model: u32,
  #[serde(rename = "cupCategory")]
  pub cup_category: u32,
  #[serde(rename = "carGroup")]
  pub car_group: String,
  #[serde(rename = "teamName")]
  pub team_name: String,
  pub nationality: i32,
  #[serde(rename = "carGuid")]
  pub car_guid: i32,
  #[serde(rename = "teamGuid")]
  pub team_guid: i32,
  pub drivers: Vec<Driver>,
  #[serde(rename = "ballastKg")]
  pub ballast_kg: i32,
}
