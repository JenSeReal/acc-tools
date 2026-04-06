use leptos::prelude::*;
use tw_merge::tw_merge;

/// Header title component displaying the site/app name.
#[component]
pub fn HeaderTitle(
  #[prop(into)] title: String,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let class = tw_merge!("flex items-center shrink-0", class);

  view! {
    <div class=class>
      <h1 class="text-lg font-bold tracking-wide text-primary sm:text-xl whitespace-nowrap">
        {title}
      </h1>
    </div>
  }
}
