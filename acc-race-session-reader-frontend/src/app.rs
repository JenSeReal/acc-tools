use acc_race_session_reader_lib::car::car_model::CarModel;
use acc_race_session_reader_lib::session::model::Session;
use leptos::prelude::*;

use crate::components::{CarSelector, DriverStats, FileUpload};

#[component]
pub fn App() -> impl IntoView {
  let (session, set_session) = signal(None::<Session>);
  let (selected_car_model, set_selected_car_model) = signal(None::<CarModel>);
  let (error, set_error) = signal(None::<String>);

  let on_session_loaded = move |new_session: Session| {
    set_session.set(Some(new_session));
    set_selected_car_model.set(None);
  };

  view! {
      <main class="container">
          <h1>"ACC Session Viewer"</h1>

          {move || error.get().map(|e| view! {
              <div class="error-banner">{e}</div>
          })}

          <FileUpload on_session_loaded=on_session_loaded on_error=set_error />

          {move || session.get().map(|s| {
              let session_type = format!("{:?}", s.session_type);
              let track = format!("{:?}", s.track_name);
              let car_count = s.session_result.leader_board_lines.len();

              view! {
                  <div class="session-info">
                      <h2>"Session Info"</h2>
                      <p><strong>"Type: "</strong>{session_type}</p>
                      <p><strong>"Track: "</strong>{track}</p>
                      <p><strong>"Cars: "</strong>{car_count}</p>
                  </div>
              }
          })}

          <Show when=move || session.get().is_some()>
              <CarSelector session=session selected_car_model=set_selected_car_model />
              <DriverStats session=session selected_car_model=selected_car_model />
          </Show>
      </main>
  }
}
