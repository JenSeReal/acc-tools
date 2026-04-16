/// Format elapsed seconds as `h:mm:ss` (or `m:ss` when under one hour).
pub fn fmt_duration(secs: f64) -> String {
  if secs.is_nan() || secs < 0.0 {
    return "—".to_string();
  }
  let total_s = secs as u64;
  let h = total_s / 3600;
  let m = (total_s % 3600) / 60;
  let s = total_s % 60;
  if h > 0 {
    format!("{h}:{m:02}:{s:02}")
  } else {
    format!("{m}:{s:02}")
  }
}

/// Format a lap time as `m:ss.mmm`.
pub fn fmt_lap_time(secs: f64) -> String {
  if secs.is_nan() || secs < 0.0 {
    return "—".to_string();
  }
  let millis = (secs.fract() * 1000.0) as u64;
  let total_s = secs as u64;
  format!("{}:{:02}.{:03}", total_s / 60, total_s % 60, millis)
}

/// Human-readable description for a known MoTeC ACC channel name.
pub fn channel_description(name: &str) -> &'static str {
  match name {
    "THROTTLE" => "Throttle pedal position",
    "BRAKE" => "Brake pedal pressure",
    "CLUTCH" => "Clutch pedal position",
    "STEERANGLE" => "Steering wheel angle",
    "SPEED" => "Vehicle speed",
    "GEAR" => "Selected gear",
    "RPMS" => "Engine RPM",
    "TC" => "Traction control intervention",
    "ABS" => "ABS intervention",
    "G_LAT" => "Lateral acceleration",
    "G_LON" => "Longitudinal acceleration",
    "ROTY" => "Yaw rate",
    "LAP_BEACON" => "Lap marker signal",
    "TIME" => "Session elapsed time",
    "BRAKE_TEMP_LF" => "Brake temperature — front left",
    "BRAKE_TEMP_RF" => "Brake temperature — front right",
    "BRAKE_TEMP_LR" => "Brake temperature — rear left",
    "BRAKE_TEMP_RR" => "Brake temperature — rear right",
    "TYRE_PRESS_LF" => "Tyre pressure — front left",
    "TYRE_PRESS_RF" => "Tyre pressure — front right",
    "TYRE_PRESS_LR" => "Tyre pressure — rear left",
    "TYRE_PRESS_RR" => "Tyre pressure — rear right",
    "TYRE_TAIR_LF" => "Tyre air temperature — front left",
    "TYRE_TAIR_RF" => "Tyre air temperature — front right",
    "TYRE_TAIR_LR" => "Tyre air temperature — rear left",
    "TYRE_TAIR_RR" => "Tyre air temperature — rear right",
    "WHEEL_SPEED_LF" => "Wheel speed — front left",
    "WHEEL_SPEED_RF" => "Wheel speed — front right",
    "WHEEL_SPEED_LR" => "Wheel speed — rear left",
    "WHEEL_SPEED_RR" => "Wheel speed — rear right",
    "SUS_TRAVEL_LF" => "Suspension travel — front left",
    "SUS_TRAVEL_RF" => "Suspension travel — front right",
    "SUS_TRAVEL_LR" => "Suspension travel — rear left",
    "SUS_TRAVEL_RR" => "Suspension travel — rear right",
    "BUMPSTOPUP_RIDE_LF" => "Bump stop upper ride height — front left",
    "BUMPSTOPUP_RIDE_RF" => "Bump stop upper ride height — front right",
    "BUMPSTOPUP_RIDE_LR" => "Bump stop upper ride height — rear left",
    "BUMPSTOPUP_RIDE_RR" => "Bump stop upper ride height — rear right",
    "BUMPSTOPDN_RIDE_LF" => "Bump stop lower ride height — front left",
    "BUMPSTOPDN_RIDE_RF" => "Bump stop lower ride height — front right",
    "BUMPSTOPDN_RIDE_LR" => "Bump stop lower ride height — rear left",
    "BUMPSTOPDN_RIDE_RR" => "Bump stop lower ride height — rear right",
    "BUMPSTOP_FORCE_LF" => "Bump stop force — front left",
    "BUMPSTOP_FORCE_RF" => "Bump stop force — front right",
    "BUMPSTOP_FORCE_LR" => "Bump stop force — rear left",
    "BUMPSTOP_FORCE_RR" => "Bump stop force — rear right",
    "EN_ET" => "Electronics — engine torque",
    "EN_TL" => "Electronics — torque limit",
    "EN_TD" => "Electronics — torque demand",
    "EN_TB" => "Electronics — torque balance",
    "EN_TW" => "Electronics — torque at wheel",
    "EN_W" => "Electronics — wheel slip",
    "EN_Dy" => "Electronics — yaw moment demand",
    "EN_AP" => "Electronics — accelerator pedal",
    "EN_GR" => "Electronics — gear request",
    _ => "",
  }
}
