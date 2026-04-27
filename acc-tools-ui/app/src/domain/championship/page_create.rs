use acc_session_viewer_lib::event::Championship;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::common::state::ChampionshipState;

#[component]
pub fn ChampionshipCreatePage() -> impl IntoView {
  let cs = expect_context::<ChampionshipState>();
  let navigate_sv = StoredValue::new(use_navigate());
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let _ = navigate_sv;

  let title: RwSignal<String> = RwSignal::new(String::new());
  let error: RwSignal<Option<String>> = RwSignal::new(None);
  let saving: RwSignal<bool> = RwSignal::new(false);

  let on_submit = move |_| {
    let t = title.get();
    if t.trim().is_empty() {
      error.set(Some("Title is required.".to_string()));
      return;
    }
    let championship = Championship {
      title: t,
      events: vec![],
    };

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
      saving.set(true);
      leptos::task::spawn_local(async move {
        match crate::common::db::save_championship(&championship).await {
          Ok(id) => {
            cs.championship.set(Some((id, championship)));
            navigate_sv.get_value()("/championship/view", Default::default());
          }
          Err(e) => {
            error.set(Some(format!("Failed to save: {e}")));
            saving.set(false);
          }
        }
      });
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    let _ = (championship, cs, saving);
  };

  view! {
    <main class="container mx-auto max-w-lg px-4 py-8">
      <h1 class="text-3xl font-bold text-center mb-8">"New Championship"</h1>

      {move || error.get().map(|e| view! {
        <div class="bg-destructive/10 border border-destructive text-destructive px-4 py-3 rounded-md mb-6 text-sm">{e}</div>
      })}

      <div class="rounded-lg border bg-card p-6 shadow-sm space-y-6">

        <div class="space-y-1">
          <p class="text-sm font-medium">"Title"</p>
          <input
            type="text"
            placeholder="e.g. GT World Challenge 2026"
            prop:value=move || title.get()
            on:input=move |ev| title.set(event_target_value(&ev))
            class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
          />
        </div>

        <button
          on:click=on_submit
          disabled=move || saving.get()
          class="w-full inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 disabled:opacity-50"
        >
          {move || if saving.get() { "Creating…" } else { "Create Championship" }}
        </button>
      </div>
    </main>
  }
}
