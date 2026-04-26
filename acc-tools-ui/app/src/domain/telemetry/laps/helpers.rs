use motec_viewer_lib::LdChannel;
pub use motec_viewer_lib::{channel_range, fmt_lap_time};

pub const LAP_COLORS: &[&str] = &[
  "#60a5fa", "#f87171", "#34d399", "#fbbf24", "#a78bfa", "#fb923c", "#38bdf8", "#f472b6",
  "#4ade80", "#e879f9",
];

pub const CHART_W: f64 = 1000.0;
pub const CHART_H: f64 = 100.0;

pub fn polyline_points(
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
