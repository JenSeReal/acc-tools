use crate::{
  car::car_model::CarModel,
  driver_stats::{
    DriverStats, DriverStatsCommon, calculate_driver_stats, calculate_driver_stats_from_qualifying,
    get_unique_car_models,
  },
  qualifying_session::QualifyingSession,
  session::model::Session,
  session_type::model::SessionType,
  track::model::Track,
};
use serde::{Deserialize, Serialize};

/// A unified session representation that can be created from either
/// JSON session files or PDF qualifying results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadedSession {
  pub session_type: SessionType,
  pub track_name: Track,
  pub car_count: usize,
  pub stats: Vec<DriverStats>,
  pub car_models: Vec<CarModel>,
}

impl LoadedSession {
  /// Create from a JSON-parsed Session
  pub fn from_session(session: &Session) -> Self {
    let stats = calculate_driver_stats(session);
    let car_models = get_unique_car_models(&stats);

    Self {
      session_type: session.session_type,
      track_name: session.track_name,
      car_count: session.session_result.leader_board_lines.len(),
      stats,
      car_models,
    }
  }

  /// Create from a PDF-parsed QualifyingSession
  pub fn from_qualifying(qualifying: &QualifyingSession) -> Self {
    let stats = calculate_driver_stats_from_qualifying(qualifying);
    let car_models = get_unique_car_models(&stats);

    Self {
      session_type: qualifying.session_type,
      track_name: qualifying.track_name,
      car_count: qualifying.entries.len(),
      stats,
      car_models,
    }
  }

  /// Filter stats by car model
  pub fn stats_for_car_model(&self, car_model: Option<CarModel>) -> Vec<&DriverStats> {
    match car_model {
      Some(model) => self
        .stats
        .iter()
        .filter(|s| s.car_model() == model)
        .collect(),
      None => self.stats.iter().collect(),
    }
  }
}
