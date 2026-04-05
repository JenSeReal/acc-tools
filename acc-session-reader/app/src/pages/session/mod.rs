use acc_ui_components::{CarSelector, DriverStatsTable, SessionInfo};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::state::SessionState;

#[component]
pub fn SessionPage() -> impl IntoView {
  let state = expect_context::<SessionState>();

  let session_read = state.session.read_only();
  let selected_car_model_read = state.selected_car_model.read_only();
  let selected_car_model_write = state.selected_car_model.write_only();

  let on_new_session = move |_| {
    state.session.set(None);
    state.selected_car_model.set(None);
    state.error.set(None);
    let navigate = use_navigate();
    navigate("/", Default::default());
  };

  Effect::new(move || {
    if state.session.get().is_none() {
      let navigate = use_navigate();
      navigate("/", Default::default());
    }
  });

  view! {
    <main class="container mx-auto max-w-6xl px-4 py-8 space-y-6">
      <h1 class="text-3xl font-bold">"ACC Session Viewer"</h1>

      <Show when=move || state.session.get().is_some()>
        <div class="flex justify-end">
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
