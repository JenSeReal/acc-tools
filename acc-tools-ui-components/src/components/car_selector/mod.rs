use acc_session_viewer_lib::car::car_model::CarModel;
use acc_session_viewer_lib::loaded_session::LoadedSession;
use leptos::prelude::*;

#[cfg(any(feature = "csr", feature = "hydrate"))]
use leptos::wasm_bindgen::JsCast;

#[component]
pub fn CarSelector(
  session: ReadSignal<Option<LoadedSession>>,
  selected_car_model: WriteSignal<Option<CarModel>>,
  #[prop(optional)] class: Option<String>,
) -> impl IntoView {
  let wrapper_class = class
    .map(|c| format!("flex items-center gap-3 {}", c))
    .unwrap_or_else(|| "flex items-center gap-3".to_string());

  #[cfg(any(feature = "csr", feature = "hydrate"))]
  let on_change = move |ev: leptos::web_sys::Event| {
    let target: leptos::web_sys::HtmlSelectElement = ev.target().unwrap().dyn_into().unwrap();
    let value = target.value();
    if value.is_empty() {
      selected_car_model.set(None);
    } else if let Ok(id) = value.parse::<u32>() {
      selected_car_model.set(Some(CarModel::from_id(id)));
    }
  };

  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let on_change = move |_: leptos::ev::Event| {
    let _ = &selected_car_model;
  };

  view! {
    <div class=wrapper_class>
      <label for="car-select" class="text-sm font-medium text-muted-foreground">
        "Filter by Car Model:"
      </label>
      <select
        id="car-select"
        on:change=on_change
        class="h-9 rounded-md border border-input bg-background px-3 py-1 text-sm shadow-sm transition-colors focus:outline-none focus:ring-1 focus:ring-ring"
      >
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
