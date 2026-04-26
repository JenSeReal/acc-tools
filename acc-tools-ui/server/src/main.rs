use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::{
  Json, Router,
  extract::{Path, State},
  http::StatusCode,
  response::IntoResponse,
  routing::{get, post},
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use surrealdb::opt::auth::Root;
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use uuid::Uuid;

use acc_session_viewer_lib::event::{Championship, ChampionshipSummary, Event, EventSummary};
use acc_session_viewer_lib::loaded_session::LoadedSession;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionRecord {
  session_id: String,
  session: LoadedSession,
  file_name: String,
  saved_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SessionSummary {
  id: String,
  track_name: String,
  session_type: String,
  car_count: usize,
  file_name: String,
  saved_at: u64,
}

impl From<&SessionRecord> for SessionSummary {
  fn from(r: &SessionRecord) -> Self {
    Self {
      id: r.session_id.clone(),
      track_name: serde_json::to_value(r.session.track_name)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", r.session.track_name)),
      session_type: serde_json::to_value(r.session.session_type)
        .ok()
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| format!("{:?}", r.session.session_type)),
      car_count: r.session.car_count,
      file_name: r.file_name.clone(),
      saved_at: r.saved_at,
    }
  }
}

#[derive(Clone)]
struct AppState {
  db: Arc<Surreal<Any>>,
}

#[derive(Deserialize)]
struct SaveSessionRequest {
  session: LoadedSession,
  file_name: String,
}

async fn save_session(
  State(state): State<AppState>,
  Json(body): Json<SaveSessionRequest>,
) -> impl IntoResponse {
  let id = Uuid::new_v4().to_string();
  let saved_at = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();

  let record = SessionRecord {
    session_id: id.clone(),
    session: body.session,
    file_name: body.file_name,
    saved_at,
  };

  let content: Value = match serde_json::to_value(&record) {
    Ok(v) => v,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": e.to_string() })),
      )
        .into_response();
    }
  };

  match state
    .db
    .create::<Option<Value>>(("sessions", id.clone()))
    .content(content)
    .await
  {
    Ok(_) => (StatusCode::OK, Json(json!({ "id": id }))).into_response(),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(json!({ "error": e.to_string() })),
    )
      .into_response(),
  }
}

async fn list_sessions(State(state): State<AppState>) -> impl IntoResponse {
  let values: Vec<Value> = match state.db.select("sessions").await {
    Ok(v) => v,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": e.to_string() })),
      )
        .into_response();
    }
  };

  let summaries: Vec<SessionSummary> = values
    .into_iter()
    .filter_map(|v| serde_json::from_value::<SessionRecord>(v).ok())
    .map(|r| SessionSummary::from(&r))
    .collect();

  (StatusCode::OK, Json(summaries)).into_response()
}

async fn get_session(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
  let value: Option<Value> = match state.db.select(("sessions", id.as_str())).await {
    Ok(v) => v,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": e.to_string() })),
      )
        .into_response();
    }
  };

  match value.and_then(|v| serde_json::from_value::<SessionRecord>(v).ok()) {
    Some(record) => (StatusCode::OK, Json(record.session)).into_response(),
    None => StatusCode::NOT_FOUND.into_response(),
  }
}

async fn delete_session(
  State(state): State<AppState>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  match state
    .db
    .delete::<Option<Value>>(("sessions", id.as_str()))
    .await
  {
    Ok(_) => StatusCode::NO_CONTENT.into_response(),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(json!({ "error": e.to_string() })),
    )
      .into_response(),
  }
}

// ── Event records ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
struct EventRecord {
  event_id: String,
  event: Event,
  saved_at: u64,
}

impl From<&EventRecord> for EventSummary {
  fn from(r: &EventRecord) -> Self {
    let uploaded = r.event.slots.iter().filter(|s| s.session.is_some()).count();
    Self {
      id: r.event_id.clone(),
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

async fn save_event(State(state): State<AppState>, Json(event): Json<Event>) -> impl IntoResponse {
  let id = Uuid::new_v4().to_string();
  let saved_at = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();

  let record = EventRecord {
    event_id: id.clone(),
    event,
    saved_at,
  };

  let content: Value = match serde_json::to_value(&record) {
    Ok(v) => v,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": e.to_string() })),
      )
        .into_response();
    }
  };

  match state
    .db
    .create::<Option<Value>>(("events", id.clone()))
    .content(content)
    .await
  {
    Ok(_) => (StatusCode::OK, Json(json!({ "id": id }))).into_response(),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(json!({ "error": e.to_string() })),
    )
      .into_response(),
  }
}

async fn list_events(State(state): State<AppState>) -> impl IntoResponse {
  let values: Vec<Value> = match state.db.select("events").await {
    Ok(v) => v,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": e.to_string() })),
      )
        .into_response();
    }
  };

  let summaries: Vec<EventSummary> = values
    .into_iter()
    .filter_map(|v| serde_json::from_value::<EventRecord>(v).ok())
    .map(|r| EventSummary::from(&r))
    .collect();

  (StatusCode::OK, Json(summaries)).into_response()
}

async fn get_event(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
  let value: Option<Value> = match state.db.select(("events", id.as_str())).await {
    Ok(v) => v,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": e.to_string() })),
      )
        .into_response();
    }
  };

  match value.and_then(|v| serde_json::from_value::<EventRecord>(v).ok()) {
    Some(record) => (StatusCode::OK, Json(record.event)).into_response(),
    None => StatusCode::NOT_FOUND.into_response(),
  }
}

async fn delete_event(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
  match state
    .db
    .delete::<Option<Value>>(("events", id.as_str()))
    .await
  {
    Ok(_) => StatusCode::NO_CONTENT.into_response(),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(json!({ "error": e.to_string() })),
    )
      .into_response(),
  }
}

// ── Championship records ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChampionshipRecord {
  championship_id: String,
  championship: Championship,
  saved_at: u64,
}

impl From<&ChampionshipRecord> for ChampionshipSummary {
  fn from(r: &ChampionshipRecord) -> Self {
    Self {
      id: r.championship_id.clone(),
      title: r.championship.title.clone(),
      event_count: r.championship.events.len(),
      saved_at: r.saved_at,
    }
  }
}

async fn save_championship(
  State(state): State<AppState>,
  Json(championship): Json<Championship>,
) -> impl IntoResponse {
  let id = Uuid::new_v4().to_string();
  let saved_at = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap_or_default()
    .as_secs();
  let record = ChampionshipRecord {
    championship_id: id.clone(),
    championship,
    saved_at,
  };
  let content: Value = match serde_json::to_value(&record) {
    Ok(v) => v,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": e.to_string() })),
      )
        .into_response();
    }
  };
  match state
    .db
    .create::<Option<Value>>(("championships", id.clone()))
    .content(content)
    .await
  {
    Ok(_) => (StatusCode::OK, Json(json!({ "id": id }))).into_response(),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(json!({ "error": e.to_string() })),
    )
      .into_response(),
  }
}

async fn list_championships(State(state): State<AppState>) -> impl IntoResponse {
  let values: Vec<Value> = match state.db.select("championships").await {
    Ok(v) => v,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": e.to_string() })),
      )
        .into_response();
    }
  };
  let summaries: Vec<ChampionshipSummary> = values
    .into_iter()
    .filter_map(|v| serde_json::from_value::<ChampionshipRecord>(v).ok())
    .map(|r| ChampionshipSummary::from(&r))
    .collect();
  (StatusCode::OK, Json(summaries)).into_response()
}

async fn get_championship(
  State(state): State<AppState>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let value: Option<Value> = match state.db.select(("championships", id.as_str())).await {
    Ok(v) => v,
    Err(e) => {
      return (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json(json!({ "error": e.to_string() })),
      )
        .into_response();
    }
  };
  match value.and_then(|v| serde_json::from_value::<ChampionshipRecord>(v).ok()) {
    Some(record) => (StatusCode::OK, Json(record.championship)).into_response(),
    None => StatusCode::NOT_FOUND.into_response(),
  }
}

async fn delete_championship(
  State(state): State<AppState>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  match state
    .db
    .delete::<Option<Value>>(("championships", id.as_str()))
    .await
  {
    Ok(_) => StatusCode::NO_CONTENT.into_response(),
    Err(e) => (
      StatusCode::INTERNAL_SERVER_ERROR,
      Json(json!({ "error": e.to_string() })),
    )
      .into_response(),
  }
}

async fn shutdown_signal() {
  let ctrl_c = async {
    tokio::signal::ctrl_c()
      .await
      .expect("failed to install Ctrl+C handler");
  };
  let terminate = async {
    tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
      .expect("failed to install SIGTERM handler")
      .recv()
      .await;
  };
  tokio::select! {
    _ = ctrl_c => {},
    _ = terminate => {},
  }
}

#[tokio::main]
async fn main() {
  let db_mode = std::env::var("DB_MODE").unwrap_or_else(|_| "file".to_string());
  let static_dir = std::env::var("STATIC_DIR").unwrap_or_else(|_| "./dist".to_string());
  let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
  let addr = format!("0.0.0.0:{port}");
  let db_ns = std::env::var("DATABASE_NS").unwrap_or_else(|_| "acc_tools".to_string());
  let db_db = std::env::var("DATABASE_DB").unwrap_or_else(|_| "sessions".to_string());

  let db: Surreal<Any> = match db_mode.as_str() {
    "memory" => {
      println!("DB mode: memory");
      surrealdb::engine::any::connect("mem://")
        .await
        .expect("Failed to open in-memory SurrealDB")
    }
    "server" => {
      let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set for server mode");
      let user = std::env::var("DATABASE_USER").unwrap_or_else(|_| "root".to_string());
      let pass = std::env::var("DATABASE_PASS").unwrap_or_else(|_| "root".to_string());
      println!("DB mode: server ({url})");
      let db = surrealdb::engine::any::connect(url)
        .await
        .expect("Failed to connect to SurrealDB server");
      db.signin(Root {
        username: user,
        password: pass,
      })
      .await
      .expect("Failed to authenticate with SurrealDB");
      db
    }
    _ => {
      let path = std::env::var("DATABASE_PATH").unwrap_or_else(|_| "./sessions.db".to_string());
      println!("DB mode: file ({path})");
      surrealdb::engine::any::connect(format!("surrealkv://{path}"))
        .await
        .expect("Failed to open SurrealDB file")
    }
  };

  db.use_ns(db_ns)
    .use_db(db_db)
    .await
    .expect("Failed to select namespace/database");

  let state = AppState { db: Arc::new(db) };

  let api_routes = Router::new()
    .route("/sessions", post(save_session).get(list_sessions))
    .route("/sessions/{id}", get(get_session).delete(delete_session))
    .route("/events", post(save_event).get(list_events))
    .route("/events/{id}", get(get_event).delete(delete_event))
    .route(
      "/championships",
      post(save_championship).get(list_championships),
    )
    .route(
      "/championships/{id}",
      get(get_championship).delete(delete_championship),
    );

  let app = Router::new()
    .nest("/api", api_routes)
    .fallback_service(ServeDir::new(&static_dir))
    .layer(CorsLayer::permissive())
    .with_state(state);

  let listener = tokio::net::TcpListener::bind(&addr)
    .await
    .unwrap_or_else(|_| panic!("Failed to bind to {addr}"));

  println!("Listening on {addr}, serving static files from {static_dir}");
  axum::serve(listener, app)
    .with_graceful_shutdown(shutdown_signal())
    .await
    .expect("Server error");
}
