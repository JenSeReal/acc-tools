use leptos::prelude::*;
use tw_merge::tw_merge;

/// Root header wrapper component with scroll collapse behavior.
/// Hides when scrolling down, shows when scrolling up.
#[component]
pub fn Header(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
  let (is_visible, set_is_visible) = signal(true);
  let (is_at_top, set_is_at_top) = signal(true);

  #[cfg(any(feature = "csr", feature = "hydrate"))]
  {
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::closure::Closure;
    use std::cell::RefCell;
    use std::rc::Rc;

    let last_scroll_y = Rc::new(RefCell::new(0.0));

    Effect::new(move |_| {
      let last_scroll = last_scroll_y.clone();

      let scroll_handler = Closure::wrap(Box::new(move || {
        let Some(window) = web_sys::window() else {
          return;
        };

        let current_scroll_y = window.scroll_y().unwrap_or(0.0);
        let last_y = *last_scroll.borrow();

        // Update at-top state
        set_is_at_top.set(current_scroll_y < 10.0);

        // Only hide/show after scrolling past threshold
        if current_scroll_y > 60.0 {
          if current_scroll_y > last_y + 5.0 {
            // Scrolling down - hide header
            set_is_visible.set(false);
          } else if current_scroll_y < last_y - 5.0 {
            // Scrolling up - show header
            set_is_visible.set(true);
          }
        } else {
          // Near top - always show
          set_is_visible.set(true);
        }

        *last_scroll.borrow_mut() = current_scroll_y;
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
    let _ = set_is_visible;
    let _ = set_is_at_top;
  }

  let base_class = tw_merge!(
    "sticky top-0 z-40 w-full border-b border-border bg-card/95 backdrop-blur supports-[backdrop-filter]:bg-card/60 transition-transform duration-300",
    class
  );

  view! {
    <header
      class=base_class
      class:translate-y-0=move || is_visible.get()
      class:-translate-y-full=move || !is_visible.get()
      class:shadow-md=move || !is_at_top.get()
    >
      <div class="relative flex flex-wrap items-center justify-between h-14 px-4 lg:px-6">
        {children()}
      </div>
    </header>
  }
}
