pub struct EventRoutes;

impl EventRoutes {
  pub fn base_url() -> &'static str {
    "/event"
  }

  pub fn view_url() -> &'static str {
    "/event/view"
  }

  pub fn label() -> &'static str {
    "Event"
  }
}
