mod header_logo;
mod header_nav;
mod header_title;
mod header_wrapper;
mod mobile_menu;

pub use header_logo::HeaderLogo;
pub use header_nav::{HeaderNav, HeaderNavLink};
pub use header_title::HeaderTitle;
pub use header_wrapper::Header;
pub use mobile_menu::{MobileMenu, MobileMenuTrigger};

use leptos::prelude::*;

/* ========================================================== */
/*                    SPLASH DETECTION                        */
/* ========================================================== */

#[cfg(any(feature = "csr", feature = "hydrate"))]
const SPLASH_PLAYED_KEY: &str = "acc_splash_played";

/// Check if splash has already played this session
#[cfg(any(feature = "csr", feature = "hydrate"))]
fn has_splash_played() -> bool {
  let Some(window) = web_sys::window() else {
    return false;
  };
  let Ok(Some(storage)) = window.session_storage() else {
    return false;
  };
  storage.get_item(SPLASH_PLAYED_KEY).ok().flatten().is_some()
}

/// Hook to track logo visibility after splash screen
pub fn use_logo_visibility() -> ReadSignal<bool> {
  #[cfg(any(feature = "csr", feature = "hydrate"))]
  let initial_visible = has_splash_played();
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let initial_visible = true;

  let (logo_visible, set_logo_visible) = signal(initial_visible);

  #[cfg(any(feature = "csr", feature = "hydrate"))]
  {
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::closure::Closure;
    use std::cell::RefCell;
    use std::rc::Rc;

    Effect::new(move |_| {
      if logo_visible.get() {
        return;
      }

      // Poll every 100ms until splash has played
      let interval_id = Rc::new(RefCell::new(None::<i32>));
      let interval_id_clone = interval_id.clone();

      let check_splash = Closure::wrap(Box::new(move || {
        if has_splash_played() {
          set_logo_visible.set(true);
          // Clear the interval
          if let Some(id) = *interval_id_clone.borrow() {
            if let Some(window) = web_sys::window() {
              window.clear_interval_with_handle(id);
            }
          }
        }
      }) as Box<dyn Fn()>);

      if let Some(window) = web_sys::window() {
        let id = window
          .set_interval_with_callback_and_timeout_and_arguments_0(
            check_splash.as_ref().unchecked_ref(),
            100,
          )
          .ok();
        *interval_id.borrow_mut() = id;
      }

      check_splash.forget();
    });
  }

  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  {
    let _ = set_logo_visible;
  }

  logo_visible
}

/* ========================================================== */
/*                    TITLEBAR (SIMPLE)                       */
/* ========================================================== */

/// A simple title bar with title centered and logo on the right.
/// The logo appears with an animation after the splash screen finishes.
#[component]
pub fn TitleBar(
  #[prop(default = "/logo.svg".to_string())] logo_src: String,
  #[prop(default = "Apex Predators Sim Racing".to_string())] title: String,
) -> impl IntoView {
  let logo_visible = use_logo_visibility();

  view! {
    <Header>
      // Title - absolutely centered
      <HeaderTitle title=title class="absolute inset-0 flex items-center justify-center pointer-events-none".to_string() />
      // Spacer
      <div class="flex-1"></div>
      // Logo on the right
      <HeaderLogo logo_src=logo_src logo_visible=logo_visible />
    </Header>
  }
}

/* ========================================================== */
/*                 TITLEBAR WITH NAV                          */
/* ========================================================== */

/// A title bar with navigation links and centered title.
/// Logo appears on the right after splash screen finishes.
#[component]
pub fn TitleBarWithNav(
  #[prop(default = "/logo.svg".to_string())] logo_src: String,
  #[prop(default = "Apex Predators Sim Racing".to_string())] title: String,
  #[prop(default = vec![])] nav_links: Vec<(String, String)>,
) -> impl IntoView {
  let logo_visible = use_logo_visibility();
  let is_mobile_menu_open = RwSignal::new(false);

  view! {
    <Header>
      // Title - absolutely centered
      <HeaderTitle title=title class="absolute inset-0 flex items-center justify-center pointer-events-none".to_string() />

      // Left: Mobile menu trigger
      <div class="flex items-center gap-4 z-10">
        <MobileMenuTrigger is_open=is_mobile_menu_open class="lg:hidden".to_string() />
        // Navigation (hidden on mobile)
        <HeaderNav nav_links=nav_links.clone() class="hidden lg:flex".to_string() />
      </div>

      // Right: Logo
      <HeaderLogo logo_src=logo_src.clone() logo_visible=logo_visible class="z-10".to_string() />

      // Mobile menu (shown when open)
      <MobileMenu is_open=is_mobile_menu_open nav_links=nav_links logo_src=logo_src />
    </Header>
  }
}
