use serde::{Deserialize, Serialize};

use crate::loaded_session::LoadedSession;
use crate::session_type::model::SessionType;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum EventType {
  #[default]
  RaceWeekend,
  TestDay,
  TrackDay,
  FreePractice,
}

impl EventType {
  pub fn label(&self) -> &'static str {
    match self {
      Self::RaceWeekend => "Race Weekend",
      Self::TestDay => "Test Day",
      Self::TrackDay => "Track Day",
      Self::FreePractice => "Free Practice",
    }
  }

  pub fn all() -> &'static [Self] {
    &[
      Self::RaceWeekend,
      Self::TestDay,
      Self::TrackDay,
      Self::FreePractice,
    ]
  }
}

/// A single predetermined time slot in an event schedule.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleSlot {
  /// Stable identifier for reactive keying (UUID).
  pub id: String,
  /// User-defined label, e.g. "Free Practice 1", "Qualifying", "Race".
  pub label: String,
  /// Optional free-text description for this session.
  #[serde(default)]
  pub description: String,
  /// The ACC session type expected for this slot.
  pub session_type: SessionType,
  /// ISO datetime-local string for the start of this session, e.g. "2026-04-26T18:00".
  pub start_at: String,
  /// ISO datetime-local string for the end of this session.
  pub end_at: String,
  /// Session uploaded to this slot (None until uploaded).
  pub session: Option<LoadedSession>,
}

/// A motorsport event with a schedule of predetermined session slots.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
  pub title: String,
  pub event_type: EventType,
  /// ISO date string, e.g. "2026-04-26".
  pub start_date: String,
  /// ISO date string.
  pub end_date: String,
  pub slots: Vec<ScheduleSlot>,
}

impl Event {
  /// Returns slots sorted chronologically by `scheduled_at`.
  pub fn sorted_slot_indices(&self) -> Vec<usize> {
    let mut indices: Vec<usize> = (0..self.slots.len()).collect();
    indices.sort_by(|&a, &b| self.slots[a].start_at.cmp(&self.slots[b].start_at));
    indices
  }
}

/// A championship season grouping multiple events (race weekends).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Championship {
  pub title: String,
  pub events: Vec<Event>,
}

/// Lightweight championship summary for list views.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionshipSummary {
  pub id: String,
  pub title: String,
  pub event_count: usize,
  pub saved_at: u64,
}

/// Lightweight summary for list views (no session data).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventSummary {
  pub id: String,
  pub title: String,
  pub event_type: String,
  pub start_date: String,
  pub end_date: String,
  pub slot_count: usize,
  pub uploaded_count: usize,
  pub saved_at: u64,
}
