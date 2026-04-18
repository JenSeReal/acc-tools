use std::time::{SystemTime, UNIX_EPOCH};

use acc_session_viewer_lib::loaded_session::LoadedSession;
use serde_json::Value;
use surrealdb::Surreal;
use surrealdb::engine::local::Db;
use uuid::Uuid;

use crate::db::{SessionRecord, SessionSummary};

#[tauri::command]
pub async fn db_save_session(
  session: LoadedSession,
  file_name: String,
  state: tauri::State<'_, Surreal<Db>>,
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
  state: tauri::State<'_, Surreal<Db>>,
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
  state: tauri::State<'_, Surreal<Db>>,
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
  state: tauri::State<'_, Surreal<Db>>,
) -> Result<(), String> {
  let _: Option<Value> = state
    .delete(("sessions", id.as_str()))
    .await
    .map_err(|e| e.to_string())?;

  Ok(())
}
