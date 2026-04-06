use leptos::prelude::*;
use tw_merge::tw_merge;

use crate::components::interactive_logo::InteractiveLogo;

/// Header logo component with animated entrance after splash screen.
/// The logo overflows the title bar slightly for visual impact.
/// Animation only plays when transitioning from hidden to visible (after splash).
#[component]
pub fn HeaderLogo(
  #[prop(into)] logo_src: String,
  #[prop(into)] logo_visible: ReadSignal<bool>,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  // Check if we started visible (splash already played) - don't animate
  let started_visible = logo_visible.get_untracked();
  let (should_animate, set_should_animate) = signal(false);

  // Only set should_animate to true when logo becomes visible after starting hidden
  Effect::new(move |prev_visible: Option<bool>| {
    let current_visible = logo_visible.get();

    // If we started hidden and now becoming visible, trigger animation
    if !started_visible && current_visible && prev_visible == Some(false) {
      set_should_animate.set(true);
    }

    current_visible
  });

  let base_class = tw_merge!("relative flex items-center shrink-0 z-50", class);

  view! {
    <div
      class=base_class
      class:opacity-0=move || !logo_visible.get()
      class:animate-fade-in-right=move || should_animate.get()
    >
      <a href="/" class="flex items-center">
        <InteractiveLogo logo_src=logo_src size="h-24 w-24 mt-2 -mb-10".to_string() />
      </a>
    </div>
  }
}
