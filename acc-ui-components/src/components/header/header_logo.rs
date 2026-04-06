use leptos::prelude::*;
use tw_merge::tw_merge;

use crate::components::interactive_logo::InteractiveLogo;

/// Header logo component with animated entrance after splash screen.
/// The logo overflows the title bar slightly for visual impact.
#[component]
pub fn HeaderLogo(
  #[prop(into)] logo_src: String,
  #[prop(into)] logo_visible: ReadSignal<bool>,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let base_class = tw_merge!("relative flex items-center shrink-0 z-50", class);

  view! {
    <div
      class=base_class
      class:opacity-0=move || !logo_visible.get()
      class:animate-fade-in-right=move || logo_visible.get()
    >
      <a href="/" class="flex items-center">
        <InteractiveLogo
          logo_src=logo_src
          size="h-24 w-24 mt-2 -mb-10".to_string()
        />
      </a>
    </div>
  }
}
