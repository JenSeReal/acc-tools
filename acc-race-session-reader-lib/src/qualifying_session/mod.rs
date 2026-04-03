use serde::Serialize;

use crate::{car::car_model::CarModel, session_type::model::SessionType, track::model::Track};

/// A qualifying session parsed from PDF results.
///
/// Unlike the JSON `Session` type, this doesn't contain individual lap data,
/// only the summarized qualifying results.
#[derive(Debug, Clone, Serialize)]
pub struct QualifyingSession {
  pub track_name: Track,
  pub session_type: SessionType,
  pub entries: Vec<QualifyingEntry>,
}

impl QualifyingSession {
  pub fn new(track_name: Track, entries: Vec<QualifyingEntry>) -> Self {
    Self {
      track_name,
      session_type: SessionType::Qualifying,
      entries,
    }
  }

  pub fn get_by_position(&self, position: u32) -> Option<&QualifyingEntry> {
    self.entries.iter().find(|e| e.position == position)
  }

  pub fn qualified_entries(&self) -> impl Iterator<Item = &QualifyingEntry> {
    self.entries.iter().filter(|e| e.qualified)
  }

  pub fn disqualified_entries(&self) -> impl Iterator<Item = &QualifyingEntry> {
    self.entries.iter().filter(|e| !e.qualified)
  }
}

/// A single entry in a qualifying session.
#[derive(Debug, Clone, Serialize)]
pub struct QualifyingEntry {
  pub position: u32,
  pub race_number: u32,
  pub driver_name: String,
  pub car_model: CarModel,
  pub timing: QualifyingTiming,
  pub valid_consecutive_laps: bool,
  pub lap_count: u32,
  pub qualified: bool,
}

/// Timing information for a qualifying entry.
///
/// All times are in milliseconds. A value of `None` indicates
/// "No valid time" (the driver didn't set a valid lap).
#[derive(Debug, Clone, Serialize)]
pub struct QualifyingTiming {
  /// Best single lap time in milliseconds
  pub best_lap: Option<u32>,
  /// Average lap time ("Durchschnitt") in milliseconds
  pub average_lap: Option<u32>,
  /// Result lap time ("Ergebnis") in milliseconds
  pub result_lap: Option<u32>,
}

impl QualifyingTiming {
  pub fn new(best_lap: Option<u32>, average_lap: Option<u32>, result_lap: Option<u32>) -> Self {
    Self {
      best_lap,
      average_lap,
      result_lap,
    }
  }

  pub fn no_valid_time() -> Self {
    Self {
      best_lap: None,
      average_lap: None,
      result_lap: None,
    }
  }

  pub fn has_valid_time(&self) -> bool {
    self.best_lap.is_some()
  }
}
