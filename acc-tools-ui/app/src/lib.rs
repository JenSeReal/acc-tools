pub mod app;
pub mod db;
pub mod layouts;
pub mod pages;
pub mod state;

pub use app::App;

#[cfg(feature = "csr")]
mod wasm {
  use crate::App;
  use leptos::prelude::*;
  use wasm_bindgen::prelude::wasm_bindgen;

  #[wasm_bindgen(start)]
  pub fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App);
  }
}
