pub mod display;
pub mod ld_file;
pub mod ldx_file;
pub mod models;
pub mod track_map;
pub mod units;

pub use display::{channel_description, fmt_duration, fmt_lap_time};
pub use ld_file::parse_ld;
pub use ldx_file::parse_ldx;
pub use models::{LapInfo, LdChannel, LdSession, channel_range, filter_racing_laps, racing_window};
pub use track_map::{Point2D, cumulative_distance, dead_reckoning, dead_reckoning_slice};
pub use units::{kmh_to_ms, kpa_to_psi, ms_to_kmh, psi_to_kpa};

#[cfg(test)]
mod tests {
  use crate::ld_file::LdError;

  const KYALAMI_LDX: &[u8] =
    include_bytes!("../tests/fixtures/Kyalami-mclaren_720s_gt3_evo-9-2026.03.28-09.06.26.ldx");
  const KYALAMI_LD: &[u8] =
    include_bytes!("../tests/fixtures/Kyalami-mclaren_720s_gt3_evo-9-2026.03.28-09.06.26.ld");

  // ── ld ──────────────────────────────────────────────────────────────────────

  #[test]
  fn test_parse_ld_kyalami() {
    let session = crate::parse_ld(KYALAMI_LD).expect("ld parse failed");
    assert!(
      !session.channels.is_empty(),
      "expected at least one channel"
    );

    // Session metadata must be populated
    assert!(!session.venue.is_empty(), "venue should not be empty");
    assert!(!session.date.is_empty(), "date should not be empty");

    // Every channel's decoded sample vec must match the recorded sample_count
    for ch in &session.channels {
      assert_eq!(
        ch.samples.len(),
        ch.sample_count as usize,
        "channel '{}' sample count mismatch",
        ch.name
      );
    }

    // Channels with samples must have finite statistics and consistent min/max
    for ch in session.channels.iter().filter(|c| !c.samples.is_empty()) {
      assert!(
        ch.min.is_finite(),
        "channel '{}' min is not finite",
        ch.name
      );
      assert!(
        ch.max.is_finite(),
        "channel '{}' max is not finite",
        ch.name
      );
      assert!(ch.max >= ch.min, "channel '{}' max < min", ch.name);
    }

    // duration_s must be positive for channels with a non-zero frequency
    for ch in session.channels.iter().filter(|c| c.frequency > 0) {
      assert!(
        ch.duration_s() > 0.0,
        "channel '{}' duration should be positive",
        ch.name
      );
    }

    // Channel names must not be empty
    for ch in &session.channels {
      assert!(!ch.name.is_empty(), "channel name must not be empty");
    }
  }

  #[test]
  fn test_ld_bad_magic() {
    let mut bad = KYALAMI_LD[..64].to_vec();
    bad[0] = 0xFF;
    bad[1] = 0xFF;
    bad[2] = 0xFF;
    bad[3] = 0xFF;
    assert!(
      matches!(crate::parse_ld(&bad), Err(LdError::BadMagic(_))),
      "expected BadMagic error"
    );
  }

  #[test]
  fn test_ld_too_short() {
    assert!(
      matches!(crate::parse_ld(&[]), Err(LdError::TooShort)),
      "expected TooShort error for empty input"
    );
    assert!(
      matches!(crate::parse_ld(&[0x40, 0x00]), Err(LdError::TooShort)),
      "expected TooShort error for 2-byte input"
    );
  }

  // ── ldx ─────────────────────────────────────────────────────────────────────

  #[test]
  fn test_parse_ldx_kyalami() {
    let laps = crate::parse_ldx(KYALAMI_LDX).expect("ldx parse failed");
    assert!(!laps.is_empty(), "expected at least one lap");

    // Lap numbers must be consecutive starting at 1
    for (i, lap) in laps.iter().enumerate() {
      assert_eq!(lap.lap_number, i + 1);
    }

    // All laps except the last must have a positive duration
    for lap in laps.iter().take(laps.len().saturating_sub(1)) {
      let dur = lap
        .duration_s
        .expect("non-final lap should have a duration");
      assert!(dur > 0.0, "lap duration must be positive");
    }

    // Last lap has no end marker so duration is None
    assert!(laps.last().unwrap().duration_s.is_none());

    // All start times must be non-negative
    for lap in &laps {
      assert!(
        lap.start_time_s >= 0.0,
        "lap {} has negative start time",
        lap.lap_number
      );
    }

    // Start times must be strictly increasing
    for w in laps.windows(2) {
      assert!(
        w[1].start_time_s > w[0].start_time_s,
        "lap start times not strictly increasing at lap {}",
        w[1].lap_number
      );
    }
  }

  #[test]
  fn test_parse_ldx_empty() {
    let laps = crate::parse_ldx(b"").expect("empty ldx should not error");
    assert!(laps.is_empty(), "empty input should yield no laps");
  }

  #[test]
  fn test_parse_ldx_no_times() {
    let laps =
      crate::parse_ldx(b"<LDXData><Markers></Markers></LDXData>").expect("parse should not error");
    assert!(
      laps.is_empty(),
      "xml with no Time values should yield no laps"
    );
  }

  // ── channel inspection (run with -- --nocapture) ────────────────────────────

  #[test]
  fn print_channels() {
    let session = crate::parse_ld(KYALAMI_LD).expect("ld parse failed");
    for ch in &session.channels {
      println!(
        "{:<32} unit={:<12} {}Hz  {} samples",
        ch.name, ch.unit, ch.frequency, ch.sample_count
      );
    }
  }

  // ── track map ───────────────────────────────────────────────────────────────

  #[test]
  fn test_dead_reckoning_kyalami() {
    let session = crate::parse_ld(KYALAMI_LD).expect("ld parse failed");
    let points = crate::dead_reckoning(&session).expect("dead_reckoning returned None");

    assert!(!points.is_empty(), "expected track points");

    // All coordinates must be finite
    for p in &points {
      assert!(p.x.is_finite(), "non-finite x");
      assert!(p.y.is_finite(), "non-finite y");
    }

    // Bounding box should span at least 500 m in each axis for a full circuit
    let x_min = points.iter().map(|p| p.x).fold(f64::INFINITY, f64::min);
    let x_max = points.iter().map(|p| p.x).fold(f64::NEG_INFINITY, f64::max);
    let y_min = points.iter().map(|p| p.y).fold(f64::INFINITY, f64::min);
    let y_max = points.iter().map(|p| p.y).fold(f64::NEG_INFINITY, f64::max);
    assert!(
      x_max - x_min > 500.0,
      "x extent too small: {:.0}m",
      x_max - x_min
    );
    assert!(
      y_max - y_min > 500.0,
      "y extent too small: {:.0}m",
      y_max - y_min
    );
  }

  // ── combined ────────────────────────────────────────────────────────────────

  #[test]
  fn test_ld_and_ldx_time_alignment() {
    let session = crate::parse_ld(KYALAMI_LD).expect("ld parse failed");
    let laps = crate::parse_ldx(KYALAMI_LDX).expect("ldx parse failed");
    let session_dur = session
      .channels
      .iter()
      .map(|ch| ch.duration_s())
      .fold(0.0f64, f64::max);

    // Every lap start must fall within the session duration
    for lap in &laps {
      assert!(
        lap.start_time_s < session_dur,
        "lap {} start ({:.1}s) exceeds session duration ({:.1}s)",
        lap.lap_number,
        lap.start_time_s,
        session_dur
      );
    }
  }
}
