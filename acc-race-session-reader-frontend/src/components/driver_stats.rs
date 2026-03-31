use acc_race_session_reader_lib::car::car_model::CarModel;
use acc_race_session_reader_lib::driver_stats::{
  calculate_driver_stats, filter_stats_by_car_model,
};
use acc_race_session_reader_lib::session::model::Session;
use leptos::prelude::*;

#[component]
pub fn DriverStats(
  session: ReadSignal<Option<Session>>,
  selected_car_model: ReadSignal<Option<CarModel>>,
) -> impl IntoView {
  let stats = move || {
    session.get().map(|s| {
      let all_stats = calculate_driver_stats(&s);
      match selected_car_model.get() {
        Some(car_model) => filter_stats_by_car_model(all_stats, car_model),
        None => all_stats,
      }
    })
  };

  view! {
      <div class="driver-stats">
          {move || {
              match stats() {
                  Some(driver_stats) if !driver_stats.is_empty() => {
                      view! {
                          <h3>"Driver Statistics"</h3>
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
                                      let race_number = driver.race_number;
                                      let driver_name = driver.driver_name.clone();
                                      let short_name = format!(" ({})", driver.short_name);
                                      let car_name = driver.car_model.name().to_string();
                                      let fastest = driver.fastest_lap_formatted();
                                      let average = driver.average_lap_formatted();
                                      let lap_count = format!("{}/{}", driver.valid_laps, driver.total_laps);
                                      let seven_class = if driver.has_seven_consecutive_valid {
                                          "seven-valid yes"
                                      } else {
                                          "seven-valid no"
                                      };
                                      let seven_text = if driver.has_seven_consecutive_valid {
                                          "Yes"
                                      } else {
                                          "No"
                                      };

                                      view! {
                                          <tr>
                                              <td class="race-number">{race_number}</td>
                                              <td class="driver-name">
                                                  {driver_name}
                                                  <span class="short-name">{short_name}</span>
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
