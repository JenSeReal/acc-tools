use leptos::prelude::*;
use motec_viewer_lib::{LdChannel, ms_to_kmh};

use super::helpers::LAP_COLORS;

// ── Colour helpers ─────────────────────────────────────────────────────────────

/// Interpolate between RGBA colour stops based on a normalised value 0..1.
fn gradient_color(t: f64, stops: &[(f64, (u8, u8, u8))]) -> String {
  if stops.is_empty() {
    return "gray".to_string();
  }
  let t = t.clamp(0.0, 1.0);
  if t <= stops[0].0 {
    let c = stops[0].1;
    return format!("rgb({},{},{})", c.0, c.1, c.2);
  }
  if t >= stops.last().unwrap().0 {
    let c = stops.last().unwrap().1;
    return format!("rgb({},{},{})", c.0, c.1, c.2);
  }
  for i in 0..stops.len() - 1 {
    let (t0, c0) = stops[i];
    let (t1, c1) = stops[i + 1];
    if t >= t0 && t <= t1 {
      let f = (t - t0) / (t1 - t0);
      let r = (c0.0 as f64 * (1.0 - f) + c1.0 as f64 * f) as u8;
      let g = (c0.1 as f64 * (1.0 - f) + c1.1 as f64 * f) as u8;
      let b = (c0.2 as f64 * (1.0 - f) + c1.2 as f64 * f) as u8;
      return format!("rgb({r},{g},{b})");
    }
  }
  "gray".to_string()
}

/// Maps tyre air temperature (°C) to a colour: cold=blue, optimal=green, hot=red.
fn tyre_temp_color(temp: f64) -> String {
  // stops: (normalised position 0..1, RGB)
  // Map 50–120°C range to 0..1
  let t = ((temp - 50.0) / 70.0).clamp(0.0, 1.0);
  gradient_color(
    t,
    &[
      (0.0, (59, 130, 246)), // 50°C  cold blue
      (0.43, (34, 197, 94)), // 80°C  optimal green
      (0.64, (234, 179, 8)), // 95°C  warm yellow
      (1.0, (239, 68, 68)),  // 120°C hot red
    ],
  )
}

/// Maps suspension travel (mm, positive = compression) to a colour.
fn susp_travel_color(travel: f64) -> String {
  // Assume 0–60 mm range
  let t = (travel / 60.0).clamp(0.0, 1.0);
  gradient_color(
    t,
    &[
      (0.0, (30, 41, 59)),   // extended – dark slate
      (0.4, (59, 130, 246)), // mid travel – blue
      (0.75, (234, 179, 8)), // high travel – yellow
      (1.0, (239, 68, 68)),  // bump stop – red
    ],
  )
}

// ── Data helpers ───────────────────────────────────────────────────────────────

fn sample_at(ch: &LdChannel, lap_start_s: f64, t: f64) -> Option<f64> {
  ch.sample_at_lap(lap_start_s, t)
}

fn find_channel<'a>(channels: &'a [LdChannel], name: &str) -> Option<&'a LdChannel> {
  channels.iter().find(|c| c.name == name)
}

// ── Steering Wheel ─────────────────────────────────────────────────────────────

#[component]
pub fn SteeringWheel(
  channels: Vec<LdChannel>,
  laps_info: Vec<(usize, f64, f64)>,
  cursor_time: RwSignal<f64>,
) -> impl IntoView {
  let steer_ch = find_channel(&channels, "STEERANGLE").cloned();
  let steer = StoredValue::new(steer_ch);
  let laps = StoredValue::new(laps_info);

  view! {
    <div class="rounded-lg border bg-card p-3 flex flex-col items-center gap-2">
      <p class="text-xs font-semibold text-muted-foreground self-start">"Steering"</p>
      <svg viewBox="-58 -58 116 116" class="w-full" style="max-height:160px">
        // ── Static GT3 wheel structure (dark base) ──
        // D-shaped rim: arc over the top, flat bottom section
        <path
          d="M 30,42 A 50,50 0 1 1 -30,42"
          fill="none" stroke="#1e293b" stroke-width="12" stroke-linecap="round"
        />
        // Flat bottom grip bar
        <line x1="-30" y1="42" x2="30" y2="42" stroke="#1e293b" stroke-width="12" stroke-linecap="round" />
        // Grip texture accents (subtle lighter strips on flat section)
        <line x1="-20" y1="42" x2="-12" y2="42" stroke="#334155" stroke-width="4" stroke-linecap="round" />
        <line x1="-4" y1="42" x2="4" y2="42" stroke="#334155" stroke-width="4" stroke-linecap="round" />
        <line x1="12" y1="42" x2="20" y2="42" stroke="#334155" stroke-width="4" stroke-linecap="round" />
        // Upper-left spoke (dark)
        <line x1="-11" y1="-6" x2="-37" y2="-27" stroke="#1e293b" stroke-width="8" stroke-linecap="round" />
        // Upper-right spoke (dark)
        <line x1="11" y1="-6" x2="37" y2="-27" stroke="#1e293b" stroke-width="8" stroke-linecap="round" />
        // Bottom spoke (dark)
        <line x1="0" y1="14" x2="0" y2="42" stroke="#1e293b" stroke-width="8" stroke-linecap="round" />
        // Center hub background
        <rect x="-15" y="-14" width="30" height="30" rx="5" fill="#1e293b" />

        // ── Rotating colored wheel per lap ──
        {move || {
          let t = cursor_time.get();
          steer.with_value(|s| {
            laps.with_value(|ls| {
              ls.iter().map(|&(lap_idx, start_s, _)| {
                let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                let deg = s.as_ref()
                  .and_then(|ch| sample_at(ch, start_s, t))
                  .unwrap_or(0.0)
                  .clamp(-450.0, 450.0);
                // Map ±450° physical → ±180° visual; negate: ACC positive = left = CCW
                let visual = -deg * 180.0 / 450.0;
                let opacity = if lap_idx == ls.first().map(|&(i,_,_)| i).unwrap_or(0) {
                  "0.95"
                } else {
                  "0.55"
                };
                view! {
                  <g transform=format!("rotate({visual:.1})") opacity=opacity>
                    // Rim arc (colored)
                    <path
                      d="M 30,42 A 50,50 0 1 1 -30,42"
                      fill="none" stroke=color stroke-width="7" stroke-linecap="round"
                    />
                    // Flat bottom
                    <line x1="-30" y1="42" x2="30" y2="42"
                      stroke=color stroke-width="7" stroke-linecap="round" />
                    // Upper-left spoke
                    <line x1="-10" y1="-5" x2="-36" y2="-26"
                      stroke=color stroke-width="4.5" stroke-linecap="round" />
                    // Upper-right spoke
                    <line x1="10" y1="-5" x2="36" y2="-26"
                      stroke=color stroke-width="4.5" stroke-linecap="round" />
                    // Bottom spoke
                    <line x1="0" y1="13" x2="0" y2="41"
                      stroke=color stroke-width="4.5" stroke-linecap="round" />
                  </g>
                }
              }).collect_view()
            })
          })
        }}

        // ── Center hub (stays static on top) ──
        <rect x="-14" y="-13" width="28" height="28" rx="4" fill="#0f172a" stroke="#475569" stroke-width="1.5" />
        // Hub emblem dot
        <circle cx="0" cy="1" r="5" fill="#334155" stroke="#64748b" stroke-width="1" />
        <circle cx="0" cy="1" r="2" fill="#ef4444" opacity="0.7" />

        // ── Button panels (static decorations on grip sections) ──
        // Left button cluster
        <rect x="-52" y="-9" width="10" height="5" rx="1.5" fill="#0f172a" stroke="#334155" stroke-width="0.8" />
        <rect x="-52" y="-2" width="10" height="5" rx="1.5" fill="#0f172a" stroke="#334155" stroke-width="0.8" />
        <rect x="-52" y="5" width="10" height="5" rx="1.5" fill="#0f172a" stroke="#334155" stroke-width="0.8" />
        // Right button cluster
        <rect x="42" y="-9" width="10" height="5" rx="1.5" fill="#0f172a" stroke="#334155" stroke-width="0.8" />
        <rect x="42" y="-2" width="10" height="5" rx="1.5" fill="#0f172a" stroke="#334155" stroke-width="0.8" />
        <rect x="42" y="5" width="10" height="5" rx="1.5" fill="#0f172a" stroke="#334155" stroke-width="0.8" />
      </svg>
      <div class="flex gap-2 flex-wrap justify-center">
        {move || {
          let t = cursor_time.get();
          steer.with_value(|s| {
            laps.with_value(|ls| {
              ls.iter().map(|&(lap_idx, start_s, _)| {
                let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                let val = s.as_ref()
                  .and_then(|ch| sample_at(ch, start_s, t))
                  .map(|v| format!("{v:+.1}°"))
                  .unwrap_or_else(|| "—".to_string());
                view! {
                  <span class="text-xs tabular-nums font-mono" style=format!("color:{color}")>
                    {val}
                  </span>
                }
              }).collect_view()
            })
          })
        }}
      </div>
    </div>
  }
}

// ── Pedal Bars ─────────────────────────────────────────────────────────────────

#[component]
pub fn PedalBars(
  channels: Vec<LdChannel>,
  laps_info: Vec<(usize, f64, f64)>,
  cursor_time: RwSignal<f64>,
) -> impl IntoView {
  let throttle = StoredValue::new(find_channel(&channels, "THROTTLE").cloned());
  let brake = StoredValue::new(find_channel(&channels, "BRAKE").cloned());
  let clutch = StoredValue::new(find_channel(&channels, "CLUTCH").cloned());
  let laps = StoredValue::new(laps_info);

  // Show first selected lap in the bars; show all lap values as text
  let bars = [
    ("Thr", "#34d399", throttle),
    ("Brk", "#f87171", brake),
    ("Clt", "#fbbf24", clutch),
  ];

  view! {
    <div class="rounded-lg border bg-card p-3">
      <p class="text-xs font-semibold text-muted-foreground mb-3">"Pedals"</p>
      <div class="flex gap-3 justify-around">
        {bars.into_iter().map(|(label, bar_color, ch_sv)| {
          view! {
            <div class="flex flex-col items-center gap-1">
              // Bar
              <div
                class="relative rounded overflow-hidden"
                style="width:26px;height:90px;background:#1e293b"
              >
                {move || {
                  let t = cursor_time.get();
                  laps.with_value(|ls| {
                    let pct = ls.first().and_then(|&(_, start_s, _)| {
                      ch_sv.with_value(|ch| {
                        ch.as_ref().and_then(|c| sample_at(c, start_s, t))
                      })
                    }).unwrap_or(0.0).clamp(0.0, 100.0);
                    view! {
                      <div
                        style=format!(
                          "position:absolute;bottom:0;left:0;right:0;height:{pct:.1}%;background:{bar_color}"
                        )
                      />
                    }
                  })
                }}
              </div>
              <span class="text-xs text-muted-foreground">{label}</span>
              // Per-lap values
              {move || {
                let t = cursor_time.get();
                laps.with_value(|ls| {
                  ls.iter().map(|&(lap_idx, start_s, _)| {
                    let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                    let val = ch_sv.with_value(|ch| {
                      ch.as_ref()
                        .and_then(|c| sample_at(c, start_s, t))
                        .map(|v| format!("{v:.0}%"))
                        .unwrap_or_else(|| "—".to_string())
                    });
                    view! {
                      <span
                        class="text-xs tabular-nums font-mono"
                        style=format!("color:{color}")
                      >
                        {val}
                      </span>
                    }
                  }).collect_view()
                })
              }}
            </div>
          }
        }).collect_view()}
      </div>
    </div>
  }
}

// ── 4-corner tyre / brake widget ───────────────────────────────────────────────

#[component]
pub fn FourCornerWidget(
  channels: Vec<LdChannel>,
  laps_info: Vec<(usize, f64, f64)>,
  cursor_time: RwSignal<f64>,
  channel_prefix: &'static str,
  title: &'static str,
  unit: &'static str,
  #[prop(default = 1)] decimals: usize,
  /// Multiply raw channel value before display (e.g. PSI → kPa).
  #[prop(default = 1.0)]
  scale: f64,
) -> impl IntoView {
  let corners: [(&'static str, String); 4] = [
    ("LF", format!("{channel_prefix}_LF")),
    ("RF", format!("{channel_prefix}_RF")),
    ("LR", format!("{channel_prefix}_LR")),
    ("RR", format!("{channel_prefix}_RR")),
  ];

  // Pre-extract channels for each corner
  let corner_chs: Vec<(&'static str, Option<LdChannel>)> = corners
    .iter()
    .map(|(label, name)| (*label, find_channel(&channels, name).cloned()))
    .collect();

  let laps = StoredValue::new(laps_info);
  let chs = StoredValue::new(corner_chs);

  view! {
    <div class="rounded-lg border bg-card p-3">
      <p class="text-xs font-semibold text-muted-foreground mb-2">{title}</p>
      // 2×2 grid mirroring car corners
      <div class="grid grid-cols-2 gap-2">
        {move || {
          let t = cursor_time.get();
          chs.with_value(|corners| {
            laps.with_value(|ls| {
              corners.iter().map(|(corner_label, ch)| {
                let vals: Vec<_> = ls.iter().map(|&(lap_idx, start_s, _)| {
                  let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                  let v = ch.as_ref()
                    .and_then(|c| sample_at(c, start_s, t))
                    .map(|v| format!("{:.decimals$}", v * scale))
                    .unwrap_or_else(|| "—".to_string());
                  (color, v)
                }).collect();

                view! {
                  <div class="rounded border border-border/50 p-2 text-center">
                    <div class="text-xs text-muted-foreground mb-1">{*corner_label}</div>
                    {vals.into_iter().map(|(color, val)| view! {
                      <div
                        class="font-mono font-semibold text-sm tabular-nums"
                        style=format!("color:{color}")
                      >{val}</div>
                    }).collect_view()}
                    <div class="text-xs text-muted-foreground/50">{unit}</div>
                  </div>
                }
              }).collect_view()
            })
          })
        }}
      </div>
    </div>
  }
}

// ── HUD Widget ─────────────────────────────────────────────────────────────────

fn fmt_gear(g: f64) -> String {
  match g.round() as i32 {
    n if n < 0 => "R".to_string(),
    0 => "N".to_string(),
    n => n.to_string(),
  }
}

#[component]
pub fn HudWidget(
  inputs_channels: Vec<LdChannel>,
  motion_channels: Vec<LdChannel>,
  engine_channels: Vec<LdChannel>,
  laps_info: Vec<(usize, f64, f64)>,
  cursor_time: RwSignal<f64>,
) -> impl IntoView {
  let throttle = StoredValue::new(find_channel(&inputs_channels, "THROTTLE").cloned());
  let brake = StoredValue::new(find_channel(&inputs_channels, "BRAKE").cloned());
  let clutch = StoredValue::new(find_channel(&inputs_channels, "CLUTCH").cloned());
  let steer = StoredValue::new(find_channel(&inputs_channels, "STEERANGLE").cloned());
  let speed = StoredValue::new(find_channel(&motion_channels, "SPEED").cloned());
  let gear = StoredValue::new(find_channel(&engine_channels, "GEAR").cloned());
  let rpm = StoredValue::new(find_channel(&engine_channels, "RPMS").cloned());
  let tc_ch = StoredValue::new(find_channel(&engine_channels, "TC").cloned());
  let abs_ch = StoredValue::new(find_channel(&engine_channels, "ABS").cloned());
  let laps = StoredValue::new(laps_info);

  view! {
    <div class="rounded-lg border p-4" style="background:#0d1117;border-color:#1e2937">
      <div class="flex gap-4 items-stretch" style="min-height:190px">

        // ── Left: pedal bars ──────────────────────────────────────────────────
        <div class="flex gap-1.5 items-end" style="padding-bottom:26px">
          {[
            ("G", "#22d3ee", throttle),
            ("B", "#f87171", brake),
            ("C", "#fbbf24", clutch),
          ].into_iter().map(|(label, color, ch_sv)| {
            view! {
              <div class="flex flex-col items-center gap-1">
                <div class="relative rounded-sm overflow-hidden"
                  style="width:14px;height:110px;background:#1c2433"
                >
                  {move || {
                    let t = cursor_time.get();
                    laps.with_value(|ls| {
                      let pct = ls.first()
                        .and_then(|&(_, start_s, _)| {
                          ch_sv.with_value(|ch| ch.as_ref().and_then(|c| sample_at(c, start_s, t)))
                        })
                        .unwrap_or(0.0)
                        .clamp(0.0, 100.0);
                      view! {
                        <div style=format!(
                          "position:absolute;bottom:0;left:0;right:0;height:{pct:.1}%;background:{color}"
                        ) />
                      }
                    })
                  }}
                </div>
                <span class="text-xs font-bold" style=format!("color:{color}")>{label}</span>
              </div>
            }
          }).collect_view()}
        </div>

        // ── Center: steer → speed → gear → TC/ABS ────────────────────────────
        <div class="flex flex-col items-center justify-between flex-1 py-1">
          // Steering lock
          <div class="text-xs tabular-nums font-mono" style="color:#64748b">
            {move || {
              let t = cursor_time.get();
              steer.with_value(|s| {
                laps.with_value(|ls| {
                  ls.first()
                    .and_then(|&(_, start_s, _)| s.as_ref().and_then(|ch| sample_at(ch, start_s, t)))
                    .map(|v| format!("{v:+.1}°"))
                    .unwrap_or_else(|| "—".to_string())
                })
              })
            }}
          </div>

          // Speed
          <div class="text-center">
            <div class="text-3xl font-bold tabular-nums font-mono leading-none" style="color:#f1f5f9">
              {move || {
                let t = cursor_time.get();
                laps.with_value(|ls| {
                  speed.with_value(|s| {
                    ls.first()
                      .and_then(|&(_, start_s, _)| s.as_ref().and_then(|ch| sample_at(ch, start_s, t)))
                      .map(|v| format!("{:.0}", ms_to_kmh(v)))
                      .unwrap_or_else(|| "0".to_string())
                  })
                })
              }}
            </div>
            <div class="text-xs font-mono" style="color:#64748b">"km/h"</div>
          </div>

          // Gear
          <div class="font-black leading-none tabular-nums font-mono" style="font-size:5rem;color:#ffffff">
            {move || {
              let t = cursor_time.get();
              laps.with_value(|ls| {
                gear.with_value(|g| {
                  ls.first()
                    .and_then(|&(_, start_s, _)| g.as_ref().and_then(|ch| sample_at(ch, start_s, t)))
                    .map(fmt_gear)
                    .unwrap_or_else(|| "N".to_string())
                })
              })
            }}
          </div>

          // TC / ABS indicators
          <div class="flex gap-2">
            {move || {
              let t = cursor_time.get();
              let active = laps.with_value(|ls| {
                ls.first()
                  .and_then(|&(_, start_s, _)| {
                    tc_ch.with_value(|c| c.as_ref().and_then(|ch| sample_at(ch, start_s, t)))
                  })
                  .unwrap_or(0.0) > 0.0
              });
              let style = if active {
                "padding:2px 10px;border-radius:4px;font-size:11px;font-weight:700;color:#22d3ee;background:#0e3a47;border:1px solid #22d3ee55"
              } else {
                "padding:2px 10px;border-radius:4px;font-size:11px;font-weight:700;color:#334155;background:#1c2433;border:1px solid #1e293b"
              };
              view! { <span style=style>"TC"</span> }
            }}
            {move || {
              let t = cursor_time.get();
              let active = laps.with_value(|ls| {
                ls.first()
                  .and_then(|&(_, start_s, _)| {
                    abs_ch.with_value(|c| c.as_ref().and_then(|ch| sample_at(ch, start_s, t)))
                  })
                  .unwrap_or(0.0) > 0.0
              });
              let style = if active {
                "padding:2px 10px;border-radius:4px;font-size:11px;font-weight:700;color:#fbbf24;background:#3a2e0e;border:1px solid #fbbf2455"
              } else {
                "padding:2px 10px;border-radius:4px;font-size:11px;font-weight:700;color:#334155;background:#1c2433;border:1px solid #1e293b"
              };
              view! { <span style=style>"ABS"</span> }
            }}
          </div>
        </div>

        // ── Right: RPM value + vertical bar ──────────────────────────────────
        <div class="flex items-center gap-2">
          <div class="text-right">
            {move || {
              let t = cursor_time.get();
              laps.with_value(|ls| {
                ls.iter().map(|&(lap_idx, start_s, _)| {
                  let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                  let val = rpm.with_value(|r| {
                    r.as_ref()
                      .and_then(|ch| sample_at(ch, start_s, t))
                      .map(|v| format!("{:.0}", v))
                      .unwrap_or_else(|| "—".to_string())
                  });
                  view! {
                    <div class="text-xl font-bold tabular-nums font-mono leading-tight"
                      style=format!("color:{color}")>{val}</div>
                  }
                }).collect_view()
              })
            }}
            <div class="text-xs font-mono" style="color:#64748b">"rpm"</div>
          </div>
          // RPM vertical bar
          <div class="relative rounded-sm overflow-hidden flex-shrink-0"
            style="width:16px;height:140px;background:#1c2433"
          >
            {move || {
              let t = cursor_time.get();
              let rpm_val = laps.with_value(|ls| {
                ls.first()
                  .and_then(|&(_, start_s, _)| {
                    rpm.with_value(|r| r.as_ref().and_then(|ch| sample_at(ch, start_s, t)))
                  })
                  .unwrap_or(0.0)
              });
              let pct = (rpm_val / 9000.0).clamp(0.0, 1.0) * 100.0;
              let color = if pct > 88.0 { "#ef4444" } else if pct > 70.0 { "#f59e0b" } else { "#22d3ee" };
              view! {
                <div style=format!(
                  "position:absolute;bottom:0;left:0;right:0;height:{pct:.1}%;background:{color}"
                ) />
              }
            }}
          </div>
        </div>

      </div>
    </div>
  }
}

// ── Car top-down SVG outline helper ────────────────────────────────────────────
//
// viewBox "0 0 160 240"
// Tyre positions (x, y, w, h):  LF(2,28,23,55)  RF(135,28,23,55)
//                                LR(2,157,23,55)  RR(135,157,23,55)

fn car_outline_svg() -> impl IntoView {
  view! {
    // Car body
    <rect x="27" y="18" width="106" height="204" rx="14" fill="#1e293b" stroke="#334155" stroke-width="1.5" />
    // Front bumper accent
    <rect x="45" y="18" width="70" height="8" rx="3" fill="#334155" />
    // Rear bumper accent
    <rect x="45" y="214" width="70" height="8" rx="3" fill="#334155" />
    // Windscreen (front)
    <rect x="42" y="36" width="76" height="38" rx="6" fill="#0f172a" opacity="0.8" />
    // Cockpit area
    <rect x="50" y="90" width="60" height="60" rx="4" fill="#0f172a" opacity="0.5" />
    // Rear window
    <rect x="42" y="166" width="76" height="28" rx="6" fill="#0f172a" opacity="0.8" />
    // Front direction arrow
    <polyline points="80,12 74,22 80,18 86,22 80,12" fill="#64748b" />
  }
}

// ── Suspension SVG widget ──────────────────────────────────────────────────────

#[component]
pub fn SuspensionSvg(
  channels: Vec<LdChannel>,
  laps_info: Vec<(usize, f64, f64)>,
  cursor_time: RwSignal<f64>,
) -> impl IntoView {
  let chs: [(&'static str, Option<LdChannel>); 4] = [
    (
      "SUS_TRAVEL_LF",
      find_channel(&channels, "SUS_TRAVEL_LF").cloned(),
    ),
    (
      "SUS_TRAVEL_RF",
      find_channel(&channels, "SUS_TRAVEL_RF").cloned(),
    ),
    (
      "SUS_TRAVEL_LR",
      find_channel(&channels, "SUS_TRAVEL_LR").cloned(),
    ),
    (
      "SUS_TRAVEL_RR",
      find_channel(&channels, "SUS_TRAVEL_RR").cloned(),
    ),
  ];
  let chs_sv = StoredValue::new(chs);
  let laps_sv = StoredValue::new(laps_info);

  // Corner layout: (index, label, rx, ry, rw, rh)
  let layout: [(usize, &'static str, f64, f64, f64, f64); 4] = [
    (0, "LF", 2.0, 28.0, 23.0, 55.0),
    (1, "RF", 135.0, 28.0, 23.0, 55.0),
    (2, "LR", 2.0, 157.0, 23.0, 55.0),
    (3, "RR", 135.0, 157.0, 23.0, 55.0),
  ];

  view! {
    <div class="rounded-lg border bg-card p-3">
      <p class="text-xs font-semibold text-muted-foreground mb-2">"Suspension Travel"</p>
      <svg viewBox="0 0 160 240" class="w-full" style="max-height:200px" preserveAspectRatio="xMidYMid meet">
        {car_outline_svg()}
        // Suspension bars per corner
        {move || {
          let t = cursor_time.get();
          chs_sv.with_value(|chs| {
            laps_sv.with_value(|ls| {
              let ref_start = ls.first().map(|&(_, s, _)| s).unwrap_or(0.0);

              layout.iter().map(|&(ci, label, rx, ry, rw, rh)| {
                let travel = chs[ci].1.as_ref()
                  .and_then(|ch| sample_at(ch, ref_start, t))
                  .unwrap_or(0.0)
                  .abs(); // ACC may use negative for compression
                let fill_color = susp_travel_color(travel);
                // Normalize travel 0–60mm to bar fill 0..rh
                let fill_h = (travel / 60.0).clamp(0.0, 1.0) * rh;
                let fill_y = ry + rh - fill_h;
                let cx = rx + rw / 2.0;

                // Per-lap value texts
                let per_lap: Vec<_> = ls.iter().enumerate().map(|(li, &(lap_idx, start_s, _))| {
                  let v = chs[ci].1.as_ref()
                    .and_then(|ch| sample_at(ch, start_s, t))
                    .map(|v| format!("{:.0}", v.abs()))
                    .unwrap_or_else(|| "—".to_string());
                  let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                  let ty = ry + 18.0 + li as f64 * 13.0;
                  (cx, ty, color, v)
                }).collect();

                view! {
                  // Background bar
                  <rect
                    x=rx.to_string() y=ry.to_string()
                    width=rw.to_string() height=rh.to_string()
                    rx="4" fill="#0f172a" stroke="#334155" stroke-width="1"
                  />
                  // Fill bar (from bottom)
                  <rect
                    x=rx.to_string() y=fill_y.to_string()
                    width=rw.to_string() height=fill_h.to_string()
                    rx="3" fill=fill_color opacity="0.85"
                  />
                  // Corner label
                  <text
                    x=cx.to_string() y=(ry + 9.0).to_string()
                    text-anchor="middle" font-size="7" fill="white" font-weight="bold"
                  >{label}</text>
                  // Per-lap values
                  {per_lap.into_iter().map(|(tx, ty, color, v)| view! {
                    <text
                      x=tx.to_string() y=ty.to_string()
                      text-anchor="middle" font-size="7.5" fill=color font-family="monospace"
                    >{v}</text>
                  }).collect_view()}
                }
              }).collect_view()
            })
          })
        }}
      </svg>
      <div class="flex gap-2 text-xs text-muted-foreground mt-1 justify-center">
        <span style="color:#334155">"extended"</span>
        <span style="color:#3b82f6">"mid"</span>
        <span style="color:#eab308">"high"</span>
        <span style="color:#ef4444">"bump stop"</span>
      </div>
    </div>
  }
}

// ── Tyre Panel ─────────────────────────────────────────────────────────────────

fn brake_temp_color(temp: f64) -> String {
  let t = ((temp - 50.0) / 750.0).clamp(0.0, 1.0);
  gradient_color(
    t,
    &[
      (0.0, (30, 41, 59)),   // cold  – slate
      (0.2, (59, 130, 246)), // 200°C – blue
      (0.47, (34, 197, 94)), // 400°C – green
      (0.67, (234, 179, 8)), // 550°C – yellow
      (1.0, (239, 68, 68)),  // 800°C – red
    ],
  )
}

/// Single corner cell used in tyre/brake grids.
#[allow(clippy::too_many_arguments)]
fn corner_cell(
  label: &'static str,
  chs: StoredValue<Vec<LdChannel>>,
  laps: StoredValue<Vec<(usize, f64, f64)>>,
  cursor_time: RwSignal<f64>,
  ch_name: String,
  ch_name2: Option<String>,
  fmt: fn(f64) -> String,
  color_fn: fn(f64) -> String,
  unit: &'static str,
) -> impl IntoView {
  let ch1 = StoredValue::new(chs.with_value(|cs| find_channel(cs, &ch_name).cloned()));
  let ch2 =
    StoredValue::new(ch_name2.and_then(|n| chs.with_value(|cs| find_channel(cs, &n).cloned())));

  view! {
    <div class="rounded-lg border border-border/60 bg-card p-3 text-center">
      <div class="text-xs font-semibold text-muted-foreground mb-2">{label}</div>
      {move || {
        let t = cursor_time.get();
        laps.with_value(|ls| {
          ls.iter().map(|&(lap_idx, start_s, _)| {
            let color_ref = LAP_COLORS[lap_idx % LAP_COLORS.len()];
            // First value (large, colored by data)
            let v1 = ch1.with_value(|c| {
              c.as_ref().and_then(|ch| sample_at(ch, start_s, t))
            });
            let display = v1.map(fmt).unwrap_or_else(|| "—".to_string());
            let data_color = v1.map(color_fn).unwrap_or_else(|| color_ref.to_string());
            // Second value (pressure only, shown smaller)
            let v2_str = ch2.with_value(|c| {
              c.as_ref()
                .and_then(|ch| sample_at(ch, start_s, t))
                .map(fmt)
            });
            view! {
              <div>
                <div class="font-mono font-bold text-2xl tabular-nums leading-tight"
                  style=format!("color:{data_color}")>{display}</div>
                {v2_str.map(|s| view! {
                  <div class="font-mono text-sm tabular-nums leading-tight"
                    style=format!("color:{color_ref}")>{s}</div>
                })}
              </div>
            }
          }).collect_view()
        })
      }}
      <div class="text-xs text-muted-foreground mt-1">{unit}</div>
    </div>
  }
}

#[component]
pub fn TyrePanel(
  /// All channels (tyres + brakes merged).
  channels: Vec<LdChannel>,
  laps_info: Vec<(usize, f64, f64)>,
  cursor_time: RwSignal<f64>,
) -> impl IntoView {
  let chs = StoredValue::new(channels);
  let laps = StoredValue::new(laps_info);

  // Helper: build a 2×2 grid of corner cells
  let grid = move |title: &'static str,
                   prefix: &'static str,
                   prefix2: Option<&'static str>,
                   fmt: fn(f64) -> String,
                   color_fn: fn(f64) -> String,
                   unit: &'static str| {
    let corners = ["LF", "RF", "LR", "RR"];
    view! {
      <div class="rounded-lg border bg-card p-3">
        <p class="text-xs font-semibold text-muted-foreground mb-3">{title}</p>
        <div class="grid grid-cols-2 gap-2">
          {corners.into_iter().map(|c| {
            let ch_name  = format!("{prefix}_{c}");
            let ch_name2 = prefix2.map(|p| format!("{p}_{c}"));
            corner_cell(c, chs, laps, cursor_time, ch_name, ch_name2, fmt, color_fn, unit)
          }).collect_view()}
        </div>
      </div>
    }
  };

  view! {
    <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
      // Tyre Temperatures (with pressure as secondary value)
      {grid(
        "Tyre Temperatures",
        "TYRE_TAIR",
        Some("TYRE_PRESS"),
        |v| format!("{v:.0}°"),
        tyre_temp_color,
        "°C",
      )}
      // Brake Temperatures
      {grid(
        "Brake Temperatures",
        "BRAKE_TEMP",
        None,
        |v| format!("{v:.0}°"),
        brake_temp_color,
        "°C",
      )}
    </div>
  }
}
