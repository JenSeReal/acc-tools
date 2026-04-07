use leptos::prelude::*;

/// Context to share header scroll state with child components
#[derive(Clone, Copy)]
pub struct HeaderScrollContext {
  pub is_at_top: ReadSignal<bool>,
}

/// Root header wrapper component with scroll collapse behavior.
/// Collapses into a floating "island" header when scrolled.
#[component]
pub fn Header(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
  let (is_at_top, set_is_at_top) = signal(true);

  // Provide context so child components can react to scroll state
  provide_context(HeaderScrollContext { is_at_top });

  #[cfg(any(feature = "csr", feature = "hydrate"))]
  {
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::closure::Closure;

    Effect::new(move |_| {
      let scroll_handler = Closure::wrap(Box::new(move || {
        let Some(window) = web_sys::window() else {
          return;
        };

        let current_scroll_y = window.scroll_y().unwrap_or(0.0);

        // Update at-top state with small threshold to prevent flickering
        set_is_at_top.set(current_scroll_y < 10.0);
      }) as Box<dyn Fn()>);

      if let Some(window) = web_sys::window() {
        let _ = window
          .add_event_listener_with_callback("scroll", scroll_handler.as_ref().unchecked_ref());
      }

      scroll_handler.forget();
    });
  }

  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  {
    let _ = set_is_at_top;
  }

  // Outer container - always sticky at top, with padding when scrolled
  let outer_class = move || {
    let base = format!(
      "sticky top-0 z-40 w-full transition-all duration-300 {}",
      class
    );
    if is_at_top.get() {
      base
    } else {
      format!("{} pt-4 pb-2", base)
    }
  };

  // Island wrapper - becomes floating island when scrolled
  let island_class = move || {
    if is_at_top.get() {
      // At top: full width, no styling
      "transition-all duration-300".to_string()
    } else {
      // Scrolled: floating island, at least 50% width, centered
      "transition-all duration-300 mx-auto min-w-[50%] max-w-[70%] rounded-full bg-card/95 backdrop-blur border border-border shadow-lg"
        .to_string()
    }
  };

  // Inner content area
  let inner_class = move || {
    let base = "relative flex items-center justify-between transition-all duration-300";
    if is_at_top.get() {
      format!("{} h-14 px-4 lg:px-6", base)
    } else {
      format!("{} h-8 px-3", base)
    }
  };

  view! {
    <header class=outer_class>
      <div class=island_class>
        <div class=inner_class>
          {children()}
        </div>
      </div>
    </header>
  }
}
