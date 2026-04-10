use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::state::TelemetryState;

#[component]
pub fn UploadPage() -> impl IntoView {
  let state = expect_context::<TelemetryState>();
  let navigate = use_navigate();

  // Pending bytes held between the two file inputs
  let ld_bytes: RwSignal<Option<Vec<u8>>> = RwSignal::new(None);
  let ldx_bytes: RwSignal<Option<Vec<u8>>> = RwSignal::new(None);

  // ── .ld file reader ──────────────────────────────────────────────────────
  #[cfg(any(feature = "csr", feature = "hydrate"))]
  let on_ld_change = {
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::web_sys::{Event, FileReader, HtmlInputElement};

    move |ev: Event| {
      let input: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
      if let Some(files) = input.files()
        && let Some(file) = files.get(0)
      {
        let reader = FileReader::new().unwrap();
        let reader_clone = reader.clone();
        let onload = Closure::wrap(Box::new(move |_: Event| {
          let result = reader_clone.result().unwrap();
          let buf = result
            .dyn_into::<js_sys::ArrayBuffer>()
            .unwrap();
          let bytes = js_sys::Uint8Array::new(&buf).to_vec();
          ld_bytes.set(Some(bytes));
        }) as Box<dyn FnMut(_)>);
        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
        reader.read_as_array_buffer(&file).unwrap();
      }
    }
  };

  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let on_ld_change = move |_: leptos::ev::Event| {};

  // ── .ldx file reader ─────────────────────────────────────────────────────
  #[cfg(any(feature = "csr", feature = "hydrate"))]
  let on_ldx_change = {
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::web_sys::{Event, FileReader, HtmlInputElement};

    move |ev: Event| {
      let input: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();
      if let Some(files) = input.files()
        && let Some(file) = files.get(0)
      {
        let reader = FileReader::new().unwrap();
        let reader_clone = reader.clone();
        let onload = Closure::wrap(Box::new(move |_: Event| {
          let result = reader_clone.result().unwrap();
          let buf = result
            .dyn_into::<js_sys::ArrayBuffer>()
            .unwrap();
          let bytes = js_sys::Uint8Array::new(&buf).to_vec();
          ldx_bytes.set(Some(bytes));
        }) as Box<dyn FnMut(_)>);
        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();
        reader.read_as_array_buffer(&file).unwrap();
      }
    }
  };

  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let on_ldx_change = move |_: leptos::ev::Event| {};

  // ── Load button ──────────────────────────────────────────────────────────
  let navigate_clone = navigate.clone();
  let on_load = move |_| {
    let Some(ld) = ld_bytes.get() else {
      state
        .error
        .set(Some("Please select a .ld telemetry file.".to_string()));
      return;
    };

    match motec_viewer_lib::parse_ld(&ld) {
      Ok(session) => {
        state.session.set(Some(session));

        // Parse optional .ldx lap markers
        let laps = ldx_bytes
          .get()
          .as_deref()
          .and_then(|b| motec_viewer_lib::parse_ldx(b).ok())
          .unwrap_or_default();
        state.laps.set(laps);
        state.error.set(None);
        navigate_clone("/telemetry", Default::default());
      }
      Err(e) => {
        state
          .error
          .set(Some(format!("Failed to parse .ld file: {e}")));
      }
    }
  };

  let ld_ready = move || ld_bytes.get().is_some();

  view! {
    <main class="container mx-auto max-w-2xl px-4 py-8">
      <h1 class="text-3xl font-bold text-center mb-8">"MoTeC Telemetry Viewer"</h1>

      {move || state.error.get().map(|e| view! {
        <div class="bg-destructive/10 border border-destructive text-destructive px-4 py-3 rounded-md mb-6">{e}</div>
      })}

      <div class="rounded-lg border bg-card p-6 shadow-sm space-y-6">
        <div class="space-y-2">
          <label class="text-sm font-medium">"Telemetry file (.ld / .ldd)"</label>
          <label class="flex items-center gap-3 cursor-pointer">
            <input
              type="file"
              accept=".ld,.ldd"
              on:change=on_ld_change
              class="sr-only"
            />
            <span class="inline-flex items-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground">
              "Choose .ld file"
            </span>
            <span class="text-sm text-muted-foreground">
              {move || if ld_bytes.get().is_some() { "File loaded" } else { "No file selected" }}
            </span>
          </label>
        </div>

        <div class="space-y-2">
          <label class="text-sm font-medium">
            "Lap markers file (.ldx) "
            <span class="text-muted-foreground font-normal">"— optional"</span>
          </label>
          <label class="flex items-center gap-3 cursor-pointer">
            <input
              type="file"
              accept=".ldx"
              on:change=on_ldx_change
              class="sr-only"
            />
            <span class="inline-flex items-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm shadow-sm transition-colors hover:bg-accent hover:text-accent-foreground">
              "Choose .ldx file"
            </span>
            <span class="text-sm text-muted-foreground">
              {move || if ldx_bytes.get().is_some() { "File loaded" } else { "No file selected" }}
            </span>
          </label>
        </div>

        <button
          on:click=on_load
          disabled=move || !ld_ready()
          class="w-full inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 disabled:opacity-50 disabled:cursor-not-allowed"
        >
          "Load Telemetry"
        </button>
      </div>
    </main>
  }
}
