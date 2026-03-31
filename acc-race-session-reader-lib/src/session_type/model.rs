use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SessionType {
  #[serde(rename = "FP")]
  FreePractice,
  #[serde(rename = "Q")]
  Qualifying,
  #[serde(rename = "R")]
  Race,
}
