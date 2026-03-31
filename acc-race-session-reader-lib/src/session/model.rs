use serde::{Deserialize, Deserializer, Serialize};

use crate::{
  lap::model::Lap, session_result::model::SessionResult, session_type::model::SessionType,
  track::model::Track,
};

#[derive(Debug, Clone, Serialize)]
pub struct Session {
  pub session_type: SessionType,
  pub track_name: Track,
  pub session_index: u32,
  pub race_weekend_index: u32,
  pub meta_data: String,
  pub server_name: String,
  pub session_result: SessionResult,
  pub laps: Vec<Lap>,
  pub penalties: Vec<serde_json::Value>,
  pub post_race_penalties: Vec<serde_json::Value>,
}

// Helper struct for deserialization that handles duplicate fields
#[derive(Debug, Deserialize)]
struct SessionRaw {
  #[serde(rename = "sessionType")]
  session_type: SessionType,
  #[serde(rename = "trackName")]
  track_name: Track,
  #[serde(rename = "sessionIndex")]
  session_index: u32,
  #[serde(rename = "raceWeekendIndex")]
  race_weekend_index: u32,
  #[serde(rename = "metaData")]
  meta_data: String,
  #[serde(rename = "serverName")]
  server_name: String,
  #[serde(rename = "sessionResult")]
  session_result: SessionResult,
  laps: Vec<Lap>,
  penalties: Vec<serde_json::Value>,
  #[serde(rename = "post_race_penalties")]
  post_race_penalties: Vec<serde_json::Value>,
}

impl<'de> Deserialize<'de> for Session {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    // First deserialize to a serde_json::Value, which has last-key-wins for duplicate keys
    let value = serde_json::Value::deserialize(deserializer)?;
    // Then deserialize from the Value to our helper struct
    let raw: SessionRaw = serde_json::from_value(value).map_err(serde::de::Error::custom)?;

    Ok(Session {
      session_type: raw.session_type,
      track_name: raw.track_name,
      session_index: raw.session_index,
      race_weekend_index: raw.race_weekend_index,
      meta_data: raw.meta_data,
      server_name: raw.server_name,
      session_result: raw.session_result,
      laps: raw.laps,
      penalties: raw.penalties,
      post_race_penalties: raw.post_race_penalties,
    })
  }
}
