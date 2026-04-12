use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use motec_viewer_lib::LdChannel;

use crate::state::TelemetryState;

const COLORS: &[&str] = &[
  "#60a5fa", "#f87171", "#34d399", "#fbbf24", "#a78bfa", "#fb923c", "#38bdf8", "#f472b6",
  "#4ade80", "#e879f9",
];

const CHART_W: f64 = 1000.0;
const CHART_H: f64 = 200.0;

fn fmt_lap_time(secs: f64) -> String {
  if secs.is_nan() || secs < 0.0 {
    return "—".to_string();
  }
  let millis = (secs.fract() * 1000.0) as u64;
  let total = secs as u64;
  format!("{}:{:02}.{:03}", total / 60, total % 60, millis)
}

fn compute_polyline(ch: &LdChannel, lap_start_s: f64, lap_dur_s: f64) -> String {
  if lap_dur_s <= 0.0 || ch.frequency == 0 || ch.samples.is_empty() {
    return String::new();
  }
  let start_idx = (lap_start_s * ch.frequency as f64) as usize;
  let end_idx = ((lap_start_s + lap_dur_s) * ch.frequency as f64) as usize;
  let start_idx = start_idx.min(ch.samples.len());
  let end_idx = end_idx.min(ch.samples.len());
  if start_idx >= end_idx {
    return String::new();
  }

  let samples = &ch.samples[start_idx..end_idx];
  let n = samples.len();

  let (min_v, max_v) = samples
    .iter()
    .copied()
    .filter(|v| v.is_finite())
    .fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), v| {
      (mn.min(v), mx.max(v))
    });
  if !min_v.is_finite() {
    return String::new();
  }

  let range = max_v - min_v;
  let step = (n / 2000).max(1);

  samples
    .iter()
    .enumerate()
    .filter(|(i, _)| i % step == 0)
    .map(|(i, &v)| {
      let x = (i as f64 / (n.max(2) - 1) as f64) * CHART_W;
      let y = if range < 1e-10 {
        CHART_H / 2.0
      } else {
        CHART_H - ((v - min_v) / range) * CHART_H
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

  let selected_lap: RwSignal<Option<usize>> = RwSignal::new(None);
  let cursor_time: RwSignal<f64> = RwSignal::new(0.0);
  let visible_ch: RwSignal<Vec<bool>> = RwSignal::new(vec![]);

  Effect::new(move || {
    if let Some(session) = state.session.get() {
      let n = session.channels.len();
      visible_ch.set((0..n).map(|i| i < 5).collect());
    }
  });

  Effect::new(move || {
    let _ = selected_lap.get();
    cursor_time.set(0.0);
  });

  view! {
    <main class="container mx-auto max-w-7xl px-4 py-8 space-y-6">
      <Show when=move || state.session.get().is_some()>
        {move || {
          let laps = state.laps.get();

          view! {
            // ── Top bar ──────────────────────────────────────────────────────
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

            // ── No lap data ──────────────────────────────────────────────────
            {if laps.is_empty() {
              view! {
                <div class="rounded-lg border bg-card p-8 text-center text-muted-foreground">
                  "No lap data available. Load a "
                  <code class="font-mono">" .ldx "</code>
                  " file alongside your "
                  <code class="font-mono">" .ld "</code>
                  " file to see individual laps."
                </div>
              }.into_any()
            } else {
              view! {
                <div class="flex gap-6">

                  // ── Lap list ────────────────────────────────────────────────
                  <div class="w-44 shrink-0 space-y-1">
                    <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">
                      "Laps"
                    </p>
                    {laps.iter().map(|lap| {
                      let idx = lap.lap_number - 1;
                      let label = format!("Lap {}", lap.lap_number);
                      let dur = lap.duration_s.map(fmt_lap_time).unwrap_or_else(|| "—".to_string());
                      view! {
                        <button
                          class=move || if selected_lap.get() == Some(idx) {
                            "w-full text-left px-3 py-2 rounded-md text-sm transition-colors bg-primary text-primary-foreground"
                          } else {
                            "w-full text-left px-3 py-2 rounded-md text-sm transition-colors border bg-card hover:bg-accent hover:text-accent-foreground"
                          }
                          on:click=move |_| selected_lap.set(Some(idx))
                        >
                          <div class="font-medium">{label}</div>
                          <div class="text-xs opacity-70 tabular-nums">{dur}</div>
                        </button>
                      }
                    }).collect_view()}
                  </div>

                  // ── Detail panel ────────────────────────────────────────────
                  <div class="flex-1 min-w-0">
                    {move || match selected_lap.get() {
                      None => view! {
                        <div class="rounded-lg border bg-card flex items-center justify-center text-muted-foreground"
                          style="min-height:320px">
                          "← Select a lap to view its telemetry"
                        </div>
                      }.into_any(),

                      Some(lap_idx) => {
                        let laps = state.laps.get();
                        let session = state.session.get().unwrap();
                        let lap = laps[lap_idx].clone();
                        let lap_dur = lap.duration_s.unwrap_or_else(|| {
                          session.channels.first()
                            .map(|ch| ch.duration_s() - lap.start_time_s)
                            .unwrap_or(60.0)
                        });

                        view! {
                          <div class="space-y-4">

                            // ── Channel toggles ───────────────────────────────
                            <div class="rounded-lg border bg-card p-3">
                              <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">
                                "Channels"
                              </p>
                              <div class="flex flex-wrap gap-2">
                                {session.channels.iter().enumerate().map(|(i, ch)| {
                                  let color = COLORS[i % COLORS.len()];
                                  let label = if ch.short_name.is_empty() {
                                    ch.name.clone()
                                  } else {
                                    ch.short_name.clone()
                                  };
                                  view! {
                                    <button
                                      class=move || {
                                        let on = visible_ch.get().get(i).copied().unwrap_or(false);
                                        if on { "px-2 py-1 rounded text-xs font-medium border-2" }
                                        else { "px-2 py-1 rounded text-xs font-medium border border-border text-muted-foreground" }
                                      }
                                      style=move || {
                                        let on = visible_ch.get().get(i).copied().unwrap_or(false);
                                        if on {
                                          format!("border-color:{color};background-color:{color}22;color:{color}")
                                        } else {
                                          String::new()
                                        }
                                      }
                                      on:click=move |_| {
                                        let mut v = visible_ch.get();
                                        if let Some(b) = v.get_mut(i) { *b = !*b; }
                                        visible_ch.set(v);
                                      }
                                    >
                                      {label}
                                    </button>
                                  }
                                }).collect_view()}
                              </div>
                            </div>

                            // ── Chart ─────────────────────────────────────────
                            <div class="rounded-lg border bg-card p-3">
                              <svg
                                viewBox=format!("0 0 {CHART_W} {CHART_H}")
                                class="w-full block"
                                style="height:200px"
                                preserveAspectRatio="none"
                              >
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

                                {move || {
                                  let vis = visible_ch.get();
                                  let session = state.session.get().unwrap();
                                  session.channels.iter().enumerate().filter_map(|(i, ch)| {
                                    if !vis.get(i).copied().unwrap_or(false) { return None; }
                                    let pts = compute_polyline(ch, lap.start_time_s, lap_dur);
                                    if pts.is_empty() { return None; }
                                    let color = COLORS[i % COLORS.len()];
                                    Some(view! {
                                      <polyline
                                        points=pts
                                        fill="none"
                                        stroke=color
                                        stroke-width="1.5"
                                        vector-effect="non-scaling-stroke"
                                      />
                                    })
                                  }).collect_view()
                                }}

                                {move || {
                                  let x = if lap_dur > 0.0 {
                                    (cursor_time.get() / lap_dur) * CHART_W
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
                                <span>{fmt_lap_time(lap_dur * 0.25)}</span>
                                <span>{fmt_lap_time(lap_dur * 0.5)}</span>
                                <span>{fmt_lap_time(lap_dur * 0.75)}</span>
                                <span>{fmt_lap_time(lap_dur)}</span>
                              </div>
                            </div>

                            // ── Scrubber ──────────────────────────────────────
                            <div class="space-y-1">
                              <input
                                type="range"
                                min="0"
                                max=format!("{lap_dur:.2}")
                                step="0.01"
                                class="w-full accent-primary"
                                prop:value=move || format!("{:.2}", cursor_time.get())
                                on:input=move |ev| {
                                  use leptos::wasm_bindgen::JsCast;
                                  if let Some(v) = ev.target()
                                    .and_then(|t| t.dyn_into::<leptos::web_sys::HtmlInputElement>().ok())
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
                                <span>{fmt_lap_time(lap_dur)}</span>
                              </div>
                            </div>

                            // ── Values at cursor ──────────────────────────────
                            <div class="rounded-lg border bg-card p-3">
                              <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-2">
                                "Values at cursor"
                              </p>
                              <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
                                {move || {
                                  let vis = visible_ch.get();
                                  let t = cursor_time.get();
                                  let session = state.session.get().unwrap();
                                  session.channels.iter().enumerate().filter_map(|(i, ch)| {
                                    if !vis.get(i).copied().unwrap_or(false) { return None; }
                                    let color = COLORS[i % COLORS.len()];
                                    let name = if ch.short_name.is_empty() {
                                      ch.name.clone()
                                    } else {
                                      ch.short_name.clone()
                                    };
                                    let unit = ch.unit.clone();
                                    let abs_t = lap.start_time_s + t;
                                    let idx = (abs_t * ch.frequency as f64) as usize;
                                    let val_str = ch.samples.get(idx)
                                      .map(|&v| format!("{v:.2}"))
                                      .unwrap_or_else(|| "—".to_string());
                                    Some(view! {
                                      <div
                                        class="rounded border p-2"
                                        style=format!("border-color:{color}44")
                                      >
                                        <div class="text-xs text-muted-foreground">
                                          {name}
                                          {if unit.is_empty() {
                                            String::new()
                                          } else {
                                            format!(" ({unit})")
                                          }}
                                        </div>
                                        <div
                                          class="font-mono font-semibold tabular-nums text-sm"
                                          style=format!("color:{color}")
                                        >
                                          {val_str}
                                        </div>
                                      </div>
                                    })
                                  }).collect_view()
                                }}
                              </div>
                            </div>

                          </div>
                        }.into_any()
                      }
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
