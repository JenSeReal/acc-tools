use acc_tools_ui_components::TimedSplashScreen;
use leptos::prelude::*;
use leptos_meta::{Title, provide_meta_context};
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;

use crate::common::state::{ChampionshipState, EventState, SessionState, TelemetryState};
use crate::components::layout::MainLayout;
use crate::domain::{
  ChampionshipCreatePage, ChampionshipViewPage, EventListPage, EventViewPage, HomePage,
  SessionUploadPage, SessionViewPage, TelemetryLapsPage, TelemetryUploadPage, TelemetryViewPage,
};

#[component]
pub fn App() -> impl IntoView {
  provide_meta_context();
  provide_context(SessionState::new());
  provide_context(TelemetryState::new());
  provide_context(EventState::new());
  provide_context(ChampionshipState::new());

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
            <Route path=path!("/telemetry/laps") view=TelemetryLapsPage />
            <Route path=path!("/event") view=EventListPage />
            <Route path=path!("/event/view") view=EventViewPage />
            <Route path=path!("/championship/create") view=ChampionshipCreatePage />
            <Route path=path!("/championship/view") view=ChampionshipViewPage />
          </Routes>
        </Router>
      </MainLayout>
    </TimedSplashScreen>
  }
}
