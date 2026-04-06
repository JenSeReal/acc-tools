use leptos::prelude::*;

use crate::components::ui::navigation_menu::{
  NavigationMenu, NavigationMenuLink, NavigationMenuList, navigation_menu_trigger_style,
};

/// A title bar component using NavigationMenu with logo on the left and title in the center.
/// The logo overflows the title bar slightly for visual impact.
#[component]
pub fn TitleBar(
  /// Path to the logo image (defaults to "/logo.svg")
  #[prop(default = "/logo.svg".to_string())]
  logo_src: String,
  /// The title text to display
  #[prop(default = "Apex Predators Sim Racing".to_string())]
  title: String,
) -> impl IntoView {
  view! {
    <header class="sticky top-0 z-40 w-full border-b border-border bg-card/95 backdrop-blur supports-[backdrop-filter]:bg-card/60">
      <div class="relative flex h-14 items-center px-4">
        // Title - absolutely centered on the page
        <div class="absolute inset-0 flex items-center justify-center pointer-events-none">
          <h1 class="text-lg font-bold tracking-wide text-primary sm:text-xl">
            {title}
          </h1>
        </div>
        // Logo on the left - overflows the title bar
        <div class="relative flex items-center z-50">
          <a href="/" class="flex items-center">
            <img
              src=logo_src
              alt="Logo"
              class="h-24 w-24 object-contain mt-2 -mb-10"
            />
          </a>
        </div>
      </div>
    </header>
  }
}

/// A title bar with navigation links using NavigationMenu.
#[component]
pub fn TitleBarWithNav(
  /// Path to the logo image (defaults to "/logo.svg")
  #[prop(default = "/logo.svg".to_string())]
  logo_src: String,
  /// The title text to display
  #[prop(default = "Apex Predators Sim Racing".to_string())]
  title: String,
  /// Navigation links as (label, href) tuples
  #[prop(default = vec![])]
  nav_links: Vec<(String, String)>,
) -> impl IntoView {
  view! {
    <header class="sticky top-0 z-40 w-full border-b border-border bg-card/95 backdrop-blur supports-[backdrop-filter]:bg-card/60">
      <div class="flex h-14 items-center px-4">
        // Logo on the left - overflows the title bar
        <div class="relative flex items-center shrink-0">
          <a href="/" class="flex items-center">
            <img
              src=logo_src
              alt="Logo"
              class="relative z-50 h-24 w-24 object-contain mt-2 -mb-10"
            />
          </a>
        </div>

        // Title
        <div class="flex items-center ml-4">
          <h1 class="text-lg font-bold tracking-wide text-primary sm:text-xl whitespace-nowrap">
            {title}
          </h1>
        </div>

        // Navigation menu
        <div class="flex flex-1 items-center justify-end">
          <NavigationMenu>
            <NavigationMenuList>
              {nav_links
                .into_iter()
                .map(|(label, href)| {
                  view! {
                    <NavigationMenuLink class=navigation_menu_trigger_style() href=href>
                      {label}
                    </NavigationMenuLink>
                  }
                })
                .collect_view()}
            </NavigationMenuList>
          </NavigationMenu>
        </div>
      </div>
    </header>
  }
}
