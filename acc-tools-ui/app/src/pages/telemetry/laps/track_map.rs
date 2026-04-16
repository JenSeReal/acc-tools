use leptos::prelude::*;
use motec_viewer_lib::{LdSession, Point2D, dead_reckoning_slice};

use super::helpers::LAP_COLORS;

#[component]
pub fn TrackMap(
  session: LdSession,
  laps_info: Vec<(usize, f64, f64)>,
  cursor_time: RwSignal<f64>,
) -> impl IntoView {
  let roty_freq = session
    .channels
    .iter()
    .find(|c| c.name == "ROTY")
    .map(|c| c.frequency as f64)
    .unwrap_or(20.0);

  let paths: Vec<(usize, Vec<Point2D>)> = laps_info
    .iter()
    .filter_map(|&(lap_idx, start_s, dur_s)| {
      Some((lap_idx, dead_reckoning_slice(&session, start_s, dur_s)?))
    })
    .collect();

  if paths.is_empty() {
    return ().into_any();
  }

  let (mut xn, mut xx, mut yn, mut yx) = (
    f64::INFINITY,
    f64::NEG_INFINITY,
    f64::INFINITY,
    f64::NEG_INFINITY,
  );
  for (_, pts) in &paths {
    for p in pts {
      xn = xn.min(p.x);
      xx = xx.max(p.x);
      yn = yn.min(p.y);
      yx = yx.max(p.y);
    }
  }
  let extent = (xx - xn).max(yx - yn).max(100.0);
  let pad = extent * 0.06;
  let vb_x = xn - pad;
  let vb_y = -(yx + pad);
  let vb_w = xx - xn + 2.0 * pad;
  let vb_h = yx - yn + 2.0 * pad;
  let viewbox = format!("{vb_x:.1} {vb_y:.1} {vb_w:.1} {vb_h:.1}");
  let stroke_w = extent * 0.006;
  let dot_r = extent * 0.012;

  let polylines: Vec<(usize, String)> = paths
    .iter()
    .map(|(lap_idx, pts)| {
      let s = pts
        .iter()
        .map(|p| format!("{:.1},{:.1}", p.x, -p.y))
        .collect::<Vec<_>>()
        .join(" ");
      (*lap_idx, s)
    })
    .collect();

  // Reference lap info for scrubbing
  let ref_dur_s = laps_info.first().map(|&(_, _, d)| d).unwrap_or(0.0);
  let paths_sv = StoredValue::new(paths);

  let scrub = move |ev: leptos::ev::MouseEvent| {
    use leptos::wasm_bindgen::JsCast;
    if let Some(el) = ev
      .current_target()
      .and_then(|t| t.dyn_into::<web_sys::Element>().ok())
    {
      let rect = el.get_bounding_client_rect();
      let px = ((ev.client_x() as f64 - rect.left()) / rect.width()).clamp(0.0, 1.0);
      let py = ((ev.client_y() as f64 - rect.top()) / rect.height()).clamp(0.0, 1.0);
      // Convert pixel fraction → SVG-space coordinates
      let svgx = vb_x + px * vb_w;
      let svgy = vb_y + py * vb_h;

      // Find the nearest point in the reference (first) lap's path
      if let Some(best_idx) = paths_sv.with_value(|ps| {
        ps.first().and_then(|(_, pts)| {
          pts
            .iter()
            .enumerate()
            .min_by_key(|(_, p)| {
              let dx = p.x - svgx;
              let dy = (-p.y) - svgy;
              ((dx * dx + dy * dy) * 1e6) as i64
            })
            .map(|(i, _)| i)
        })
      }) {
        let t = (best_idx as f64 / roty_freq).min(ref_dur_s);
        cursor_time.set(t);
      }
    }
  };

  view! {
    <div class="rounded-lg border bg-card p-3">
      <p class="text-xs font-semibold text-muted-foreground mb-2">"Track Map"</p>
      <svg
        viewBox=viewbox
        class="w-full block"
        style="max-height:220px;cursor:crosshair"
        preserveAspectRatio="xMidYMid meet"
        on:mousedown=scrub
        on:mousemove=move |ev| { if ev.buttons() != 0 { scrub(ev); } }
      >
        {polylines
          .into_iter()
          .map(|(lap_idx, pts_str)| {
            let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
            view! {
              <polyline
                points=pts_str
                fill="none"
                stroke=color
                stroke-width=stroke_w.to_string()
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            }
          })
          .collect_view()}
        {move || {
          let t = cursor_time.get();
          let idx = (t * roty_freq) as usize;
          paths_sv
            .with_value(|ps| {
              ps.iter()
                .map(|(lap_idx, pts)| {
                  let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                  if let Some(p) = pts.get(idx) {
                    view! {
                      <circle
                        cx=p.x.to_string()
                        cy=(-p.y).to_string()
                        r=dot_r.to_string()
                        fill=color
                      />
                    }
                    .into_any()
                  } else {
                    ().into_any()
                  }
                })
                .collect_view()
            })
        }}
      </svg>
    </div>
  }
  .into_any()
}
