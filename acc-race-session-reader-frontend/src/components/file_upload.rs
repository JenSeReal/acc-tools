use acc_race_session_reader_lib::loaded_session::LoadedSession;
use acc_race_session_reader_lib::parser::{PdfParser, SessionParser};
use acc_race_session_reader_lib::session::model::Session;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{Event, FileReader, HtmlInputElement};

#[component]
pub fn FileUpload<F>(on_session_loaded: F, on_error: WriteSignal<Option<String>>) -> impl IntoView
where
  F: Fn(LoadedSession) + 'static + Clone,
{
  let on_change = move |ev: Event| {
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
          // Parse as PDF qualifying results
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
          // Parse as JSON session
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
  };

  view! {
      <div class="file-upload">
          <label class="file-upload-label">
              <input
                  type="file"
                  accept=".json,.pdf"
                  on:change=on_change
                  class="file-input"
              />
              <span class="file-upload-button">"Choose Session File"</span>
          </label>
      </div>
  }
}
