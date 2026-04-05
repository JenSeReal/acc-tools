use acc_session_reader_lib::loaded_session::LoadedSession;
use acc_ui_components::FileUpload;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::state::SessionState;

#[component]
pub fn UploadPage() -> impl IntoView {
  let state = expect_context::<SessionState>();
  let navigate = use_navigate();

  let on_session_loaded = move |loaded: LoadedSession| {
    state.session.set(Some(loaded));
    state.selected_car_model.set(None);
    state.error.set(None);
    navigate("/session", Default::default());
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
    </main>
  }
}
