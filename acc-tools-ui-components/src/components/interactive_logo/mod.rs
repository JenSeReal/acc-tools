use leptos::prelude::*;

/// An interactive logo component with animated eye.
/// The eye animation (blink + subtle movement) is handled by CSS in the SVG itself.
#[component]
pub fn InteractiveLogo(
  /// Path to the logo SVG (defaults to "/logo.svg")
  #[prop(default = "/logo.svg".to_string())]
  logo_src: String,
  /// CSS classes for the container
  #[prop(default = "".to_string())]
  class: String,
  /// Size classes for the logo (e.g., "h-24 w-24") - can be reactive
  #[prop(into)]
  size: Signal<String>,
) -> impl IntoView {
  view! {
    <img
      src=logo_src
      alt="Logo"
      class=move || format!("{} {} object-contain transition-all duration-300", size.get(), class)
    />
  }
}
