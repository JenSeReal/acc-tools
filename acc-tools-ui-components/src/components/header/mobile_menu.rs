use icons::{Menu, X};
use leptos::prelude::*;
use tw_merge::tw_merge;

use crate::components::interactive_logo::InteractiveLogo;

/// Mobile menu trigger button with animated hamburger/close icon.
#[component]
pub fn MobileMenuTrigger(
  is_open: RwSignal<bool>,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let toggle_menu = move |_| {
    is_open.update(|open| *open = !*open);
  };

  let data_state = move || if is_open.get() { "active" } else { "inactive" };

  let class = tw_merge!("relative z-20 p-2.5 -m-2.5 cursor-pointer", class);

  view! {
    <button
      aria-label="Toggle Menu"
      class=class
      data-state=data_state
      on:click=toggle_menu
    >
      <Menu class="m-auto size-5 transition-all duration-200 in-data-[state=active]:rotate-180 in-data-[state=active]:scale-0 in-data-[state=active]:opacity-0" />
      <X class="absolute inset-0 m-auto size-5 opacity-0 scale-0 -rotate-180 transition-all duration-200 in-data-[state=active]:rotate-0 in-data-[state=active]:scale-100 in-data-[state=active]:opacity-100" />
    </button>
  }
}

/// Mobile navigation menu that slides down when open.
#[component]
pub fn MobileMenu(
  is_open: RwSignal<bool>,
  #[prop(default = vec![])] nav_links: Vec<(String, String)>,
  #[prop(default = "/logo.svg".to_string())] logo_src: String,
) -> impl IntoView {
  let display_class = move || {
    if is_open.get() { "flex" } else { "hidden" }
  };

  view! {
    <div class=move || format!(
      "flex-col gap-4 w-full lg:hidden border-t border-border mt-4 pt-4 pb-4 {}",
      display_class()
    )>
      // Navigation links
      <nav class="flex flex-col gap-2">
        {nav_links
          .into_iter()
          .map(|(label, href)| {
            view! {
              <a
                href=href
                class="px-4 py-2 text-sm font-medium rounded-md hover:bg-accent hover:text-accent-foreground transition-colors"
              >
                {label}
              </a>
            }
          })
          .collect_view()}
      </nav>

      // Logo at the bottom of mobile menu
      <div class="flex justify-center mt-4 pt-4 border-t border-border">
        <a href="/" class="flex items-center">
          <InteractiveLogo
            logo_src=logo_src
            size=Signal::derive(|| "h-16 w-16".to_string())
          />
        </a>
      </div>
    </div>
  }
}
