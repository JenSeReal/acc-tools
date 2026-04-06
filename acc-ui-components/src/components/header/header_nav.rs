use leptos::prelude::*;
use tw_merge::tw_merge;

use crate::components::ui::navigation_menu::{
  NavigationMenu, NavigationMenuLink, NavigationMenuList, navigation_menu_trigger_style,
};

/// Header navigation component containing navigation links.
#[component]
pub fn HeaderNav(
  #[prop(default = vec![])] nav_links: Vec<(String, String)>,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let class = tw_merge!("flex flex-1 items-center justify-center", class);

  view! {
    <div class=class>
      <NavigationMenu>
        <NavigationMenuList>
          {nav_links
            .into_iter()
            .map(|(label, href)| {
              view! {
                <HeaderNavLink href=href>{label}</HeaderNavLink>
              }
            })
            .collect_view()}
        </NavigationMenuList>
      </NavigationMenu>
    </div>
  }
}

/// A single navigation link in the header.
#[component]
pub fn HeaderNavLink(
  children: Children,
  #[prop(into)] href: String,
  #[prop(optional, into)] class: String,
) -> impl IntoView {
  let class = tw_merge!(navigation_menu_trigger_style(), class);

  view! {
    <NavigationMenuLink class=class href=href>
      {children()}
    </NavigationMenuLink>
  }
}
