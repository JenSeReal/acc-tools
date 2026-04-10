use acc_ui_components::TitleBar;
use leptos::prelude::*;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
  view! {
    <div class="min-h-screen bg-background">
      <TitleBar title="MoTeC Telemetry Viewer".to_string() />
      <main>
        {children()}
      </main>
    </div>
  }
}
