use leptos::prelude::*;
use tw_merge::tw_merge;

use super::header_wrapper::HeaderScrollContext;
use crate::components::interactive_logo::InteractiveLogo;

/// Header logo component with animated entrance after splash screen.
/// The logo overflows the title bar slightly for visual impact.
/// Animation only plays when transitioning from hidden to visible (after splash).
/// Logo scales down when header is collapsed (scrolled).
#[component]
pub fn HeaderLogo(
  #[prop(into)] logo_src: String,
  #[prop(into)] logo_visible: ReadSignal<bool>,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  // Check if we started visible (splash already played) - don't animate
  let started_visible = logo_visible.get_untracked();
  let (should_animate, set_should_animate) = signal(false);

  // Get scroll context to scale logo when collapsed
  let scroll_ctx = use_context::<HeaderScrollContext>();

  // Only set should_animate to true when logo becomes visible after starting hidden
  Effect::new(move |prev_visible: Option<bool>| {
    let current_visible = logo_visible.get();

    // If we started hidden and now becoming visible, trigger animation
    if !started_visible && current_visible && prev_visible == Some(false) {
      set_should_animate.set(true);
    }

    current_visible
  });

  let base_class = tw_merge!(
    "relative flex items-center shrink-0 z-50 transition-all duration-300",
    class
  );

  // Get whether header is collapsed
  let is_collapsed = move || scroll_ctx.map(|ctx| !ctx.is_at_top.get()).unwrap_or(false);

  // Logo size - different when collapsed, but always overflows
  let logo_size = Signal::derive(move || {
    if is_collapsed() {
      // Collapsed: smaller logo, still overflows
      "h-12 w-12 -my-2".to_string()
    } else {
      // Expanded: larger logo with more overflow
      "h-24 w-24 mt-2 -mb-10".to_string()
    }
  });

  view! {
    <div
      class=base_class
      class:opacity-0=move || !logo_visible.get()
      class:animate-fade-in-right=move || should_animate.get()
    >
      <a href="/" class="flex items-center">
        <InteractiveLogo logo_src=logo_src size=logo_size />
      </a>
    </div>
  }
}
