use crate::CopyToClipboardButton;
use crate::components::ui::data_table::*;
use acc_session_viewer_lib::car::car_model::CarModel;
use acc_session_viewer_lib::driver_stats::{
  DriverStatsCommon, filter_stats_by_car_model, stats_to_csv,
};
use acc_session_viewer_lib::loaded_session::LoadedSession;
use leptos::prelude::*;

#[component]
pub fn DriverStatsTable(
  session: ReadSignal<Option<LoadedSession>>,
  #[allow(unused_variables)] selected_car_model: ReadSignal<Option<CarModel>>,
  #[prop(optional)] class: Option<String>,
) -> impl IntoView {
  let stats = move || {
    session.get().map(|s| match selected_car_model.get() {
      Some(car_model) => filter_stats_by_car_model(s.stats.clone(), car_model),
      None => s.stats.clone(),
    })
  };

  let csv_text = Signal::derive(move || stats_to_csv(&stats().unwrap_or_default()));

  let wrapper_class = class
    .map(|c| format!("space-y-4 {}", c))
    .unwrap_or_else(|| "space-y-4".to_string());

  view! {
    <div class=wrapper_class>
      {move || {
        match stats() {
          Some(driver_stats) if !driver_stats.is_empty() => {
            view! {
              <div class="flex items-center justify-between">
                <h3 class="text-lg font-semibold">"Driver Statistics"</h3>
                <CopyToClipboardButton
                  text=csv_text
                  label="Copy as CSV".to_string()
                />
              </div>
              <DataTableWrapper>
                <DataTable>
                  <DataTableHeader>
                    <DataTableRow>
                      <DataTableHead>"#"</DataTableHead>
                      <DataTableHead>"Driver"</DataTableHead>
                      <DataTableHead>"Car"</DataTableHead>
                      <DataTableHead>"Fastest"</DataTableHead>
                      <DataTableHead>"Average"</DataTableHead>
                      <DataTableHead>"Laps"</DataTableHead>
                      <DataTableHead>"7 Valid"</DataTableHead>
                    </DataTableRow>
                  </DataTableHeader>
                  <DataTableBody>
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
                        "text-green-600 font-medium"
                      } else {
                        "text-red-600"
                      };
                      let seven_text = if driver.has_seven_consecutive_valid() {
                        "Yes"
                      } else {
                        "No"
                      };

                      view! {
                        <DataTableRow>
                          <DataTableCell class="tabular-nums">{race_number}</DataTableCell>
                          <DataTableCell>
                            {driver_name}
                            <Show when=move || show_short_name>
                              <span class="text-muted-foreground text-xs ml-1">{short_name_display.clone()}</span>
                            </Show>
                          </DataTableCell>
                          <DataTableCell>{car_name}</DataTableCell>
                          <DataTableCell class="tabular-nums text-green-600 font-medium">{fastest}</DataTableCell>
                          <DataTableCell class="tabular-nums">{average}</DataTableCell>
                          <DataTableCell class="font-mono">{lap_count}</DataTableCell>
                          <DataTableCell class=seven_class>{seven_text}</DataTableCell>
                        </DataTableRow>
                      }
                    }).collect_view()}
                  </DataTableBody>
                </DataTable>
              </DataTableWrapper>
            }.into_any()
          }
          Some(_) => view! { <p class="text-muted-foreground text-center py-8">"No drivers found"</p> }.into_any(),
          None => view! { <p class="text-muted-foreground text-center py-8">"Upload a session file to view statistics"</p> }.into_any()
        }
      }}
    </div>
  }
}
