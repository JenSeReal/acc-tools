use acc_tools_ui_components::{CarSelector, DriverStatsTable, SessionInfo};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::common::state::SessionState;

#[component]
pub fn SessionViewPage() -> impl IntoView {
  let state = expect_context::<SessionState>();

  let session_read = state.session.read_only();
  let selected_car_model_read = state.selected_car_model.read_only();
  let selected_car_model_write = state.selected_car_model.write_only();

  // None = idle, Some(msg) = result message (success or error)
  let save_status: RwSignal<Option<String>> = RwSignal::new(None);

  Effect::new(move || {
    if state.session.get().is_none() {
      let navigate = use_navigate();
      navigate("/session", Default::default());
    }
  });

  let on_new_session = move |_| {
    state.session.set(None);
    state.selected_car_model.set(None);
    state.error.set(None);
    let navigate = use_navigate();
    navigate("/session", Default::default());
  };

  let on_save = move |_| {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
      let session = state.session.get();
      if let Some(session) = session {
        save_status.set(Some("Saving...".to_string()));
        leptos::task::spawn_local(async move {
          match crate::common::db::save_session(&session, "session").await {
            Ok(_id) => save_status.set(Some("Saved!".to_string())),
            Err(e) => save_status.set(Some(format!("Error: {e}"))),
          }
        });
      }
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    {
      save_status.set(Some("Save not available in SSR mode".to_string()));
    }
  };

  view! {
    <main class="container mx-auto max-w-6xl px-4 py-8 space-y-6">
      <Show when=move || state.session.get().is_some()>
        <div class="flex items-center justify-end gap-3">
          {move || save_status.get().map(|msg| {
            let is_error = msg.starts_with("Error:");
            let cls = if is_error {
              "text-sm text-destructive"
            } else {
              "text-sm text-muted-foreground"
            };
            view! { <span class=cls>{msg}</span> }
          })}

          <button
            class="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
            on:click=on_save
          >
            "Save to Database"
          </button>

          <button
            class="inline-flex items-center gap-2 rounded-md bg-secondary px-4 py-2 text-sm font-medium text-secondary-foreground shadow-sm transition-colors hover:bg-secondary/80"
            on:click=on_new_session
          >
            "Upload New Session"
          </button>
        </div>

        <SessionInfo session=session_read />
        <CarSelector session=session_read selected_car_model=selected_car_model_write />
        <DriverStatsTable session=session_read selected_car_model=selected_car_model_read />
      </Show>
    </main>
  }
}
