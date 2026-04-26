pub struct SessionRoutes;

impl SessionRoutes {
  pub fn base_url() -> &'static str {
    "/session"
  }

  pub fn view_url() -> &'static str {
    "/session/view"
  }

  pub fn label() -> &'static str {
    "Session"
  }
}
