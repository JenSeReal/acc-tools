use leptos::prelude::*;
use motec_viewer_lib::LdChannel;

use super::helpers::{CHART_H, CHART_W, LAP_COLORS, channel_range, fmt_lap_time, polyline_points};

pub struct GroupDef {
  pub title: &'static str,
  /// Channel names that belong to this group (exact match or prefix).
  pub names: &'static [&'static str],
}

pub const CHANNEL_GROUPS: &[GroupDef] = &[
  GroupDef {
    title: "Driver Inputs",
    names: &["THROTTLE", "BRAKE", "CLUTCH", "STEERANGLE"],
  },
  GroupDef {
    title: "Motion & Speed",
    names: &["SPEED", "G_LAT", "G_LON", "ROTY"],
  },
  GroupDef {
    title: "Engine",
    names: &["RPMS", "GEAR", "TC", "ABS"],
  },
  GroupDef {
    title: "Tyres",
    names: &["TYRE_PRESS_", "TYRE_TAIR_", "WHEEL_SPEED_"],
  },
  GroupDef {
    title: "Brake Temperatures",
    names: &["BRAKE_TEMP_"],
  },
  GroupDef {
    title: "Suspension",
    names: &["SUS_TRAVEL_", "BUMPSTOP"],
  },
  GroupDef {
    title: "Electronics",
    names: &["EN_"],
  },
];

/// Returns the index into `CHANNEL_GROUPS` for the given channel name,
/// or `None` if it belongs to the "Other" bucket.
pub fn channel_group_index(name: &str) -> Option<usize> {
  CHANNEL_GROUPS.iter().position(|g| {
    g.names.iter().any(|&pat| {
      if pat.ends_with('_') {
        // prefix pattern: "BRAKE_TEMP_" matches "BRAKE_TEMP_LF" but not "BRAKE"
        name.starts_with(pat)
      } else {
        // exact match: "BRAKE" matches only "BRAKE"
        name == pat
      }
    })
  })
}

// ── Pre-computed chart data ────────────────────────────────────────────────────

/// One channel's worth of display data: label + per-lap polyline strings.
#[derive(Clone)]
pub struct ChartData {
  pub label: String,
  /// (polyline_points_string, color)
  pub lines: Vec<(String, &'static str)>,
}

pub fn build_charts(
  channels: &[LdChannel],
  laps_info: &[(usize, f64, f64)],
  reference_dur: f64,
) -> Vec<ChartData> {
  let slices: Vec<(f64, f64)> = laps_info.iter().map(|&(_, s, d)| (s, d)).collect();
  channels
    .iter()
    .map(|ch| {
      let label = {
        let name = if ch.short_name.is_empty() {
          &ch.name
        } else {
          &ch.short_name
        };
        if ch.unit.is_empty() {
          name.clone()
        } else {
          format!("{name} ({})", ch.unit)
        }
      };
      let (gmin, gmax) = channel_range(ch, &slices);
      let lines = laps_info
        .iter()
        .map(|&(lap_idx, start_s, dur_s)| {
          let pts = polyline_points(ch, start_s, dur_s, gmin, gmax, reference_dur);
          let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
          (pts, color)
        })
        .collect();
      ChartData { label, lines }
    })
    .collect()
}

// ── Shared interactive SVG chart ───────────────────────────────────────────────

/// Renders a single polyline chart with an interactive cursor line.
/// Moving the mouse over any chart updates `cursor_time` for all charts.
#[component]
pub fn InteractiveChart(
  chart: ChartData,
  reference_dur: f64,
  cursor_time: RwSignal<f64>,
  #[prop(default = 80)] height_px: u32,
) -> impl IntoView {
  let label = chart.label.clone();
  let lines = chart.lines.clone();

  view! {
    <div>
      <p class="text-xs text-muted-foreground mb-1">{label}</p>
      <svg
        viewBox=format!("0 0 {CHART_W} {CHART_H}")
        class="w-full block"
        style=format!("height:{height_px}px;cursor:crosshair")
        preserveAspectRatio="none"
        on:mousedown=move |ev: leptos::ev::MouseEvent| {
          use leptos::wasm_bindgen::JsCast;
          if reference_dur <= 0.0 { return; }
          if let Some(el) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
          {
            let rect = el.get_bounding_client_rect();
            let frac = ((ev.client_x() as f64 - rect.left()) / rect.width()).clamp(0.0, 1.0);
            cursor_time.set(frac * reference_dur);
          }
        }
        on:mousemove=move |ev: leptos::ev::MouseEvent| {
          if ev.buttons() == 0 { return; } // only while dragging
          use leptos::wasm_bindgen::JsCast;
          if reference_dur <= 0.0 { return; }
          if let Some(el) = ev
            .current_target()
            .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
          {
            let rect = el.get_bounding_client_rect();
            let frac = ((ev.client_x() as f64 - rect.left()) / rect.width()).clamp(0.0, 1.0);
            cursor_time.set(frac * reference_dur);
          }
        }
      >
        // Grid lines
        {(1..4u8)
          .map(|i| {
            let y = CHART_H * i as f64 / 4.0;
            view! {
              <line
                x1="0"
                y1=y.to_string()
                x2=CHART_W.to_string()
                y2=y.to_string()
                stroke="#334155"
                stroke-width="0.5"
              />
            }
          })
          .collect_view()}

        // Data lines
        {lines
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

        // Cursor line (reactive)
        {move || {
          let x = if reference_dur > 0.0 {
            (cursor_time.get() / reference_dur) * CHART_W
          } else {
            0.0
          };
          view! {
            <line
              x1=x.to_string()
              y1="0"
              x2=x.to_string()
              y2=CHART_H.to_string()
              stroke="white"
              stroke-width="1"
              opacity="0.8"
            />
          }
        }}
      </svg>
    </div>
  }
}

// ── Component ─────────────────────────────────────────────────────────────────

#[component]
pub fn ChannelGroupCard(
  title: &'static str,
  channels: Vec<LdChannel>,
  laps_info: Vec<(usize, f64, f64)>,
  reference_dur: f64,
  cursor_time: RwSignal<f64>,
  #[prop(default = true)] default_open: bool,
) -> impl IntoView {
  if channels.is_empty() {
    return ().into_any();
  }

  let open = RwSignal::new(default_open);
  let charts = StoredValue::new(build_charts(&channels, &laps_info, reference_dur));

  view! {
    <div class="rounded-lg border bg-card">
      <button
        class="w-full flex items-center justify-between px-3 py-2 text-sm font-semibold text-left hover:bg-accent/50 transition-colors rounded-lg"
        on:click=move |_| open.update(|v| *v = !*v)
      >
        <span>{title}</span>
        <span class="text-muted-foreground text-xs select-none">
          {move || if open.get() { "▲" } else { "▼" }}
        </span>
      </button>

      <Show when=move || open.get()>
        <div class="px-3 pb-3 space-y-3 border-t border-border/50 pt-2">
          {charts.with_value(|charts| {
            charts
              .iter()
              .map(|chart| {
                view! {
                  <InteractiveChart
                    chart=chart.clone()
                    reference_dur
                    cursor_time
                  />
                }
              })
              .collect_view()
          })}
          <div class="flex justify-between text-xs text-muted-foreground tabular-nums">
            <span>"0:00.000"</span>
            <span>{fmt_lap_time(reference_dur)}</span>
          </div>
        </div>
      </Show>
    </div>
  }
  .into_any()
}
