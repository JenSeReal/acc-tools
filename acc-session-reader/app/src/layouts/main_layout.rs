use acc_ui_components::TitleBar;
use leptos::prelude::*;

/// Main layout with title bar and content area.
#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
  view! {
    <div class="min-h-screen bg-background">
      <TitleBar />
      <main>
        {children()}
      </main>
    </div>
  }
}
