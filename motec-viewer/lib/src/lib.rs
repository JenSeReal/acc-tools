pub mod ld_file;
pub mod ldx_file;
pub mod models;

pub use ld_file::parse_ld;
pub use ldx_file::parse_ldx;
pub use models::{LapInfo, LdChannel, LdSession};

#[cfg(test)]
mod tests {
  const KYALAMI_LDX: &[u8] =
    include_bytes!("../tests/fixtures/Kyalami-mclaren_720s_gt3_evo-9-2026.03.28-09.06.26.ldx");
  const KYALAMI_LD: &[u8] =
    include_bytes!("../tests/fixtures/Kyalami-mclaren_720s_gt3_evo-9-2026.03.28-09.06.26.ld");

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
  }

  #[test]
  fn test_parse_ld_kyalami() {
    let session = crate::parse_ld(KYALAMI_LD).expect("ld parse failed");
    assert!(
      !session.channels.is_empty(),
      "expected at least one channel"
    );
    // Every channel's decoded sample vec must match the recorded sample_count
    for ch in &session.channels {
      assert_eq!(
        ch.samples.len(),
        ch.sample_count as usize,
        "channel '{}' sample count mismatch",
        ch.name
      );
    }
    // Channels with samples must have finite statistics
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
  }

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
