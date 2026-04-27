pub struct ChampionshipRoutes;

impl ChampionshipRoutes {
  pub fn base_url() -> &'static str {
    "/championship"
  }

  pub fn create_url() -> &'static str {
    "/championship/create"
  }

  pub fn view_url() -> &'static str {
    "/championship/view"
  }

  pub fn label() -> &'static str {
    "Championship"
  }
}
