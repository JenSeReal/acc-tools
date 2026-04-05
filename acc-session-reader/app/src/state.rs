use acc_session_reader_lib::car::car_model::CarModel;
use acc_session_reader_lib::loaded_session::LoadedSession;
use leptos::prelude::*;

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
