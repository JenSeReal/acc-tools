use leptos::prelude::*;
use tw_merge::tw_merge;

use super::header_wrapper::HeaderScrollContext;

/// Header title component displaying the site/app name.
/// Hides when header is collapsed (scrolled).
#[component]
pub fn HeaderTitle(
  #[prop(into)] title: String,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let scroll_ctx = use_context::<HeaderScrollContext>();
  let is_collapsed = move || scroll_ctx.map(|ctx| !ctx.is_at_top.get()).unwrap_or(false);

  let class = tw_merge!(
    "flex items-center shrink-0 transition-all duration-300",
    class
  );

  view! {
    <div
      class=class
      class:opacity-0=is_collapsed
      class:scale-0=is_collapsed
    >
      <h1 class="text-lg font-bold tracking-wide text-primary sm:text-xl whitespace-nowrap">
        {title}
      </h1>
    </div>
  }
}
