use acc_race_session_reader_lib::car::car_model::CarModel;
use acc_race_session_reader_lib::driver_stats::{
  DriverStatsCommon, filter_stats_by_car_model, stats_to_csv,
};
use acc_race_session_reader_lib::loaded_session::LoadedSession;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn DriverStatsTable(
  session: ReadSignal<Option<LoadedSession>>,
  selected_car_model: ReadSignal<Option<CarModel>>,
) -> impl IntoView {
  let (copy_status, set_copy_status) = signal(None::<String>);

  let stats = move || {
    session.get().map(|s| match selected_car_model.get() {
      Some(car_model) => filter_stats_by_car_model(s.stats.clone(), car_model),
      None => s.stats.clone(),
    })
  };

  let on_copy_csv = move |_| {
    if let Some(driver_stats) = stats() {
      let csv = stats_to_csv(&driver_stats);

      let window = web_sys::window().unwrap();
      let navigator = window.navigator();
      let clipboard = navigator.clipboard();

      let promise = clipboard.write_text(&csv);
      let _ = wasm_bindgen_futures::JsFuture::from(promise);

      set_copy_status.set(Some("Copied to clipboard!".to_string()));

      // Clear status after 2 seconds
      let set_copy_status_clone = set_copy_status;
      let timeout = Closure::wrap(Box::new(move || {
        set_copy_status_clone.set(None);
      }) as Box<dyn FnMut()>);

      window
        .set_timeout_with_callback_and_timeout_and_arguments_0(
          timeout.as_ref().unchecked_ref(),
          2000,
        )
        .unwrap();
      timeout.forget();
    }
  };

  view! {
      <div class="driver-stats">
          {move || {
              match stats() {
                  Some(driver_stats) if !driver_stats.is_empty() => {
                      view! {
                          <div class="stats-header">
                              <h3>"Driver Statistics"</h3>
                              <button class="copy-csv-btn" on:click=on_copy_csv>
                                  "Copy as CSV"
                              </button>
                              {move || copy_status.get().map(|msg| view! {
                                  <span class="copy-status">{msg}</span>
                              })}
                          </div>
                          <table class="stats-table">
                              <thead>
                                  <tr>
                                      <th>"#"</th>
                                      <th>"Driver"</th>
                                      <th>"Car"</th>
                                      <th>"Fastest"</th>
                                      <th>"Average"</th>
                                      <th>"Laps"</th>
                                      <th>"7 Valid"</th>
                                  </tr>
                              </thead>
                              <tbody>
                                  {driver_stats.iter().map(|driver| {
                                      let race_number = driver.race_number();
                                      let driver_name = driver.driver_name().to_string();
                                      let short_name = driver.short_name().map(|s| s.to_string());
                                      let show_short_name = short_name.as_ref().map(|s| !s.is_empty()).unwrap_or(false);
                                      let short_name_display = short_name.map(|s| format!(" ({})", s)).unwrap_or_default();
                                      let car_name = driver.car_model().name().to_string();
                                      let fastest = driver.fastest_lap_formatted();
                                      let average = driver.average_lap_formatted();
                                      let lap_count = format!("{}/{}", driver.valid_laps(), driver.total_laps());
                                      let seven_class = if driver.has_seven_consecutive_valid() {
                                          "seven-valid yes"
                                      } else {
                                          "seven-valid no"
                                      };
                                      let seven_text = if driver.has_seven_consecutive_valid() {
                                          "Yes"
                                      } else {
                                          "No"
                                      };

                                      view! {
                                          <tr>
                                              <td class="race-number">{race_number}</td>
                                              <td class="driver-name">
                                                  {driver_name}
                                                  <Show when=move || show_short_name>
                                                      <span class="short-name">{short_name_display.clone()}</span>
                                                  </Show>
                                              </td>
                                              <td class="car-name">{car_name}</td>
                                              <td class="laptime fastest">{fastest}</td>
                                              <td class="laptime average">{average}</td>
                                              <td class="lap-count">{lap_count}</td>
                                              <td class={seven_class}>{seven_text}</td>
                                          </tr>
                                      }
                                  }).collect_view()}
                              </tbody>
                          </table>
                      }.into_any()
                  }
                  Some(_) => view! { <p class="no-data">"No drivers found"</p> }.into_any(),
                  None => view! { <p class="no-selection">"Upload a session file to view statistics"</p> }.into_any()
              }
          }}
      </div>
  }
}
