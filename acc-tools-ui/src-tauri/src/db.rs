use acc_session_viewer_lib::event::{Championship, ChampionshipSummary, Event, EventSummary};
use acc_session_viewer_lib::loaded_session::LoadedSession;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;

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

/// An event record as stored in SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRecord {
  pub id: String,
  pub event: Event,
  pub saved_at: u64,
}

impl From<&EventRecord> for EventSummary {
  fn from(r: &EventRecord) -> Self {
    let uploaded = r.event.slots.iter().filter(|s| s.session.is_some()).count();
    Self {
      id: r.id.clone(),
      title: r.event.title.clone(),
      event_type: r.event.event_type.label().to_string(),
      start_date: r.event.start_date.clone(),
      end_date: r.event.end_date.clone(),
      slot_count: r.event.slots.len(),
      uploaded_count: uploaded,
      saved_at: r.saved_at,
    }
  }
}

/// A championship record as stored in SurrealDB.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChampionshipRecord {
  pub id: String,
  pub championship: Championship,
  pub saved_at: u64,
}

impl From<&ChampionshipRecord> for ChampionshipSummary {
  fn from(r: &ChampionshipRecord) -> Self {
    Self {
      id: r.id.clone(),
      title: r.championship.title.clone(),
      event_count: r.championship.events.len(),
      saved_at: r.saved_at,
    }
  }
}

/// Initialise the database.  Mode is controlled by the `DB_MODE` env var:
///   `memory`  — in-process SurrealDB (ephemeral, useful for development)
///   `file`    — SurrealKV on disk at `path` (default)
///   `server`  — remote SurrealDB over WebSocket; `path` is ignored,
///               `DATABASE_URL` / `DATABASE_USER` / `DATABASE_PASS` are read instead
pub async fn init_db(path: &str) -> Result<Surreal<Any>, surrealdb::Error> {
  let mode = std::env::var("DB_MODE").unwrap_or_else(|_| "file".to_string());
  let db: Surreal<Any> = match mode.as_str() {
    "memory" => surrealdb::engine::any::connect("mem://").await?,
    "server" => {
      let url = std::env::var("DATABASE_URL").unwrap_or_else(|_| "ws://localhost:8000".to_string());
      let user = std::env::var("DATABASE_USER").unwrap_or_else(|_| "root".to_string());
      let pass = std::env::var("DATABASE_PASS").unwrap_or_else(|_| "root".to_string());
      let db = surrealdb::engine::any::connect(url).await?;
      db.signin(Root {
        username: user,
        password: pass,
      })
      .await?;
      db
    }
    _ => surrealdb::engine::any::connect(format!("surrealkv://{path}")).await?,
  };
  db.use_ns("acc_tools").use_db("sessions").await?;
  Ok(db)
}
