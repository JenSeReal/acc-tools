use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Driver {
  #[serde(rename = "firstName")]
  pub first_name: String,
  #[serde(rename = "lastName")]
  pub last_name: String,
  #[serde(rename = "shortName")]
  pub short_name: String,
  #[serde(rename = "playerId")]
  pub player_id: String,
}
