use acc_tools_ui_components::TimedSplashScreen;
use leptos::prelude::*;
use leptos_meta::{Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::layouts::MainLayout;
use crate::pages::{
  HomePage, SessionUploadPage, SessionViewPage, TelemetryUploadPage, TelemetryViewPage,
};
use crate::state::{SessionState, TelemetryState};

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_context(SessionState::new());
  provide_context(TelemetryState::new());

  view! {
    <Title text="Apex Predators Sim Racing" />
    <TimedSplashScreen delay_ms=800>
      <MainLayout>
        <Router>
          <Routes fallback=|| view! { <p>"Page not found"</p> }>
            <Route path=path!("/") view=HomePage />
            <Route path=path!("/session") view=SessionUploadPage />
            <Route path=path!("/session/view") view=SessionViewPage />
            <Route path=path!("/telemetry") view=TelemetryUploadPage />
            <Route path=path!("/telemetry/view") view=TelemetryViewPage />
          </Routes>
        </Router>
      </MainLayout>
    </TimedSplashScreen>
  }
}
