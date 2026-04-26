use acc_session_viewer_lib::car::car_model::CarModel;
use acc_session_viewer_lib::event::{Championship, Event};
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

/// When an event is opened from within a championship, this provides the
/// context needed to save changes back to the championship record.
#[derive(Clone)]
pub struct ChampionshipEventCtx {
  /// DB id of the parent championship.
  pub id: String,
  /// Index of this event in `championship.events`.
  pub event_idx: usize,
}

#[derive(Clone, Copy)]
pub struct EventState {
  pub event: RwSignal<Option<Event>>,
  /// Sessions uploaded to slots, keyed by slot ID.
  pub slot_sessions: RwSignal<std::collections::HashMap<String, LoadedSession>>,
  pub error: RwSignal<Option<String>>,
  /// Set when the event was opened from a championship view.
  pub championship_ctx: RwSignal<Option<ChampionshipEventCtx>>,
}

impl EventState {
  pub fn new() -> Self {
    Self {
      event: RwSignal::new(None),
      slot_sessions: RwSignal::new(std::collections::HashMap::new()),
      error: RwSignal::new(None),
      championship_ctx: RwSignal::new(None),
    }
  }
}

impl Default for EventState {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Clone, Copy)]
pub struct ChampionshipState {
  /// The active championship together with its DB id.
  pub championship: RwSignal<Option<(String, Championship)>>,
}

impl ChampionshipState {
  pub fn new() -> Self {
    Self {
      championship: RwSignal::new(None),
    }
  }
}

impl Default for ChampionshipState {
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
