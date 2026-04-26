//! Database bridge – Tauri invoke (desktop) or HTTP fetch (web).

use serde::{Deserialize, Serialize};

pub use acc_session_viewer_lib::event::{ChampionshipSummary, EventSummary};

/// Lightweight session summary returned by the list command.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSummary {
  pub id: String,
  pub track_name: String,
  pub session_type: String,
  pub car_count: usize,
  pub file_name: String,
  pub saved_at: u64,
}

// ── Tauri invoke bridge (desktop WASM, no `web` feature) ─────────────────────

#[cfg(all(any(feature = "csr", feature = "hydrate"), not(feature = "web")))]
mod tauri_bridge {
  use super::*;
  use acc_session_viewer_lib::event::{Championship, Event};
  use acc_session_viewer_lib::loaded_session::LoadedSession;
  use wasm_bindgen::prelude::*;

  #[wasm_bindgen]
  extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], catch)]
    pub async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
  }

  fn to_js(value: &impl Serialize) -> Result<JsValue, String> {
    let json = serde_json::to_string(value).map_err(|e| e.to_string())?;
    js_sys::JSON::parse(&json).map_err(|e| format!("{e:?}"))
  }

  fn from_js<T: for<'de> Deserialize<'de>>(value: JsValue) -> Result<T, String> {
    let json = js_sys::JSON::stringify(&value)
      .map_err(|e| format!("{e:?}"))?
      .as_string()
      .ok_or_else(|| "JSON.stringify returned None".to_string())?;
    serde_json::from_str(&json).map_err(|e| e.to_string())
  }

  fn js_err(e: JsValue) -> String {
    js_sys::JSON::stringify(&e)
      .ok()
      .and_then(|s| s.as_string())
      .unwrap_or_else(|| format!("{e:?}"))
  }

  pub async fn save_session(session: &LoadedSession, file_name: &str) -> Result<String, String> {
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args<'a> {
      session: &'a LoadedSession,
      file_name: &'a str,
    }
    let args = to_js(&Args { session, file_name })?;
    let res = invoke("db_save_session", args).await.map_err(js_err)?;
    from_js(res)
  }

  pub async fn list_sessions() -> Result<Vec<SessionSummary>, String> {
    let args = to_js(&serde_json::json!({}))?;
    let res = invoke("db_list_sessions", args).await.map_err(js_err)?;
    from_js(res)
  }

  pub async fn get_session(id: &str) -> Result<Option<LoadedSession>, String> {
    #[derive(Serialize)]
    struct Args<'a> {
      id: &'a str,
    }
    let args = to_js(&Args { id })?;
    let res = invoke("db_get_session", args).await.map_err(js_err)?;
    from_js(res)
  }

  pub async fn delete_session(id: &str) -> Result<(), String> {
    #[derive(Serialize)]
    struct Args<'a> {
      id: &'a str,
    }
    let args = to_js(&Args { id })?;
    invoke("db_delete_session", args).await.map_err(js_err)?;
    Ok(())
  }

  pub async fn save_event(event: &Event) -> Result<String, String> {
    #[derive(Serialize)]
    struct Args<'a> {
      event: &'a Event,
    }
    let args = to_js(&Args { event })?;
    let res = invoke("db_save_event", args).await.map_err(js_err)?;
    from_js(res)
  }

  pub async fn list_events() -> Result<Vec<EventSummary>, String> {
    let args = to_js(&serde_json::json!({}))?;
    let res = invoke("db_list_events", args).await.map_err(js_err)?;
    from_js(res)
  }

  pub async fn get_event(id: &str) -> Result<Option<Event>, String> {
    #[derive(Serialize)]
    struct Args<'a> {
      id: &'a str,
    }
    let args = to_js(&Args { id })?;
    let res = invoke("db_get_event", args).await.map_err(js_err)?;
    from_js(res)
  }

  pub async fn delete_event(id: &str) -> Result<(), String> {
    #[derive(Serialize)]
    struct Args<'a> {
      id: &'a str,
    }
    let args = to_js(&Args { id })?;
    invoke("db_delete_event", args).await.map_err(js_err)?;
    Ok(())
  }

  pub async fn save_championship(championship: &Championship) -> Result<String, String> {
    #[derive(Serialize)]
    struct Args<'a> {
      championship: &'a Championship,
    }
    let args = to_js(&Args { championship })?;
    let res = invoke("db_save_championship", args).await.map_err(js_err)?;
    from_js(res)
  }

  pub async fn list_championships() -> Result<Vec<ChampionshipSummary>, String> {
    let args = to_js(&serde_json::json!({}))?;
    let res = invoke("db_list_championships", args)
      .await
      .map_err(js_err)?;
    from_js(res)
  }

  pub async fn get_championship(id: &str) -> Result<Option<Championship>, String> {
    #[derive(Serialize)]
    struct Args<'a> {
      id: &'a str,
    }
    let args = to_js(&Args { id })?;
    let res = invoke("db_get_championship", args).await.map_err(js_err)?;
    from_js(res)
  }

  pub async fn delete_championship(id: &str) -> Result<(), String> {
    #[derive(Serialize)]
    struct Args<'a> {
      id: &'a str,
    }
    let args = to_js(&Args { id })?;
    invoke("db_delete_championship", args)
      .await
      .map_err(js_err)?;
    Ok(())
  }
}

// ── HTTP fetch bridge (web WASM, `web` feature enabled) ──────────────────────

#[cfg(feature = "web")]
mod http_bridge {
  use super::*;
  use acc_session_viewer_lib::event::{Championship, Event};
  use acc_session_viewer_lib::loaded_session::LoadedSession;
  use gloo_net::http::Request;

  pub async fn save_session(session: &LoadedSession, file_name: &str) -> Result<String, String> {
    #[derive(Serialize)]
    struct Body<'a> {
      session: &'a LoadedSession,
      file_name: &'a str,
    }
    #[derive(Deserialize)]
    struct Resp {
      id: String,
    }
    let resp = Request::post("/api/sessions")
      .json(&Body { session, file_name })
      .map_err(|e| e.to_string())?
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    let r: Resp = resp.json().await.map_err(|e| e.to_string())?;
    Ok(r.id)
  }

  pub async fn list_sessions() -> Result<Vec<SessionSummary>, String> {
    let resp = Request::get("/api/sessions")
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
  }

  pub async fn get_session(id: &str) -> Result<Option<LoadedSession>, String> {
    let resp = Request::get(&format!("/api/sessions/{id}"))
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if resp.status() == 404 {
      return Ok(None);
    }
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    let session: LoadedSession = resp.json().await.map_err(|e| e.to_string())?;
    Ok(Some(session))
  }

  pub async fn delete_session(id: &str) -> Result<(), String> {
    let resp = Request::delete(&format!("/api/sessions/{id}"))
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    Ok(())
  }

  pub async fn save_event(event: &Event) -> Result<String, String> {
    #[derive(Deserialize)]
    struct Resp {
      id: String,
    }
    let resp = Request::post("/api/events")
      .json(event)
      .map_err(|e| e.to_string())?
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    let r: Resp = resp.json().await.map_err(|e| e.to_string())?;
    Ok(r.id)
  }

  pub async fn list_events() -> Result<Vec<EventSummary>, String> {
    let resp = Request::get("/api/events")
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
  }

  pub async fn get_event(id: &str) -> Result<Option<Event>, String> {
    let resp = Request::get(&format!("/api/events/{id}"))
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if resp.status() == 404 {
      return Ok(None);
    }
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    let event: Event = resp.json().await.map_err(|e| e.to_string())?;
    Ok(Some(event))
  }

  pub async fn delete_event(id: &str) -> Result<(), String> {
    let resp = Request::delete(&format!("/api/events/{id}"))
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    Ok(())
  }

  pub async fn save_championship(championship: &Championship) -> Result<String, String> {
    #[derive(Deserialize)]
    struct Resp {
      id: String,
    }
    let resp = Request::post("/api/championships")
      .json(championship)
      .map_err(|e| e.to_string())?
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    let r: Resp = resp.json().await.map_err(|e| e.to_string())?;
    Ok(r.id)
  }

  pub async fn list_championships() -> Result<Vec<ChampionshipSummary>, String> {
    let resp = Request::get("/api/championships")
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    resp.json().await.map_err(|e| e.to_string())
  }

  pub async fn get_championship(id: &str) -> Result<Option<Championship>, String> {
    let resp = Request::get(&format!("/api/championships/{id}"))
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if resp.status() == 404 {
      return Ok(None);
    }
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    let c: Championship = resp.json().await.map_err(|e| e.to_string())?;
    Ok(Some(c))
  }

  pub async fn delete_championship(id: &str) -> Result<(), String> {
    let resp = Request::delete(&format!("/api/championships/{id}"))
      .send()
      .await
      .map_err(|e| e.to_string())?;
    if !resp.ok() {
      return Err(format!("HTTP {}", resp.status()));
    }
    Ok(())
  }
}

// ── Re-exports ────────────────────────────────────────────────────────────────

#[cfg(all(any(feature = "csr", feature = "hydrate"), not(feature = "web")))]
pub use tauri_bridge::{
  delete_championship, delete_event, delete_session, get_championship, get_event, get_session,
  list_championships, list_events, list_sessions, save_championship, save_event, save_session,
};

#[cfg(feature = "web")]
pub use http_bridge::{
  delete_championship, delete_event, delete_session, get_championship, get_event, get_session,
  list_championships, list_events, list_sessions, save_championship, save_event, save_session,
};
