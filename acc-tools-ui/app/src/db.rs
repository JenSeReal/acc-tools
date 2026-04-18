//! Tauri command bridge – only active in CSR/hydrate (WASM) builds.

use serde::{Deserialize, Serialize};

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

// ── Tauri invoke binding (WASM only) ─────────────────────────────────────────

#[cfg(any(feature = "csr", feature = "hydrate"))]
mod bridge {
  use super::*;
  use acc_session_viewer_lib::loaded_session::LoadedSession;
  use wasm_bindgen::prelude::*;

  #[wasm_bindgen]
  extern "C" {
    /// Tauri 2 core invoke. `catch` makes async rejections surface as Err.
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
    // Tauri 2 converts camelCase JS keys → snake_case Rust params.
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
}

#[cfg(any(feature = "csr", feature = "hydrate"))]
pub use bridge::{delete_session, get_session, list_sessions, save_session};
