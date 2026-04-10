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
/// Once loaded, displays a "tap anywhere" prompt. On click, plays a sound and
/// zooms the logo out, then reveals the page content.
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
  /// Path to the sound file played during the exit animation (defaults to "/splash_scren.ogg")
  #[prop(default = "/splash_scren.ogg".to_string())]
  sound_src: String,
) -> impl IntoView {
  // Check if we should skip the splash (already played this session)
  #[cfg(any(feature = "csr", feature = "hydrate"))]
  let skip_splash = has_splash_played();
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let skip_splash = false;

  // Track animation state: false = showing logo, true = showing content
  let (animation_complete, set_animation_complete) = signal(skip_splash);
  // True once the user taps/clicks to trigger the exit animation
  let (clicked, set_clicked) = signal(false);

  // When the user clicks, play sound and wait for exit animation to finish
  #[cfg(any(feature = "csr", feature = "hydrate"))]
  {
    use leptos::wasm_bindgen::JsCast;
    use leptos::wasm_bindgen::closure::Closure;

    let sound_src_clone = sound_src.clone();
    Effect::new(move |_| {
      if skip_splash {
        return;
      }

      if clicked.get() && !animation_complete.get() {
        // Play sound at the start of the exit animation
        if let Ok(audio) = web_sys::HtmlAudioElement::new_with_src(&sound_src_clone) {
          let _ = audio.play();
        }

        // Wait for the exit animation duration (1000ms) plus a small buffer
        let timeout_callback = Closure::wrap(Box::new(move || {
          set_animation_complete.set(true);
          mark_splash_played();
        }) as Box<dyn Fn()>);

        if let Some(window) = leptos::web_sys::window() {
          let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            timeout_callback.as_ref().unchecked_ref(),
            1100,
          );
        }

        timeout_callback.forget();
      }
    });
  }

  // For SSR, suppress unused warnings
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  {
    let _ = is_loaded;
    let _ = set_animation_complete;
    let _ = sound_src;
    let _ = clicked;
    let _ = set_clicked;
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
    <div class="relative min-h-screen w-full bg-background">
      // Logo overlay with black background — clickable once loaded
      <div
        class="fixed inset-0 z-50 flex flex-col items-center justify-center bg-black transition-opacity duration-300"
        class:opacity-0=move || animation_complete.get()
        class:pointer-events-none=move || animation_complete.get()
        class:cursor-pointer=move || is_loaded.get() && !clicked.get()
        on:click=move |_| {
          if is_loaded.get() && !clicked.get() {
            set_clicked.set(true);
          }
        }
      >
        <img
          src=logo_src
          alt="Loading..."
          class="h-48 w-48 object-contain"
          class:animate-breathing=move || !clicked.get()
          class:animate-zoom-in=move || clicked.get()
        />
        // "Tap anywhere" prompt fades in when loaded, fades out on click
        <p
          class="absolute bottom-16 text-white/50 text-xs tracking-[0.3em] uppercase animate-pulse transition-opacity duration-700"
          class:opacity-0=move || !is_loaded.get() || clicked.get()
        >
          "Tap anywhere"
        </p>
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
  /// Path to the sound file played during the exit animation (defaults to "/splash_scren.ogg")
  #[prop(default = "/splash_scren.ogg".to_string())]
  sound_src: String,
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
    <SplashScreen is_loaded=is_loaded logo_src=logo_src sound_src=sound_src>
      {children()}
    </SplashScreen>
  }
}
