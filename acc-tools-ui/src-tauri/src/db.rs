use acc_session_viewer_lib::loaded_session::LoadedSession;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::local::{Db, SurrealKv};

/// A session record as stored in SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionRecord {
  pub id: String,
  pub session: LoadedSession,
  pub file_name: String,
  pub saved_at: u64,
}

/// Lightweight summary returned by the list command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
  pub id: String,
  pub track_name: String,
  pub session_type: String,
  pub car_count: usize,
  pub file_name: String,
  pub saved_at: u64,
}

impl From<&SessionRecord> for SessionSummary {
  fn from(r: &SessionRecord) -> Self {
    Self {
      id: r.id.clone(),
      track_name: serde_json::to_value(&r.session.track_name)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", r.session.track_name)),
      session_type: serde_json::to_value(&r.session.session_type)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", r.session.session_type)),
      car_count: r.session.car_count,
      file_name: r.file_name.clone(),
      saved_at: r.saved_at,
    }
  }
}

pub async fn init_db(path: &str) -> Result<Surreal<Db>, surrealdb::Error> {
  let db = Surreal::new::<SurrealKv>(path).await?;
  db.use_ns("acc_tools").use_db("sessions").await?;
  Ok(db)
}
