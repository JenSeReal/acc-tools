use acc_ui_components::components::ui::data_table::*;
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::state::TelemetryState;

/// Format a duration in seconds as mm:ss or hh:mm:ss.
fn fmt_duration(secs: f64) -> String {
  if secs.is_nan() || secs < 0.0 {
    return "—".to_string();
  }
  let total = secs as u64;
  let h = total / 3600;
  let m = (total % 3600) / 60;
  let s = total % 60;
  if h > 0 {
    format!("{h}:{m:02}:{s:02}")
  } else {
    format!("{m}:{s:02}")
  }
}

fn fmt_f64(v: f64, decimals: usize) -> String {
  if v.is_nan() || v.is_infinite() {
    "—".to_string()
  } else {
    format!("{v:.decimals$}")
  }
}

#[component]
pub fn TelemetryPage() -> impl IntoView {
  let state = expect_context::<TelemetryState>();

  // Redirect to upload if no session is loaded
  Effect::new(move || {
    if state.session.get().is_none() {
      let navigate = use_navigate();
      navigate("/", Default::default());
    }
  });

  let on_new = move |_| {
    state.session.set(None);
    state.laps.set(vec![]);
    state.error.set(None);
    let navigate = use_navigate();
    navigate("/", Default::default());
  };

  view! {
    <main class="container mx-auto max-w-7xl px-4 py-8 space-y-6">
      <Show when=move || state.session.get().is_some()>
        {move || {
          let session = state.session.get().unwrap();
          let laps = state.laps.get();

          view! {
            // ── Top bar ────────────────────────────────────────────────────
            <div class="flex items-center justify-between">
              <h1 class="text-2xl font-bold">"Telemetry"</h1>
              <button
                on:click=on_new
                class="inline-flex items-center gap-2 rounded-md bg-secondary px-4 py-2 text-sm font-medium text-secondary-foreground shadow-sm transition-colors hover:bg-secondary/80"
              >
                "Load New File"
              </button>
            </div>

            // ── Session metadata ───────────────────────────────────────────
            <div class="rounded-lg border bg-card p-4 shadow-sm space-y-1">
              <h2 class="text-lg font-semibold text-card-foreground mb-2">"Session Info"</h2>
              <MetaRow label="Driver" value=session.driver.clone() />
              <MetaRow label="Vehicle" value=session.vehicle_id.clone() />
              <MetaRow label="Venue" value=session.venue.clone() />
              <MetaRow label="Date" value=format!("{} {}", session.date, session.time_of_day) />
              <MetaRow label="Device" value=session.device_type.clone() />
              {(!session.event_name.is_empty()).then(|| view! {
                <MetaRow label="Event" value=session.event_name.clone() />
              })}
              {(!session.short_comment.is_empty()).then(|| view! {
                <MetaRow label="Comment" value=session.short_comment.clone() />
              })}
              <MetaRow label="Channels" value=session.channels.len().to_string() />
            </div>

            // ── Lap table (if .ldx was provided) ──────────────────────────
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
                        let dur = lap.duration_s
                          .map(fmt_duration)
                          .unwrap_or_else(|| "—".to_string());
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

            // ── Channel table ──────────────────────────────────────────────
            <div class="space-y-3">
              <h2 class="text-lg font-semibold">"Channels"</h2>
              <DataTableWrapper>
                <DataTable>
                  <DataTableHeader>
                    <DataTableRow>
                      <DataTableHead>"Channel"</DataTableHead>
                      <DataTableHead>"Short"</DataTableHead>
                      <DataTableHead>"Unit"</DataTableHead>
                      <DataTableHead>"Hz"</DataTableHead>
                      <DataTableHead>"Samples"</DataTableHead>
                      <DataTableHead>"Duration"</DataTableHead>
                      <DataTableHead>"Min"</DataTableHead>
                      <DataTableHead>"Max"</DataTableHead>
                      <DataTableHead>"Avg"</DataTableHead>
                    </DataTableRow>
                  </DataTableHeader>
                  <DataTableBody>
                    {session.channels.iter().map(|ch| {
                      let name = ch.name.clone();
                      let short = ch.short_name.clone();
                      let unit = ch.unit.clone();
                      let freq = ch.frequency.to_string();
                      let samples = ch.sample_count.to_string();
                      let dur = fmt_duration(ch.duration_s());
                      let min = fmt_f64(ch.min, 3);
                      let max = fmt_f64(ch.max, 3);
                      let avg = fmt_f64(ch.avg, 3);
                      view! {
                        <DataTableRow>
                          <DataTableCell>{name}</DataTableCell>
                          <DataTableCell class="text-muted-foreground">{short}</DataTableCell>
                          <DataTableCell class="text-muted-foreground">{unit}</DataTableCell>
                          <DataTableCell class="tabular-nums">{freq}</DataTableCell>
                          <DataTableCell class="tabular-nums">{samples}</DataTableCell>
                          <DataTableCell class="tabular-nums">{dur}</DataTableCell>
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

/// Small helper for the session info grid rows.
#[component]
fn MetaRow(label: &'static str, value: String) -> impl IntoView {
  view! {
    <p class="text-sm text-muted-foreground">
      <strong class="text-foreground">{label}": "</strong>
      {value}
    </p>
  }
}
