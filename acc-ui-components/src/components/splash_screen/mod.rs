use leptos::prelude::*;

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

/// Mark splash as played in session storage
#[cfg(any(feature = "csr", feature = "hydrate"))]
fn mark_splash_played() {
  let Some(window) = web_sys::window() else {
    return;
  };
  let Ok(Some(storage)) = window.session_storage() else {
    return;
  };
  let _ = storage.set_item(SPLASH_PLAYED_KEY, "1");
}

/// A splash screen component that shows an animated logo while content loads.
/// The logo starts large and zooms towards the viewer when loading completes,
/// then fades out to reveal the page content.
/// Only plays once per browser session.
#[component]
pub fn SplashScreen(
  /// Signal that indicates when the page content has finished loading
  #[prop(into)]
  is_loaded: Signal<bool>,
  /// The page content to display after the animation
  children: Children,
  /// Path to the logo image (defaults to "/logo.svg")
  #[prop(default = "/logo.svg".to_string())]
  logo_src: String,
) -> impl IntoView {
  // Check if we should skip the splash (already played this session)
  #[cfg(any(feature = "csr", feature = "hydrate"))]
  let skip_splash = has_splash_played();
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let skip_splash = false;

  // Track animation state: false = showing logo, true = showing content
  let (animation_complete, set_animation_complete) = signal(skip_splash);

  // When loaded, wait for zoom animation to finish before showing content
  #[cfg(any(feature = "csr", feature = "hydrate"))]
  {
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::closure::Closure;

    Effect::new(move |_| {
      if skip_splash {
        return;
      }

      if is_loaded.get() && !animation_complete.get() {
        // Wait for the zoom animation duration (600ms) plus a small buffer
        let timeout_callback = Closure::wrap(Box::new(move || {
          set_animation_complete.set(true);
          mark_splash_played();
        }) as Box<dyn Fn()>);

        if let Some(window) = leptos::web_sys::window() {
          let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            timeout_callback.as_ref().unchecked_ref(),
            650,
          );
        }

        timeout_callback.forget();
      }
    });
  }

  // For SSR, just mark as complete if loaded
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  {
    let _ = is_loaded;
    let _ = set_animation_complete;
  }

  // If splash already played, just render children directly
  if skip_splash {
    return view! {
      <div class="min-h-screen w-full bg-background">
        {children()}
      </div>
    }
    .into_any();
  }

  view! {
    <div class="relative min-h-screen w-full overflow-hidden bg-background">
      // Logo overlay with black background
      <div
        class="fixed inset-0 z-50 flex items-center justify-center bg-black transition-opacity duration-300"
        class:opacity-0=move || animation_complete.get()
        class:pointer-events-none=move || animation_complete.get()
      >
        <img
          src=logo_src
          alt="Loading..."
          class="h-48 w-48 object-contain"
          class:animate-breathing=move || !is_loaded.get()
          class:animate-zoom-in=move || is_loaded.get()
        />
      </div>

      // Page content
      <div
        class="transition-opacity duration-500"
        class:opacity-0=move || !animation_complete.get()
        class:opacity-100=move || animation_complete.get()
      >
        {children()}
      </div>
    </div>
  }
  .into_any()
}

/// A splash screen that auto-triggers after a delay.
/// Useful when you don't have explicit load tracking.
/// Only plays once per browser session.
#[component]
pub fn TimedSplashScreen(
  /// Delay in milliseconds before the animation starts
  #[prop(default = 1000)]
  delay_ms: u32,
  /// The page content to display after the animation
  children: Children,
  /// Path to the logo image (defaults to "/logo.svg")
  #[prop(default = "/logo.svg".to_string())]
  logo_src: String,
) -> impl IntoView {
  // Check if we should skip the splash
  #[cfg(any(feature = "csr", feature = "hydrate"))]
  let skip_splash = has_splash_played();
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let skip_splash = false;

  let (is_loaded, set_is_loaded) = signal(skip_splash);

  #[cfg(any(feature = "csr", feature = "hydrate"))]
  {
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::closure::Closure;

    Effect::new(move |_| {
      if skip_splash {
        return;
      }

      let timeout_callback = Closure::wrap(Box::new(move || {
        set_is_loaded.set(true);
      }) as Box<dyn Fn()>);

      if let Some(window) = leptos::web_sys::window() {
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
          timeout_callback.as_ref().unchecked_ref(),
          delay_ms as i32,
        );
      }

      timeout_callback.forget();
    });
  }

  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  {
    let _ = delay_ms;
    let _ = set_is_loaded;
  }

  view! {
    <SplashScreen is_loaded=is_loaded logo_src=logo_src>
      {children()}
    </SplashScreen>
  }
}
