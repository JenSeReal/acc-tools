/// Delta-time graph: shows time gained/lost vs the reference lap at each track distance.
///
/// X-axis = distance through reference lap (shown as reference lap time for readability).
/// Y-axis = Δt in seconds (positive = slower than reference, negative = faster).
use leptos::prelude::*;
use motec_viewer_lib::{LdChannel, LdSession, cumulative_distance};

use super::helpers::{CHART_W, LAP_COLORS, fmt_lap_time};

const DELTA_CHART_H: f64 = 120.0;
const N_POINTS: usize = 400;

// ── Math helpers ───────────────────────────────────────────────────────────────

/// Given a cumulative distance array and a target distance, return the time (seconds)
/// when that distance was reached via linear interpolation.
fn time_at_dist(dist: &[f64], target: f64, dt: f64) -> f64 {
  if dist.is_empty() {
    return 0.0;
  }
  if target <= 0.0 {
    return 0.0;
  }
  let last = *dist.last().unwrap();
  if target >= last {
    return (dist.len() - 1) as f64 * dt;
  }
  let i = dist.partition_point(|&d| d < target);
  if i == 0 {
    return 0.0;
  }
  let d0 = dist[i - 1];
  let d1 = dist[i];
  let t0 = (i - 1) as f64 * dt;
  let t1 = i as f64 * dt;
  if (d1 - d0).abs() < 1e-10 {
    t0
  } else {
    t0 + (target - d0) / (d1 - d0) * (t1 - t0)
  }
}

/// (x_chart, delta_s) pairs for one comparison lap vs reference.
fn delta_points(
  speed_ch: &LdChannel,
  ref_start_s: f64,
  ref_dur_s: f64,
  cmp_start_s: f64,
  cmp_dur_s: f64,
) -> Vec<(f64, f64)> {
  let freq = speed_ch.frequency as f64;
  if freq == 0.0 {
    return vec![];
  }
  let dt = 1.0 / freq;
  let n = speed_ch.samples.len();

  let ref_si = (ref_start_s * freq) as usize;
  let ref_ei = ((ref_start_s + ref_dur_s) * freq).min(n as f64) as usize;
  let cmp_si = (cmp_start_s * freq) as usize;
  let cmp_ei = ((cmp_start_s + cmp_dur_s) * freq).min(n as f64) as usize;

  if ref_si >= ref_ei || cmp_si >= cmp_ei {
    return vec![];
  }

  let ref_speeds = &speed_ch.samples[ref_si..ref_ei];
  let cmp_speeds = &speed_ch.samples[cmp_si..cmp_ei];

  let ref_dist = cumulative_distance(ref_speeds, dt);
  let cmp_dist = cumulative_distance(cmp_speeds, dt);

  let max_dist = *ref_dist.last().unwrap_or(&0.0);
  if max_dist <= 0.0 {
    return vec![];
  }

  (0..=N_POINTS)
    .map(|k| {
      let d = max_dist * k as f64 / N_POINTS as f64;
      let ref_t = time_at_dist(&ref_dist, d, dt);
      let cmp_t = time_at_dist(&cmp_dist, d, dt);
      // x in chart coords: fraction of ref lap duration → chart width
      let x = (ref_t / ref_dur_s) * CHART_W;
      let delta = cmp_t - ref_t; // positive = slower than ref
      (x, delta)
    })
    .collect()
}

fn points_to_svg(pts: &[(f64, f64)], y_min: f64, y_max: f64) -> String {
  let range = (y_max - y_min).max(0.1);
  pts
    .iter()
    .map(|&(x, dy)| {
      let y = DELTA_CHART_H - ((dy - y_min) / range) * DELTA_CHART_H;
      format!("{x:.1},{y:.1}")
    })
    .collect::<Vec<_>>()
    .join(" ")
}

// ── Pre-computation ────────────────────────────────────────────────────────────

#[derive(Clone)]
struct DeltaLine {
  pts_svg: String,
  color: &'static str,
}

fn build_delta_lines(
  session: &LdSession,
  laps_info: &[(usize, f64, f64)],
) -> (Vec<DeltaLine>, f64, f64) {
  if laps_info.len() < 2 {
    return (vec![], -0.5, 0.5);
  }

  let Some(speed_ch) = session.channels.iter().find(|c| c.name == "SPEED") else {
    return (vec![], -0.5, 0.5);
  };

  let (ref_idx, ref_start, ref_dur) = laps_info[0];
  let _ = ref_idx;

  // Compute all delta series first to find global y range
  let all_pts: Vec<(usize, Vec<(f64, f64)>)> = laps_info[1..]
    .iter()
    .map(|&(lap_idx, start_s, dur_s)| {
      let pts = delta_points(speed_ch, ref_start, ref_dur, start_s, dur_s);
      (lap_idx, pts)
    })
    .collect();

  let mut y_min = f64::INFINITY;
  let mut y_max = f64::NEG_INFINITY;
  for (_, pts) in &all_pts {
    for &(_, dy) in pts {
      if dy.is_finite() {
        y_min = y_min.min(dy);
        y_max = y_max.max(dy);
      }
    }
  }
  if !y_min.is_finite() {
    y_min = -0.5;
  }
  if !y_max.is_finite() {
    y_max = 0.5;
  }
  // Symmetric around zero feels natural for delta
  let abs_max = y_min.abs().max(y_max.abs()).max(0.1);
  y_min = -abs_max;
  y_max = abs_max;

  let lines = all_pts
    .into_iter()
    .map(|(lap_idx, pts)| {
      let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
      let pts_svg = points_to_svg(&pts, y_min, y_max);
      DeltaLine { pts_svg, color }
    })
    .collect();

  (lines, y_min, y_max)
}

// ── Component ──────────────────────────────────────────────────────────────────

#[component]
pub fn DeltaGraph(
  session: LdSession,
  laps_info: Vec<(usize, f64, f64)>,
  reference_dur: f64,
  cursor_time: RwSignal<f64>,
) -> impl IntoView {
  if laps_info.len() < 2 {
    return ().into_any();
  }

  let (lines, y_min, y_max) = build_delta_lines(&session, &laps_info);
  let abs_max = y_min.abs().max(y_max.abs());
  let lines = StoredValue::new(lines);

  // Reference lap label
  let ref_lap_idx = laps_info[0].0;
  let ref_label = format!("Lap {}", ref_lap_idx + 1);

  view! {
    <div class="rounded-lg border bg-card p-3">
      <div class="flex items-center justify-between mb-2">
        <p class="text-xs font-semibold text-muted-foreground">
          "Delta Time (vs " {ref_label} ")"
        </p>
        <span class="text-xs text-muted-foreground tabular-nums">
          {format!("±{abs_max:.2}s")}
        </span>
      </div>
      <svg
        viewBox=format!("0 0 {CHART_W} {DELTA_CHART_H}")
        class="w-full block"
        style=format!("height:100px;cursor:crosshair")
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
          if ev.buttons() == 0 { return; }
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
        // Zero line (reference)
        <line
          x1="0" y1={format!("{:.1}", DELTA_CHART_H / 2.0)}
          x2=CHART_W.to_string() y2={format!("{:.1}", DELTA_CHART_H / 2.0)}
          stroke="#475569" stroke-width="1"
        />
        // ±0.25s and ±0.5s guide lines
        {[-0.5f64, -0.25, 0.25, 0.5].iter().map(|&guide| {
          let y = DELTA_CHART_H - ((guide - y_min) / (y_max - y_min)) * DELTA_CHART_H;
          if (0.0..=DELTA_CHART_H).contains(&y) {
            view! {
              <line
                x1="0" y1=format!("{y:.1}")
                x2=CHART_W.to_string() y2=format!("{y:.1}")
                stroke="#1e293b" stroke-width="0.5"
              />
            }.into_any()
          } else {
            ().into_any()
          }
        }).collect_view()}

        // Delta lines
        {lines.with_value(|ls| {
          ls.iter().map(|line| {
            let color = line.color;
            let pts = line.pts_svg.clone();
            view! {
              <polyline
                points=pts
                fill="none"
                stroke=color
                stroke-width="1.5"
                vector-effect="non-scaling-stroke"
              />
            }
          }).collect_view()
        })}

        // Cursor
        {move || {
          let x = if reference_dur > 0.0 {
            (cursor_time.get() / reference_dur) * CHART_W
          } else {
            0.0
          };
          view! {
            <line
              x1=x.to_string() y1="0"
              x2=x.to_string() y2=DELTA_CHART_H.to_string()
              stroke="white" stroke-width="1" opacity="0.8"
            />
          }
        }}
      </svg>
      // Y axis labels
      <div class="flex justify-between text-xs text-muted-foreground tabular-nums mt-1">
        <span>"0:00.000"</span>
        <span class="text-center opacity-60">"faster ↑ / slower ↓"</span>
        <span>{fmt_lap_time(reference_dur)}</span>
      </div>
      <div class="flex items-center gap-1 text-xs text-muted-foreground tabular-nums mt-1">
        <span class="font-mono">{move || format!("T {}", fmt_lap_time(cursor_time.get()))}</span>
      </div>
    </div>
  }
  .into_any()
}
