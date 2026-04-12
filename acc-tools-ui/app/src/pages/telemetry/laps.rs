use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use motec_viewer_lib::LdChannel;

use crate::state::TelemetryState;

const LAP_COLORS: &[&str] = &[
  "#60a5fa", "#f87171", "#34d399", "#fbbf24", "#a78bfa", "#fb923c", "#38bdf8", "#f472b6",
  "#4ade80", "#e879f9",
];

const CHART_W: f64 = 1000.0;
const CHART_H: f64 = 100.0;

fn fmt_lap_time(secs: f64) -> String {
  if secs.is_nan() || secs < 0.0 {
    return "—".to_string();
  }
  let millis = (secs.fract() * 1000.0) as u64;
  let total = secs as u64;
  format!("{}:{:02}.{:03}", total / 60, total % 60, millis)
}

/// Global [min, max] for `ch` across the given (start_s, dur_s) slices.
fn channel_range(ch: &LdChannel, slices: &[(f64, f64)]) -> (f64, f64) {
  let mut mn = f64::INFINITY;
  let mut mx = f64::NEG_INFINITY;
  for &(start_s, dur_s) in slices {
    let si = (start_s * ch.frequency as f64) as usize;
    let ei = ((start_s + dur_s) * ch.frequency as f64) as usize;
    let si = si.min(ch.samples.len());
    let ei = ei.min(ch.samples.len());
    for &v in &ch.samples[si..ei] {
      if v.is_finite() {
        mn = mn.min(v);
        mx = mx.max(v);
      }
    }
  }
  (mn, mx)
}

/// SVG polyline points for `ch` within one lap, normalised to the given y-range
/// and x-range [0, reference_dur].
fn polyline_points(
  ch: &LdChannel,
  lap_start_s: f64,
  lap_dur_s: f64,
  global_min: f64,
  global_max: f64,
  reference_dur: f64,
) -> String {
  if lap_dur_s <= 0.0 || ch.frequency == 0 || ch.samples.is_empty() || reference_dur <= 0.0 {
    return String::new();
  }
  let si = (lap_start_s * ch.frequency as f64) as usize;
  let ei = ((lap_start_s + lap_dur_s) * ch.frequency as f64) as usize;
  let si = si.min(ch.samples.len());
  let ei = ei.min(ch.samples.len());
  if si >= ei {
    return String::new();
  }

  let samples = &ch.samples[si..ei];
  let n = samples.len();
  let range = global_max - global_min;
  let step = (n / 2000).max(1);
  let freq = ch.frequency as f64;

  samples
    .iter()
    .enumerate()
    .filter(|(i, _)| i % step == 0)
    .map(|(i, &v)| {
      let t = i as f64 / freq;
      let x = (t / reference_dur) * CHART_W;
      let y = if range < 1e-10 {
        CHART_H / 2.0
      } else {
        CHART_H - ((v - global_min) / range) * CHART_H
      };
      format!("{x:.1},{y:.1}")
    })
    .collect::<Vec<_>>()
    .join(" ")
}

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
  let visible_ch: RwSignal<Vec<bool>> = RwSignal::new(vec![]);

  Effect::new(move || {
    if let Some(session) = state.session.get() {
      let n = session.channels.len();
      visible_ch.set((0..n).map(|i| i < 5).collect());
    }
  });

  // reset cursor when lap selection changes
  Effect::new(move || {
    let _ = selected_laps.get();
    cursor_time.set(0.0);
  });

  view! {
    <main class="container mx-auto max-w-7xl px-4 py-8 space-y-6">
      <Show when=move || state.session.get().is_some()>
        {move || {
          let laps = state.laps.get();

          view! {
            <div class="flex items-center justify-between">
              <h1 class="text-2xl font-bold">"Laps"</h1>
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
              }.into_any()
            } else {
              view! {
                <div class="flex gap-6">

                  // ── Lap list (multi-select) ───────────────────────────────────
                  <div class="w-48 shrink-0 space-y-1">
                    <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">
                      "Select Laps"
                    </p>
                    {laps.iter().map(|lap| {
                      let idx = lap.lap_number - 1;
                      let label = format!("Lap {}", lap.lap_number);
                      let dur = lap.duration_s.map(fmt_lap_time).unwrap_or_else(|| "—".to_string());
                      let color = LAP_COLORS[idx % LAP_COLORS.len()];
                      view! {
                        <button
                          class=move || if selected_laps.get().contains(&idx) {
                            "w-full text-left px-3 py-2 rounded-md text-sm transition-colors border-2"
                          } else {
                            "w-full text-left px-3 py-2 rounded-md text-sm transition-colors border bg-card hover:bg-accent hover:text-accent-foreground"
                          }
                          style=move || if selected_laps.get().contains(&idx) {
                            format!("border-color:{color};background-color:{color}22;color:{color}")
                          } else {
                            String::new()
                          }
                          on:click=move |_| {
                            let mut v = selected_laps.get();
                            if let Some(pos) = v.iter().position(|&x| x == idx) {
                              v.remove(pos);
                            } else {
                              v.push(idx);
                            }
                            selected_laps.set(v);
                          }
                        >
                          <div class="font-medium">{label}</div>
                          <div class="text-xs opacity-70 tabular-nums">{dur}</div>
                        </button>
                      }
                    }).collect_view()}
                  </div>

                  // ── Detail panel ─────────────────────────────────────────────
                  <div class="flex-1 min-w-0">
                    {move || {
                      let sel = selected_laps.get();

                      if sel.is_empty() {
                        return view! {
                          <div
                            class="rounded-lg border bg-card flex items-center justify-center text-muted-foreground"
                            style="min-height:320px"
                          >
                            "← Select one or more laps to compare"
                          </div>
                        }.into_any();
                      }

                      let laps = state.laps.get();
                      let session = state.session.get().unwrap();

                      // (lap_idx, start_s, dur_s) for each selected lap
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

                      view! {
                        <div class="space-y-4">

                          // ── Lap colour legend ──────────────────────────────
                          <div class="flex flex-wrap gap-3">
                            {laps_info.iter().map(|&(lap_idx, _, dur_s)| {
                              let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                              let label = format!("Lap {}", lap_idx + 1);
                              let time = fmt_lap_time(dur_s);
                              view! {
                                <div
                                  class="flex items-center gap-1.5 text-xs font-medium"
                                  style=format!("color:{color}")
                                >
                                  <span
                                    class="inline-block w-4 h-0.5 rounded"
                                    style=format!("background:{color}")
                                  />
                                  {label}
                                  <span class="opacity-60 tabular-nums">{time}</span>
                                </div>
                              }
                            }).collect_view()}
                          </div>

                          // ── Channel toggles ────────────────────────────────
                          <div class="rounded-lg border bg-card p-3">
                            <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">
                              "Channels"
                            </p>
                            <div class="flex flex-wrap gap-2">
                              {session.channels.iter().enumerate().map(|(i, ch)| {
                                let label = if ch.short_name.is_empty() {
                                  ch.name.clone()
                                } else {
                                  ch.short_name.clone()
                                };
                                view! {
                                  <button
                                    class=move || {
                                      let on = visible_ch.get().get(i).copied().unwrap_or(false);
                                      if on {
                                        "px-2 py-1 rounded text-xs font-medium border-2 border-primary bg-primary/10 text-primary"
                                      } else {
                                        "px-2 py-1 rounded text-xs font-medium border border-border text-muted-foreground"
                                      }
                                    }
                                    on:click=move |_| {
                                      let mut v = visible_ch.get();
                                      if let Some(b) = v.get_mut(i) {
                                        *b = !*b;
                                      }
                                      visible_ch.set(v);
                                    }
                                  >
                                    {label}
                                  </button>
                                }
                              }).collect_view()}
                            </div>
                          </div>

                          // ── Per-channel charts ─────────────────────────────
                          {move || {
                            let vis = visible_ch.get();
                            let sel = selected_laps.get();
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

                            let slices: Vec<(f64, f64)> =
                              laps_info.iter().map(|&(_, s, d)| (s, d)).collect();

                            session
                              .channels
                              .iter()
                              .enumerate()
                              .filter_map(|(ch_i, ch)| {
                                if !vis.get(ch_i).copied().unwrap_or(false) {
                                  return None;
                                }

                                let ch_label = if ch.short_name.is_empty() {
                                  ch.name.clone()
                                } else {
                                  ch.short_name.clone()
                                };
                                let unit = ch.unit.clone();
                                let heading = if unit.is_empty() {
                                  ch_label
                                } else {
                                  format!("{ch_label} ({unit})")
                                };

                                let (global_min, global_max) = channel_range(ch, &slices);

                                let polylines: Vec<(String, &'static str)> = laps_info
                                  .iter()
                                  .map(|&(lap_idx, start_s, dur_s)| {
                                    let pts = polyline_points(
                                      ch,
                                      start_s,
                                      dur_s,
                                      global_min,
                                      global_max,
                                      reference_dur,
                                    );
                                    let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                                    (pts, color)
                                  })
                                  .collect();

                                Some(view! {
                                  <div class="rounded-lg border bg-card p-3">
                                    <p class="text-xs font-semibold text-muted-foreground mb-1">
                                      {heading}
                                    </p>
                                    <svg
                                      viewBox=format!("0 0 {CHART_W} {CHART_H}")
                                      class="w-full block"
                                      style="height:100px"
                                      preserveAspectRatio="none"
                                    >
                                      // grid lines
                                      {(1..4u8).map(|i| {
                                        let y = CHART_H * i as f64 / 4.0;
                                        view! {
                                          <line
                                            x1="0" y1=y.to_string()
                                            x2=CHART_W.to_string() y2=y.to_string()
                                            stroke="#334155" stroke-width="0.5"
                                          />
                                        }
                                      }).collect_view()}

                                      // lap polylines
                                      {polylines
                                        .into_iter()
                                        .filter(|(pts, _)| !pts.is_empty())
                                        .map(|(pts, color)| {
                                          view! {
                                            <polyline
                                              points=pts
                                              fill="none"
                                              stroke=color
                                              stroke-width="1.5"
                                              vector-effect="non-scaling-stroke"
                                            />
                                          }
                                        })
                                        .collect_view()}

                                      // cursor
                                      {move || {
                                        let x = if reference_dur > 0.0 {
                                          (cursor_time.get() / reference_dur) * CHART_W
                                        } else {
                                          0.0
                                        };
                                        view! {
                                          <line
                                            x1=x.to_string() y1="0"
                                            x2=x.to_string() y2=CHART_H.to_string()
                                            stroke="white" stroke-width="1" opacity="0.8"
                                          />
                                        }
                                      }}
                                    </svg>
                                    <div class="flex justify-between text-xs text-muted-foreground tabular-nums mt-1">
                                      <span>"0:00.000"</span>
                                      <span>{fmt_lap_time(reference_dur)}</span>
                                    </div>
                                  </div>
                                })
                              })
                              .collect_view()
                          }}

                          // ── Scrubber ───────────────────────────────────────
                          <div class="space-y-1">
                            <input
                              type="range"
                              min="0"
                              max=format!("{reference_dur:.2}")
                              step="0.01"
                              class="w-full accent-primary"
                              prop:value=move || format!("{:.2}", cursor_time.get())
                              on:input=move |ev| {
                                use leptos::wasm_bindgen::JsCast;
                                if let Some(v) = ev
                                  .target()
                                  .and_then(|t| {
                                    t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok()
                                  })
                                  .and_then(|el| el.value().parse::<f64>().ok())
                                {
                                  cursor_time.set(v);
                                }
                              }
                            />
                            <div class="flex justify-between text-xs text-muted-foreground tabular-nums">
                              <span>"0:00.000"</span>
                              <span class="font-medium text-foreground">
                                {move || format!("T + {}", fmt_lap_time(cursor_time.get()))}
                              </span>
                              <span>{fmt_lap_time(reference_dur)}</span>
                            </div>
                          </div>

                          // ── Values at cursor ───────────────────────────────
                          {move || {
                            let vis = visible_ch.get();
                            let sel = selected_laps.get();
                            let t = cursor_time.get();
                            let laps = state.laps.get();
                            let session = state.session.get().unwrap();

                            let laps_info: Vec<(usize, f64)> = sel
                              .iter()
                              .filter_map(|&i| {
                                let lap = laps.get(i)?;
                                Some((i, lap.start_time_s))
                              })
                              .collect();

                            view! {
                              <div class="rounded-lg border bg-card p-3">
                                <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">
                                  "Values at cursor"
                                </p>
                                <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
                                  {session
                                    .channels
                                    .iter()
                                    .enumerate()
                                    .filter_map(|(ch_i, ch)| {
                                      if !vis.get(ch_i).copied().unwrap_or(false) {
                                        return None;
                                      }
                                      let ch_label = if ch.short_name.is_empty() {
                                        ch.name.clone()
                                      } else {
                                        ch.short_name.clone()
                                      };
                                      let unit = ch.unit.clone();
                                      let per_lap: Vec<(usize, String, &'static str)> = laps_info
                                        .iter()
                                        .map(|&(lap_idx, start_s)| {
                                          let abs_t = start_s + t;
                                          let si = (abs_t * ch.frequency as f64) as usize;
                                          let val = ch
                                            .samples
                                            .get(si)
                                            .map(|&v| format!("{v:.2}"))
                                            .unwrap_or_else(|| "—".to_string());
                                          let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                                          (lap_idx + 1, val, color)
                                        })
                                        .collect();

                                      Some(view! {
                                        <div class="rounded border border-border p-2">
                                          <div class="text-xs text-muted-foreground mb-1">
                                            {ch_label}
                                            {if unit.is_empty() {
                                              String::new()
                                            } else {
                                              format!(" ({unit})")
                                            }}
                                          </div>
                                          {per_lap
                                            .into_iter()
                                            .map(|(lap_no, val, color)| {
                                              view! {
                                                <div class="flex items-center justify-between gap-1">
                                                  <span
                                                    class="text-xs tabular-nums"
                                                    style=format!("color:{color}")
                                                  >
                                                    {format!("L{lap_no}")}
                                                  </span>
                                                  <span
                                                    class="font-mono font-semibold tabular-nums text-sm"
                                                    style=format!("color:{color}")
                                                  >
                                                    {val}
                                                  </span>
                                                </div>
                                              }
                                            })
                                            .collect_view()}
                                        </div>
                                      })
                                    })
                                    .collect_view()}
                                </div>
                              </div>
                            }
                          }}

                        </div>
                      }.into_any()
                    }}
                  </div>
                </div>
              }.into_any()
            }}
          }
        }}
      </Show>
    </main>
  }
}
