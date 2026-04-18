measures::define_measure_types! {
  exact,
  vector_properties [
    Velocity [
      MetrePerSecond {
        suffix: " m/s",
      }
      KilometrePerHour {
        suffix: " km/h",
        ratio: 0.277_777_8,
      }
    ]
  ]
}

/// Convert metres per second to kilometres per hour.
pub fn ms_to_kmh(v: f64) -> f64 {
  Measure::<MetrePerSecond>::new(v)
    .convert::<KilometrePerHour>()
    .value
}

/// Convert kilometres per hour to metres per second.
pub fn kmh_to_ms(v: f64) -> f64 {
  Measure::<KilometrePerHour>::new(v)
    .convert::<MetrePerSecond>()
    .value
}

/// Convert PSI to kilopascals.
pub fn psi_to_kpa(v: f64) -> f64 {
  v * 6.894_757_293_168_36
}

/// Convert kilopascals to PSI.
pub fn kpa_to_psi(v: f64) -> f64 {
  v / 6.894_757_293_168_36
}
