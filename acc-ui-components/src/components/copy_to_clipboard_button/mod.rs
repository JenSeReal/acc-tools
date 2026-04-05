use leptos::prelude::*;

#[cfg(any(feature = "csr", feature = "hydrate"))]
use leptos::wasm_bindgen::JsCast;
#[cfg(any(feature = "csr", feature = "hydrate"))]
use leptos::wasm_bindgen::prelude::*;

#[component]
pub fn CopyToClipboardButton(
  text: Signal<String>,
  #[prop(default = "Copy".to_string())] label: String,
  #[prop(optional)] class: Option<String>,
) -> impl IntoView {
  let (copied, set_copied) = signal(false);
  let extra_class = class.unwrap_or_default();

  #[cfg(any(feature = "csr", feature = "hydrate"))]
  let on_click = {
    move |_| {
      let content = text.get();

      let window = leptos::web_sys::window().unwrap();
      let navigator = window.navigator();
      let clipboard = navigator.clipboard();

      let promise = clipboard.write_text(&content);
      drop(wasm_bindgen_futures::JsFuture::from(promise));

      set_copied.set(true);

      let set_copied_clone = set_copied;
      let timeout = Closure::wrap(Box::new(move || {
        set_copied_clone.set(false);
      }) as Box<dyn FnMut()>);

      window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
          timeout.as_ref().unchecked_ref(),
          2000,
        )
        .unwrap();
      timeout.forget();
    }
  };

  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let on_click = {
    // Suppress unused variable warning
    let _ = text;
    let _ = set_copied;
    move |_: leptos::ev::MouseEvent| {
      // SSR: Click handling will be hydrated on client
    }
  };

  let button_class = {
    let extra_class = extra_class.clone();
    move || {
      let base = "inline-flex items-center gap-2 rounded-md px-3 py-2 text-sm font-medium transition-colors focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-2";
      if copied.get() {
        format!(
          "{} bg-green-600 text-white hover:bg-green-700 {}",
          base, extra_class
        )
      } else {
        format!(
          "{} bg-secondary text-secondary-foreground hover:bg-secondary/80 {}",
          base, extra_class
        )
      }
    }
  };

  let button_text = {
    let label = label.clone();
    move || {
      if copied.get() {
        "Copied!".to_string()
      } else {
        label.clone()
      }
    }
  };

  view! {
    <button class=button_class on:click=on_click>
      <span class="text-base">{"\u{1F4CB}"}</span>
      <span>{button_text}</span>
    </button>
  }
}
