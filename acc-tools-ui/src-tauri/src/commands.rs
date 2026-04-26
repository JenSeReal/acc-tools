use std::time::{SystemTime, UNIX_EPOCH};

use acc_session_viewer_lib::event::{Championship, ChampionshipSummary, Event, EventSummary};
use acc_session_viewer_lib::loaded_session::LoadedSession;
use serde_json::Value;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use uuid::Uuid;

use crate::db::{ChampionshipRecord, EventRecord, SessionRecord, SessionSummary};

#[tauri::command]
pub async fn db_save_session(
  session: LoadedSession,
  file_name: String,
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<String, String> {
  let id = Uuid::new_v4().to_string();
  let saved_at = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();

  let record = SessionRecord {
    id: id.clone(),
    session,
    file_name,
    saved_at,
  };

  let content: Value = serde_json::to_value(record).map_err(|e| e.to_string())?;
  let _: Option<Value> = state
    .create(("sessions", id.clone()))
    .content(content)
    .await
    .map_err(|e| e.to_string())?;

  Ok(id)
}

#[tauri::command]
pub async fn db_list_sessions(
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<Vec<SessionSummary>, String> {
  let values: Vec<Value> = state.select("sessions").await.map_err(|e| e.to_string())?;

  let records: Vec<SessionRecord> = values
    .into_iter()
    .filter_map(|v| serde_json::from_value(v).ok())
    .collect();

  Ok(records.iter().map(SessionSummary::from).collect())
}

#[tauri::command]
pub async fn db_get_session(
  id: String,
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<Option<LoadedSession>, String> {
  let value: Option<Value> = state
    .select(("sessions", id.as_str()))
    .await
    .map_err(|e| e.to_string())?;

  let record: Option<SessionRecord> = value.and_then(|v| serde_json::from_value(v).ok());
  Ok(record.map(|r| r.session))
}

#[tauri::command]
pub async fn db_delete_session(
  id: String,
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<(), String> {
  let _: Option<Value> = state
    .delete(("sessions", id.as_str()))
    .await
    .map_err(|e| e.to_string())?;

  Ok(())
}

#[tauri::command]
pub async fn db_save_event(
  event: Event,
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<String, String> {
  let id = Uuid::new_v4().to_string();
  let saved_at = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();

  let record = EventRecord {
    id: id.clone(),
    event,
    saved_at,
  };

  let content: Value = serde_json::to_value(record).map_err(|e| e.to_string())?;
  let _: Option<Value> = state
    .create(("events", id.clone()))
    .content(content)
    .await
    .map_err(|e| e.to_string())?;

  Ok(id)
}

#[tauri::command]
pub async fn db_list_events(
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<Vec<EventSummary>, String> {
  let values: Vec<Value> = state.select("events").await.map_err(|e| e.to_string())?;

  let records: Vec<EventRecord> = values
    .into_iter()
    .filter_map(|v| serde_json::from_value(v).ok())
    .collect();

  Ok(records.iter().map(EventSummary::from).collect())
}

#[tauri::command]
pub async fn db_get_event(
  id: String,
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<Option<Event>, String> {
  let value: Option<Value> = state
    .select(("events", id.as_str()))
    .await
    .map_err(|e| e.to_string())?;

  let record: Option<EventRecord> = value.and_then(|v| serde_json::from_value(v).ok());
  Ok(record.map(|r| r.event))
}

#[tauri::command]
pub async fn db_delete_event(
  id: String,
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<(), String> {
  let _: Option<Value> = state
    .delete(("events", id.as_str()))
    .await
    .map_err(|e| e.to_string())?;

  Ok(())
}

// ── Championship commands ─────────────────────────────────────────────────────

#[tauri::command]
pub async fn db_save_championship(
  championship: Championship,
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<String, String> {
  let id = Uuid::new_v4().to_string();
  let saved_at = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();

  let record = ChampionshipRecord {
    id: id.clone(),
    championship,
    saved_at,
  };
  let content: Value = serde_json::to_value(record).map_err(|e| e.to_string())?;
  let _: Option<Value> = state
    .create(("championships", id.clone()))
    .content(content)
    .await
    .map_err(|e| e.to_string())?;

  Ok(id)
}

#[tauri::command]
pub async fn db_list_championships(
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<Vec<ChampionshipSummary>, String> {
  let values: Vec<Value> = state
    .select("championships")
    .await
    .map_err(|e| e.to_string())?;
  let records: Vec<ChampionshipRecord> = values
    .into_iter()
    .filter_map(|v| serde_json::from_value(v).ok())
    .collect();
  Ok(records.iter().map(ChampionshipSummary::from).collect())
}

#[tauri::command]
pub async fn db_get_championship(
  id: String,
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<Option<Championship>, String> {
  let value: Option<Value> = state
    .select(("championships", id.as_str()))
    .await
    .map_err(|e| e.to_string())?;
  let record: Option<ChampionshipRecord> = value.and_then(|v| serde_json::from_value(v).ok());
  Ok(record.map(|r| r.championship))
}

#[tauri::command]
pub async fn db_delete_championship(
  id: String,
  state: tauri::State<'_, Surreal<Any>>,
) -> Result<(), String> {
  let _: Option<Value> = state
    .delete(("championships", id.as_str()))
    .await
    .map_err(|e| e.to_string())?;
  Ok(())
}
