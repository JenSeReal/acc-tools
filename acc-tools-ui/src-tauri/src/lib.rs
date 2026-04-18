mod commands;
mod db;

use commands::{db_delete_session, db_get_session, db_list_sessions, db_save_session};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_opener::init())
    .setup(|app| {
      use tauri::Manager;
      let db_path = app
        .path()
        .app_data_dir()
        .expect("failed to resolve app data dir")
        .join("sessions.db");
      std::fs::create_dir_all(&db_path).ok();
      let db =
        tauri::async_runtime::block_on(db::init_db(db_path.to_str().expect("non-UTF-8 path")))
          .expect("failed to initialise SurrealDB");
      app.manage(db);
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      db_save_session,
      db_list_sessions,
      db_get_session,
      db_delete_session,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
