mod channel_groups;
mod delta;
mod helpers;
mod lap_list;
mod scrubber;
mod track_map;
mod widgets;

use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use motec_viewer_lib::LdChannel;

use crate::state::TelemetryState;
use channel_groups::{
  CHANNEL_GROUPS, ChannelGroupCard, InteractiveChart, build_charts, channel_group_index,
};
use delta::DeltaGraph;
use helpers::{LAP_COLORS, fmt_lap_time};
use lap_list::LapSelector;
use scrubber::CursorValues;
use track_map::TrackMap;
use widgets::{FourCornerWidget, HudWidget, PedalBars, SteeringWheel, SuspensionSvg, TyrePanel};

// ── View tabs ──────────────────────────────────────────────────────────────────

const VIEWS: &[&str] = &[
  "Overview",
  "Driver Inputs",
  "Tyres & Brakes",
  "Suspension",
  "Electronics",
  "All Channels",
];

// ── Page ───────────────────────────────────────────────────────────────────────

#[component]
pub fn TelemetryLapsPage() -> impl IntoView {
  let state = expect_context::<TelemetryState>();

  Effect::new(move || {
    if state.session.get().is_none() {
      let navigate = use_navigate();
      navigate("/telemetry", Default::default());
    }
  });

  let selected_laps: RwSignal<Vec<usize>> = RwSignal::new(vec![]);
  let cursor_time: RwSignal<f64> = RwSignal::new(0.0);
  let active_view: RwSignal<&'static str> = RwSignal::new("Overview");

  Effect::new(move || {
    let _ = selected_laps.get();
    cursor_time.set(0.0);
  });

  view! {
    <main class="container mx-auto max-w-7xl px-4 py-6 space-y-4">
      <Show when=move || state.session.get().is_some()>
        {move || {
          let laps = state.laps.get();
          view! {
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <h1 class="text-2xl font-bold">"Telemetry"</h1>
                <button
                  on:click=move |_| {
                    let navigate = use_navigate();
                    navigate("/telemetry/view", Default::default());
                  }
                  class="inline-flex items-center gap-2 rounded-md bg-secondary px-4 py-2 text-sm font-medium text-secondary-foreground shadow-sm transition-colors hover:bg-secondary/80"
                >
                  "← Back"
                </button>
              </div>

              {if laps.is_empty() {
                view! {
                  <div class="rounded-lg border bg-card p-8 text-center text-muted-foreground">
                    "No lap data available. Load a "
                    <code class="font-mono">".ldx"</code>
                    " file alongside your "
                    <code class="font-mono">".ld"</code>
                    " file to see individual laps."
                  </div>
                }
                .into_any()
              } else {
                view! {
                  <div class="flex gap-4 items-start">
                    // ── Left: lap selector sidebar ──
                    <div class="flex-shrink-0 w-44 sticky top-4">
                      <LapSelector laps=laps.clone() selected_laps />
                    </div>

                    // ── Right: tab content ──
                    <div class="flex-1 min-w-0">
                  {move || {
                    let sel = selected_laps.get();
                    if sel.is_empty() {
                      return view! {
                        <div
                          class="rounded-lg border bg-card flex items-center justify-center text-muted-foreground"
                          style="min-height:200px"
                        >
                          "Select one or more laps →"
                        </div>
                      }
                      .into_any();
                    }

                    let laps = state.laps.get();
                    let session = state.session.get().unwrap();

                    let laps_info: Vec<(usize, f64, f64)> = sel
                      .iter()
                      .filter_map(|&i| {
                        let lap = laps.get(i)?;
                        let dur = lap.duration_s.unwrap_or_else(|| {
                          session
                            .channels
                            .first()
                            .map(|ch| ch.duration_s() - lap.start_time_s)
                            .unwrap_or(60.0)
                        });
                        Some((i, lap.start_time_s, dur))
                      })
                      .collect();

                    let reference_dur =
                      laps_info.iter().map(|&(_, _, d)| d).fold(0.0f64, f64::max);

                    // Bucket channels into groups + overflow
                    let mut groups: Vec<Vec<LdChannel>> =
                      vec![vec![]; CHANNEL_GROUPS.len() + 1];
                    for ch in &session.channels {
                      let g = channel_group_index(&ch.name).unwrap_or(CHANNEL_GROUPS.len());
                      groups[g].push(ch.clone());
                    }
                    let other_channels = groups.pop().unwrap_or_default();

                    // Channel indices for cursor values panel
                    const SKIP: &[&str] = &["LAP_BEACON", "TIME"];
                    let channel_indices: Vec<usize> = session
                      .channels
                      .iter()
                      .enumerate()
                      .filter(|(_, ch)| {
                        channel_group_index(&ch.name).is_some()
                          && !SKIP.contains(&ch.name.as_str())
                      })
                      .map(|(i, _)| i)
                      .collect();

                    let laps_info_cursor: Vec<(usize, f64)> =
                      laps_info.iter().map(|&(i, s, _)| (i, s)).collect();

                    // Extract channel groups by index (cloned)
                    let group_idx = |title: &str| -> usize {
                      CHANNEL_GROUPS.iter().position(|g| g.title == title).unwrap_or(999)
                    };
                    let inputs_chs = groups.get(group_idx("Driver Inputs")).cloned().unwrap_or_default();
                    let motion_chs = groups.get(group_idx("Motion & Speed")).cloned().unwrap_or_default();
                    let engine_chs = groups.get(group_idx("Engine")).cloned().unwrap_or_default();
                    let tyres_chs  = groups.get(group_idx("Tyres")).cloned().unwrap_or_default();
                    let brakes_chs = groups.get(group_idx("Brake Temperatures")).cloned().unwrap_or_default();
                    let susp_chs   = groups.get(group_idx("Suspension")).cloned().unwrap_or_default();
                    let elec_chs   = groups.get(group_idx("Electronics")).cloned().unwrap_or_default();

                    // Wrap shared data in StoredValue so it can be accessed from multiple <Show> closures
                    let session_sv      = StoredValue::new(session);
                    let laps_info_sv    = StoredValue::new(laps_info.clone());
                    let inputs_sv       = StoredValue::new(inputs_chs);
                    let motion_sv       = StoredValue::new(motion_chs);
                    let engine_sv       = StoredValue::new(engine_chs);
                    let tyres_sv        = StoredValue::new(tyres_chs);
                    let brakes_sv       = StoredValue::new(brakes_chs);
                    let susp_sv         = StoredValue::new(susp_chs);
                    let elec_sv         = StoredValue::new(elec_chs);
                    let groups_sv       = StoredValue::new(groups);
                    let other_sv        = StoredValue::new(other_channels);
                    let channel_idx_sv  = StoredValue::new(channel_indices);
                    let cursor_sv       = StoredValue::new(laps_info_cursor);

                    view! {
                      <div class="space-y-4">
                        // Lap colour legend
                        <div class="flex flex-wrap gap-3">
                          {laps_info
                            .iter()
                            .map(|&(lap_idx, _, dur_s)| {
                              let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                              view! {
                                <div
                                  class="flex items-center gap-1.5 text-xs font-medium"
                                  style=format!("color:{color}")
                                >
                                  <span class="inline-block w-4 h-0.5 rounded" style=format!("background:{color}") />
                                  {format!("Lap {}", lap_idx + 1)}
                                  <span class="opacity-60 tabular-nums">{fmt_lap_time(dur_s)}</span>
                                </div>
                              }
                            })
                            .collect_view()}
                        </div>

                        // View tabs
                        <div class="flex flex-wrap gap-1">
                          {VIEWS.iter().map(|&v| view! {
                            <button
                              class=move || if active_view.get() == v {
                                "px-3 py-1.5 rounded-md text-xs font-medium bg-primary text-primary-foreground"
                              } else {
                                "px-3 py-1.5 rounded-md text-xs font-medium bg-secondary text-secondary-foreground hover:bg-secondary/80"
                              }
                              on:click=move |_| active_view.set(v)
                            >{v}</button>
                          }).collect_view()}
                        </div>

                        // ── Overview ──────────────────────────────────────────
                        <Show when=move || active_view.get() == "Overview">
                          <div class="space-y-4">
                            // Row 1: HUD + Track map
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                              <HudWidget
                                inputs_channels=inputs_sv.get_value()
                                motion_channels=motion_sv.get_value()
                                engine_channels=engine_sv.get_value()
                                laps_info=laps_info_sv.get_value()
                                cursor_time
                              />
                              <TrackMap
                                session=session_sv.get_value()
                                laps_info=laps_info_sv.get_value()
                                cursor_time
                              />
                            </div>

                            // Row 2: Speed trace + Delta time
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                              {motion_sv.with_value(|chs| {
                                let speed_chs: Vec<_> = chs.iter().filter(|c| c.name == "SPEED").cloned().collect();
                                let charts = build_charts(&speed_chs, &laps_info_sv.get_value(), reference_dur);
                                charts.into_iter().map(|chart| view! {
                                  <div class="rounded-lg border bg-card p-3">
                                    <p class="text-xs font-semibold text-muted-foreground mb-2">"Speed trace"</p>
                                    <InteractiveChart chart reference_dur cursor_time height_px=80 />
                                    <div class="flex justify-between text-xs text-muted-foreground tabular-nums mt-1">
                                      <span>"0:00.000"</span>
                                      <span>{fmt_lap_time(reference_dur)}</span>
                                    </div>
                                  </div>
                                }).collect_view()
                              })}
                              <DeltaGraph
                                session=session_sv.get_value()
                                laps_info=laps_info_sv.get_value()
                                reference_dur cursor_time
                              />
                            </div>

                            // Row 3: Tyre & Brake panel
                            <TyrePanel
                              channels={
                                let mut all = tyres_sv.get_value();
                                all.extend(brakes_sv.get_value());
                                all
                              }
                              laps_info=laps_info_sv.get_value()
                              cursor_time
                            />
                          </div>
                        </Show>

                        // ── Driver Inputs ────────────────────────────────────
                        <Show when=move || active_view.get() == "Driver Inputs">
                          <div class="space-y-4">
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                              <SteeringWheel
                                channels=inputs_sv.get_value()
                                laps_info=laps_info_sv.get_value()
                                cursor_time
                              />
                              <PedalBars
                                channels=inputs_sv.get_value()
                                laps_info=laps_info_sv.get_value()
                                cursor_time
                              />
                            </div>
                            <ChannelGroupCard
                              title="Driver Inputs"
                              channels=inputs_sv.get_value()
                              laps_info=laps_info_sv.get_value()
                              reference_dur cursor_time default_open=true
                            />
                            <DeltaGraph
                              session=session_sv.get_value()
                              laps_info=laps_info_sv.get_value()
                              reference_dur cursor_time
                            />
                          </div>
                        </Show>

                        // ── Tyres & Brakes ───────────────────────────────────
                        <Show when=move || active_view.get() == "Tyres & Brakes">
                          <div class="space-y-4">
                            // Combined 4-corner panel
                            <TyrePanel
                              channels={
                                let mut all = tyres_sv.get_value();
                                all.extend(brakes_sv.get_value());
                                all.extend(susp_sv.get_value());
                                all
                              }
                              laps_info=laps_info_sv.get_value()
                              cursor_time
                            />
                            <ChannelGroupCard
                              title="Tyres"
                              channels=tyres_sv.get_value()
                              laps_info=laps_info_sv.get_value()
                              reference_dur cursor_time default_open=true
                            />
                            <ChannelGroupCard
                              title="Brake Temperatures"
                              channels=brakes_sv.get_value()
                              laps_info=laps_info_sv.get_value()
                              reference_dur cursor_time default_open=true
                            />
                          </div>
                        </Show>

                        // ── Suspension ───────────────────────────────────────
                        <Show when=move || active_view.get() == "Suspension">
                          <div class="space-y-4">
                            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                              <SuspensionSvg
                                channels=susp_sv.get_value()
                                laps_info=laps_info_sv.get_value()
                                cursor_time
                              />
                              <FourCornerWidget
                                channels=susp_sv.get_value()
                                laps_info=laps_info_sv.get_value()
                                cursor_time channel_prefix="SUS_TRAVEL"
                                title="Suspension Travel" unit="mm" decimals=1
                              />
                            </div>
                            <ChannelGroupCard
                              title="Suspension"
                              channels=susp_sv.get_value()
                              laps_info=laps_info_sv.get_value()
                              reference_dur cursor_time default_open=true
                            />
                          </div>
                        </Show>

                        // ── Electronics ──────────────────────────────────────
                        <Show when=move || active_view.get() == "Electronics">
                          <ChannelGroupCard
                            title="Electronics"
                            channels=elec_sv.get_value()
                            laps_info=laps_info_sv.get_value()
                            reference_dur cursor_time default_open=true
                          />
                        </Show>

                        // ── All Channels ─────────────────────────────────────
                        <Show when=move || active_view.get() == "All Channels">
                          <div class="space-y-4">
                            {CHANNEL_GROUPS.iter().enumerate().map(|(gi, group_def)| {
                              let channels = groups_sv.with_value(|gs| gs.get(gi).cloned().unwrap_or_default());
                              view! {
                                <ChannelGroupCard
                                  title=group_def.title
                                  channels
                                  laps_info=laps_info_sv.get_value()
                                  reference_dur cursor_time
                                  default_open=gi < 2
                                />
                              }
                            }).collect_view()}
                            <ChannelGroupCard
                              title="Other"
                              channels=other_sv.get_value()
                              laps_info=laps_info_sv.get_value()
                              reference_dur cursor_time default_open=false
                            />
                            <CursorValues
                              session=session_sv.get_value()
                              laps_info=cursor_sv.get_value()
                              cursor_time
                              channel_indices=channel_idx_sv.get_value()
                            />
                          </div>
                        </Show>
                      </div>
                    }
                    .into_any()
                  }}
                    </div>
                  </div>
                }
                .into_any()
              }}
            </div>
          }
        }}
      </Show>
    </main>
  }
}
