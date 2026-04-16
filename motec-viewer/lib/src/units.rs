/// Convert metres per second to kilometres per hour.
pub fn ms_to_kmh(v: f64) -> f64 {
  v * 3.6
}

/// Convert kilometres per hour to metres per second.
pub fn kmh_to_ms(v: f64) -> f64 {
  v / 3.6
}

/// Convert PSI to kilopascals.
pub fn psi_to_kpa(v: f64) -> f64 {
  v * 6.894_757_293_168_36
}

/// Convert kilopascals to PSI.
pub fn kpa_to_psi(v: f64) -> f64 {
  v / 6.894_757_293_168_36
}
