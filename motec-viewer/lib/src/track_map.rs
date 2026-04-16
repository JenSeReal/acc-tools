//! Dead-reckoning track map from SPEED + ROTY channels.
//!
//! Integration step (at the ROTY sample rate):
//!   heading += ROTY[i] * dt                       (rad)
//!   x       += SPEED[i] * cos(heading) * dt       (metres)
//!   y       += SPEED[i] * sin(heading) * dt       (metres)
//!
//! SPEED is in m/s. It is averaged down to the ROTY sample rate before
//! integration; all other rate ratios are handled generically.

use measurements::Speed;

use crate::models::LdSession;

/// A 2-D position relative to the session start point, in metres.
#[derive(Debug, Clone, Copy)]
pub struct Point2D {
  pub x: f64,
  pub y: f64,
}

/// Integrate SPEED + ROTY into a sequence of (x, y) positions for a
/// specific time window within the session.
///
/// `start_s` – session-relative start time in seconds.
/// `dur_s`   – duration in seconds.
///
/// Returns `None` if either channel is absent, has no samples, or the
/// time window falls outside the recorded data.
pub fn dead_reckoning_slice(session: &LdSession, start_s: f64, dur_s: f64) -> Option<Vec<Point2D>> {
  let speed_ch = session.channels.iter().find(|c| c.name == "SPEED")?;
  let roty_ch = session.channels.iter().find(|c| c.name == "ROTY")?;

  if speed_ch.samples.is_empty() || roty_ch.samples.is_empty() {
    return None;
  }

  let dt = 1.0 / roty_ch.frequency as f64;
  let ratio = ((speed_ch.frequency as usize) / (roty_ch.frequency as usize).max(1)).max(1);

  let roty_start = (start_s * roty_ch.frequency as f64) as usize;
  let roty_end = ((start_s + dur_s) * roty_ch.frequency as f64) as usize;
  let roty_end = roty_end.min(roty_ch.samples.len());

  if roty_start >= roty_ch.samples.len() {
    return None;
  }

  let speed_start = roty_start * ratio;
  if speed_start >= speed_ch.samples.len() {
    return None;
  }

  let n = (roty_end - roty_start).min((speed_ch.samples.len() - speed_start) / ratio);

  if n == 0 {
    return None;
  }

  let mut points = Vec::with_capacity(n);
  let mut x = 0.0f64;
  let mut y = 0.0f64;
  let mut heading = 0.0f64;

  for i in 0..n {
    let r_idx = roty_start + i;
    let s_start = speed_start + i * ratio;
    let s_end = (s_start + ratio).min(speed_ch.samples.len());
    if s_start >= s_end {
      break;
    }
    let raw_speed = speed_ch.samples[s_start..s_end].iter().sum::<f64>() / (s_end - s_start) as f64;
    let v_ms = Speed::from_metres_per_second(raw_speed).as_metres_per_second();
    let omega = roty_ch.samples[r_idx];

    heading += omega * dt;
    x += v_ms * heading.cos() * dt;
    y += v_ms * heading.sin() * dt;

    points.push(Point2D { x, y });
  }

  Some(points)
}

/// Integrate SPEED + ROTY into a sequence of (x, y) positions.
///
/// Returns `None` if either channel is absent or has no samples.
pub fn dead_reckoning(session: &LdSession) -> Option<Vec<Point2D>> {
  let speed_ch = session.channels.iter().find(|c| c.name == "SPEED")?;
  let roty_ch = session.channels.iter().find(|c| c.name == "ROTY")?;

  if speed_ch.samples.is_empty() || roty_ch.samples.is_empty() {
    return None;
  }

  let dt = 1.0 / roty_ch.frequency as f64;

  // How many SPEED samples fall in each ROTY time step.
  let ratio = if roty_ch.frequency == 0 {
    return None;
  } else {
    (speed_ch.frequency as usize).max(1) / (roty_ch.frequency as usize).max(1)
  }
  .max(1);

  let n = roty_ch.samples.len().min(speed_ch.samples.len() / ratio);

  let mut points = Vec::with_capacity(n);
  let mut x = 0.0f64;
  let mut y = 0.0f64;
  let mut heading = 0.0f64;

  for i in 0..n {
    // Average SPEED samples that correspond to this ROTY time step.
    let start = i * ratio;
    let end = (start + ratio).min(speed_ch.samples.len());
    let raw_speed: f64 = speed_ch.samples[start..end].iter().sum::<f64>() / (end - start) as f64;
    let v_ms = Speed::from_metres_per_second(raw_speed).as_metres_per_second();

    heading += roty_ch.samples[i] * dt;
    x += v_ms * heading.cos() * dt;
    y += v_ms * heading.sin() * dt;

    points.push(Point2D { x, y });
  }

  Some(points)
}
