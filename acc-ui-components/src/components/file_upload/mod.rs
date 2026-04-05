use acc_session_reader_lib::loaded_session::LoadedSession;
use leptos::prelude::*;

#[cfg(any(feature = "csr", feature = "hydrate"))]
use acc_session_reader_lib::parser::{PdfParser, SessionParser};
#[cfg(any(feature = "csr", feature = "hydrate"))]
use acc_session_reader_lib::session::model::Session;
#[cfg(any(feature = "csr", feature = "hydrate"))]
use leptos::wasm_bindgen::JsCast;
#[cfg(any(feature = "csr", feature = "hydrate"))]
use leptos::wasm_bindgen::prelude::*;
#[cfg(any(feature = "csr", feature = "hydrate"))]
use leptos::web_sys::{Event, FileReader, HtmlInputElement};

#[component]
pub fn FileUpload<F>(
  on_session_loaded: F,
  on_error: WriteSignal<Option<String>>,
  #[prop(optional)] class: Option<String>,
) -> impl IntoView
where
  F: Fn(LoadedSession) + 'static + Clone,
{
  let wrapper_class = class
    .map(|c| format!("flex justify-center {}", c))
    .unwrap_or_else(|| "flex justify-center".to_string());

  #[cfg(any(feature = "csr", feature = "hydrate"))]
  let on_change = {
    let on_session_loaded = on_session_loaded.clone();
    move |ev: Event| {
      let on_session_loaded = on_session_loaded.clone();
      let input: HtmlInputElement = ev.target().unwrap().dyn_into().unwrap();

      if let Some(files) = input.files()
        && let Some(file) = files.get(0)
      {
        let file_name = file.name();
        let is_pdf = file_name.to_lowercase().ends_with(".pdf");

        let reader = FileReader::new().unwrap();
        let reader_clone = reader.clone();

        let onload = Closure::wrap(Box::new(move |_: Event| {
          let result = reader_clone.result().unwrap();
          let array_buffer = result.dyn_into::<js_sys::ArrayBuffer>().unwrap();
          let uint8_array = js_sys::Uint8Array::new(&array_buffer);
          let bytes = uint8_array.to_vec();

          if is_pdf {
            match PdfParser::parse(&bytes) {
              Ok(qualifying) => {
                on_error.set(None);
                on_session_loaded(LoadedSession::from_qualifying(&qualifying));
              }
              Err(e) => {
                on_error.set(Some(format!("Failed to parse PDF: {}", e)));
              }
            }
          } else {
            match Session::try_from(bytes.as_slice()) {
              Ok(session) => {
                on_error.set(None);
                on_session_loaded(LoadedSession::from_session(&session));
              }
              Err(e) => {
                on_error.set(Some(format!("Failed to parse session: {}", e)));
              }
            }
          }
        }) as Box<dyn FnMut(_)>);

        reader.set_onload(Some(onload.as_ref().unchecked_ref()));
        onload.forget();

        reader.read_as_array_buffer(&file).unwrap();
      }
    }
  };

  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let on_change = {
    // Suppress unused variable warnings for SSR
    let _ = on_session_loaded;
    let _ = on_error;
    move |_: leptos::ev::Event| {
      // SSR: Event handling will be hydrated on client
    }
  };

  view! {
    <div class=wrapper_class>
      <label class="cursor-pointer">
        <input
          type="file"
          accept=".json,.pdf"
          on:change=on_change
          class="sr-only"
        />
        <span class="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90 focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2">
          "Choose Session File"
        </span>
      </label>
    </div>
  }
}
