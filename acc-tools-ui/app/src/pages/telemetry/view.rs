use acc_tools_ui_components::components::ui::data_table::*;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use motec_viewer_lib::{channel_description, fmt_duration};

use crate::state::TelemetryState;

fn fmt_f64(v: f64) -> String {
  if v.is_nan() || v.is_infinite() {
    "—".to_string()
  } else {
    format!("{v:.3}")
  }
}

#[component]
pub fn TelemetryViewPage() -> impl IntoView {
  let state = expect_context::<TelemetryState>();

  Effect::new(move || {
    if state.session.get().is_none() {
      let navigate = use_navigate();
      navigate("/telemetry", Default::default());
    }
  });

  let on_new = move |_| {
    state.session.set(None);
    state.laps.set(vec![]);
    state.error.set(None);
    let navigate = use_navigate();
    navigate("/telemetry", Default::default());
  };

  view! {
    <main class="container mx-auto max-w-7xl px-4 py-8 space-y-6">
      <Show when=move || state.session.get().is_some()>
        {move || {
          let session = state.session.get().unwrap();
          let laps = state.laps.get();

          view! {
            <div class="flex items-center justify-between">
              <h1 class="text-2xl font-bold">"Telemetry"</h1>
              <div class="flex items-center gap-2">
                <button
                  on:click=move |_| {
                    let navigate = use_navigate();
                    navigate("/telemetry/laps", Default::default());
                  }
                  class="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
                >
                  "View Laps"
                </button>
                <button
                  on:click=on_new
                  class="inline-flex items-center gap-2 rounded-md bg-secondary px-4 py-2 text-sm font-medium text-secondary-foreground shadow-sm transition-colors hover:bg-secondary/80"
                >
                  "Load New File"
                </button>
              </div>
            </div>

            // ── Session metadata ─────────────────────────────────────────
            <div class="rounded-lg border bg-card p-4 shadow-sm">
              <h2 class="text-lg font-semibold text-card-foreground mb-3">"Session Info"</h2>
              <div class="grid grid-cols-2 gap-x-8 gap-y-1 sm:grid-cols-3">
                <MetaRow label="Driver" value=session.driver.clone() />
                <MetaRow label="Vehicle" value=session.vehicle_id.clone() />
                <MetaRow label="Venue" value=session.venue.clone() />
                <MetaRow label="Date" value=format!("{} {}", session.date, session.time_of_day) />
                <MetaRow label="Device" value=session.device_type.clone() />
                <MetaRow label="Channels" value=session.channels.len().to_string() />
                {(!session.event_name.is_empty()).then(|| view! {
                  <MetaRow label="Event" value=session.event_name.clone() />
                })}
                {(!session.short_comment.is_empty()).then(|| view! {
                  <MetaRow label="Comment" value=session.short_comment.clone() />
                })}
              </div>
            </div>

            // ── Lap table ────────────────────────────────────────────────
            {(!laps.is_empty()).then(|| view! {
              <div class="rounded-lg border bg-card p-4 shadow-sm">
                <h2 class="text-lg font-semibold text-card-foreground mb-3">"Laps"</h2>
                <DataTableWrapper>
                  <DataTable>
                    <DataTableHeader>
                      <DataTableRow>
                        <DataTableHead>"Lap"</DataTableHead>
                        <DataTableHead>"Start"</DataTableHead>
                        <DataTableHead>"Duration"</DataTableHead>
                      </DataTableRow>
                    </DataTableHeader>
                    <DataTableBody>
                      {laps.iter().map(|lap| {
                        let lap_no = lap.lap_number.to_string();
                        let start = fmt_duration(lap.start_time_s);
                        let dur = lap.duration_s.map(fmt_duration).unwrap_or_else(|| "—".to_string());
                        view! {
                          <DataTableRow>
                            <DataTableCell class="tabular-nums">{lap_no}</DataTableCell>
                            <DataTableCell class="tabular-nums">{start}</DataTableCell>
                            <DataTableCell class="tabular-nums">{dur}</DataTableCell>
                          </DataTableRow>
                        }
                      }).collect_view()}
                    </DataTableBody>
                  </DataTable>
                </DataTableWrapper>
              </div>
            })}

            // ── Channel table ────────────────────────────────────────────
            <div class="space-y-3">
              <h2 class="text-lg font-semibold">"Channels"</h2>
              <DataTableWrapper>
                <DataTable>
                  <DataTableHeader>
                    <DataTableRow>
                      <DataTableHead>"Channel"</DataTableHead>
                      <DataTableHead>"Description"</DataTableHead>
                      <DataTableHead>"Unit"</DataTableHead>
                      <DataTableHead>"Hz"</DataTableHead>
                      <DataTableHead>"Min"</DataTableHead>
                      <DataTableHead>"Max"</DataTableHead>
                      <DataTableHead>"Avg"</DataTableHead>
                    </DataTableRow>
                  </DataTableHeader>
                  <DataTableBody>
                    {session.channels.iter().map(|ch| {
                      let name = ch.name.clone();
                      let desc = channel_description(&ch.name);
                      let unit = ch.unit.clone();
                      let freq = ch.frequency.to_string();
                      let min = fmt_f64(ch.min);
                      let max = fmt_f64(ch.max);
                      let avg = fmt_f64(ch.avg);
                      view! {
                        <DataTableRow>
                          <DataTableCell>{name}</DataTableCell>
                          <DataTableCell class="text-muted-foreground">{desc}</DataTableCell>
                          <DataTableCell class="text-muted-foreground">{unit}</DataTableCell>
                          <DataTableCell class="tabular-nums">{freq}</DataTableCell>
                          <DataTableCell class="tabular-nums">{min}</DataTableCell>
                          <DataTableCell class="tabular-nums">{max}</DataTableCell>
                          <DataTableCell class="tabular-nums">{avg}</DataTableCell>
                        </DataTableRow>
                      }
                    }).collect_view()}
                  </DataTableBody>
                </DataTable>
              </DataTableWrapper>
            </div>
          }
        }}
      </Show>
    </main>
  }
}

#[component]
fn MetaRow(label: &'static str, value: String) -> impl IntoView {
  view! {
    <p class="text-sm text-muted-foreground">
      <strong class="text-foreground">{label}": "</strong>
      {value}
    </p>
  }
}
