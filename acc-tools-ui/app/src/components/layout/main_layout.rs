use acc_tools_ui_components::TitleBarWithNav;
use leptos::prelude::*;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
  let nav_links = vec![
    ("Session".to_string(), "/session".to_string()),
    ("Telemetry".to_string(), "/telemetry".to_string()),
    ("Event".to_string(), "/event".to_string()),
    (
      "Championship".to_string(),
      "/championship/create".to_string(),
    ),
  ];

  view! {
    <div class="min-h-screen bg-background">
      <TitleBarWithNav nav_links=nav_links />
      <main>
        {children()}
      </main>
    </div>
  }
}
