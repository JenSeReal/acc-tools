use acc_race_session_reader_lib::car::car_model::CarModel;
use acc_race_session_reader_lib::loaded_session::LoadedSession;
use leptos::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlSelectElement;

#[component]
pub fn CarSelector(
  session: ReadSignal<Option<LoadedSession>>,
  selected_car_model: WriteSignal<Option<CarModel>>,
) -> impl IntoView {
  let on_change = move |ev: web_sys::Event| {
    let target: HtmlSelectElement = ev.target().unwrap().dyn_into().unwrap();
    let value = target.value();
    if value.is_empty() {
      selected_car_model.set(None);
    } else if let Ok(id) = value.parse::<u32>() {
      selected_car_model.set(Some(CarModel::from_id(id)));
    }
  };

  view! {
      <div class="car-selector">
          <label for="car-select">"Filter by Car Model:"</label>
          <select id="car-select" on:change=on_change class="car-select">
              <option value="">"All Cars"</option>
              {move || {
                  session.get().map(|s| {
                      s.car_models.iter().map(|car_model| {
                          let id = car_model.to_id();
                          let label = car_model.name().to_string();
                          view! {
                              <option value={id.to_string()}>{label}</option>
                          }
                      }).collect_view()
                  })
              }}
          </select>
      </div>
  }
}
