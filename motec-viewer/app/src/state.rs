use leptos::prelude::*;
use motec_viewer_lib::{LapInfo, LdSession};

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
