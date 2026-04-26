use acc_session_viewer_lib::loaded_session::LoadedSession;
use acc_tools_ui_components::FileUpload;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::common::state::SessionState;

#[component]
pub fn SessionUploadPage() -> impl IntoView {
  let state = expect_context::<SessionState>();
  let navigate = use_navigate();

  let on_session_loaded = move |loaded: LoadedSession| {
    state.session.set(Some(loaded));
    state.selected_car_model.set(None);
    state.error.set(None);
    navigate("/session/view", Default::default());
  };

  let error_write = state.error.write_only();

  view! {
    <main class="container mx-auto max-w-4xl px-4 py-8">
      <h1 class="text-3xl font-bold text-center mb-8">"ACC Session Viewer"</h1>

      {move || state.error.get().map(|e| view! {
        <div class="bg-destructive/10 border border-destructive text-destructive px-4 py-3 rounded-md mb-6">{e}</div>
      })}

      <div class="flex flex-col items-center gap-6">
        <p class="text-muted-foreground text-center">"Upload a session file (.json or .pdf) to view driver statistics"</p>
        <FileUpload on_session_loaded=on_session_loaded on_error=error_write />
      </div>

      <SavedSessions />
    </main>
  }
}

#[component]
fn SavedSessions() -> impl IntoView {
  #[cfg(any(feature = "csr", feature = "hydrate"))]
  {
    let state = expect_context::<SessionState>();
    use crate::common::db::SessionSummary;

    let sessions: RwSignal<Vec<SessionSummary>> = RwSignal::new(Vec::new());
    let load_error: RwSignal<Option<String>> = RwSignal::new(None);

    // Load saved sessions on mount.
    Effect::new(move || {
      leptos::task::spawn_local(async move {
        match crate::common::db::list_sessions().await {
          Ok(list) => sessions.set(list),
          Err(e) => load_error.set(Some(e)),
        }
      });
    });

    let refresh = move || {
      leptos::task::spawn_local(async move {
        match crate::common::db::list_sessions().await {
          Ok(list) => sessions.set(list),
          Err(e) => load_error.set(Some(e)),
        }
      });
    };

    let navigate_load = StoredValue::new(leptos_router::hooks::use_navigate());

    view! {
      <div class="mt-10">
        <h2 class="text-xl font-semibold mb-4">"Saved Sessions"</h2>

        {move || load_error.get().map(|e| view! {
          <div class="bg-destructive/10 border border-destructive text-destructive px-4 py-3 rounded-md mb-4">{e}</div>
        })}

        <Show
          when=move || !sessions.get().is_empty()
          fallback=|| view! {
            <p class="text-muted-foreground text-sm">"No saved sessions found."</p>
          }
        >
          <div class="rounded-md border divide-y">
            <For
              each=move || sessions.get()
              key=|s| s.id.clone()
              children=move |summary| {
                let id_load = summary.id.clone();
                let id_delete = summary.id.clone();
                let on_load = move |_| {
                  let id = id_load.clone();
                  let navigate_load = navigate_load.get_value();
                  leptos::task::spawn_local(async move {
                    match crate::common::db::get_session(&id).await {
                      Ok(Some(session)) => {
                        state.session.set(Some(session));
                        state.selected_car_model.set(None);
                        state.error.set(None);
                        navigate_load("/session/view", Default::default());
                      }
                      Ok(None) => {
                        state.error.set(Some("Session not found in database".to_string()));
                      }
                      Err(e) => {
                        state.error.set(Some(format!("Failed to load session: {e}")));
                      }
                    }
                  });
                };

                let on_delete = move |_| {
                  let id = id_delete.clone();
                  leptos::task::spawn_local(async move {
                    match crate::common::db::delete_session(&id).await {
                      Ok(()) => refresh(),
                      Err(e) => load_error.set(Some(format!("Failed to delete: {e}"))),
                    }
                  });
                };

                view! {
                  <div class="flex items-center justify-between px-4 py-3">
                    <div class="flex flex-col gap-0.5">
                      <span class="font-medium text-sm">{summary.track_name.clone()}</span>
                      <span class="text-xs text-muted-foreground">
                        {summary.session_type.clone()}
                        " · "
                        {summary.car_count}
                        " cars · "
                        {summary.file_name.clone()}
                      </span>
                    </div>
                    <div class="flex items-center gap-2">
                      <button
                        class="inline-flex items-center rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
                        on:click=on_load
                      >
                        "Load"
                      </button>
                      <button
                        class="inline-flex items-center rounded-md bg-destructive px-3 py-1.5 text-xs font-medium text-destructive-foreground shadow-sm transition-colors hover:bg-destructive/90"
                        on:click=on_delete
                      >
                        "Delete"
                      </button>
                    </div>
                  </div>
                }
              }
            />
          </div>
        </Show>
      </div>
    }.into_any()
  }

  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  {
    view! { <div /> }.into_any()
  }
}
