use acc_session_viewer_lib::car::car_model::CarModel;
use acc_session_viewer_lib::loaded_session::LoadedSession;
use leptos::prelude::*;
use motec_viewer_lib::{LapInfo, LdSession};

#[derive(Clone, Copy)]
pub struct SessionState {
  pub session: RwSignal<Option<LoadedSession>>,
  pub selected_car_model: RwSignal<Option<CarModel>>,
  pub error: RwSignal<Option<String>>,
}

impl SessionState {
  pub fn new() -> Self {
    Self {
      session: RwSignal::new(None),
      selected_car_model: RwSignal::new(None),
      error: RwSignal::new(None),
    }
  }
}

impl Default for SessionState {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Clone, Copy)]
pub struct TelemetryState {
  pub session: RwSignal<Option<LdSession>>,
  pub laps: RwSignal<Vec<LapInfo>>,
  pub error: RwSignal<Option<String>>,
}

impl TelemetryState {
  pub fn new() -> Self {
    Self {
      session: RwSignal::new(None),
      laps: RwSignal::new(vec![]),
      error: RwSignal::new(None),
    }
  }
}

impl Default for TelemetryState {
  fn default() -> Self {
    Self::new()
  }
}
