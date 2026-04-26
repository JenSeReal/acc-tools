pub struct TelemetryRoutes;

impl TelemetryRoutes {
  pub fn base_url() -> &'static str {
    "/telemetry"
  }

  pub fn view_url() -> &'static str {
    "/telemetry/view"
  }

  pub fn laps_url() -> &'static str {
    "/telemetry/laps"
  }

  pub fn label() -> &'static str {
    "Telemetry"
  }
}
