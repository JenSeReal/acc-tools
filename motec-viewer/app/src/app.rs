use acc_ui_components::TimedSplashScreen;
use leptos::prelude::*;
use leptos_meta::{Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::layouts::MainLayout;
use crate::pages::{TelemetryPage, UploadPage};
use crate::state::TelemetryState;

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  let state = TelemetryState::new();
  provide_context(state);

  view! {
    <Title text="MoTeC Viewer" />
    <TimedSplashScreen delay_ms=800>
      <MainLayout>
        <Router>
          <Routes fallback=|| view! { <p>"Page not found"</p> }>
            <Route path=path!("/") view=UploadPage />
            <Route path=path!("/telemetry") view=TelemetryPage />
          </Routes>
        </Router>
      </MainLayout>
    </TimedSplashScreen>
  }
}
