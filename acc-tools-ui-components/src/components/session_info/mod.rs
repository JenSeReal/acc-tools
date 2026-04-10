use acc_session_viewer_lib::loaded_session::LoadedSession;
use leptos::prelude::*;

#[component]
pub fn SessionInfo(
  session: ReadSignal<Option<LoadedSession>>,
  #[prop(optional)] class: Option<String>,
) -> impl IntoView {
  view! {
    {move || session.get().map(|s| {
      let session_type = format!("{:?}", s.session_type);
      let track = format!("{:?}", s.track_name);
      let car_count = s.car_count;

      let wrapper_class = class.clone()
        .map(|c| format!("rounded-lg border bg-card p-4 shadow-sm {}", c))
        .unwrap_or_else(|| "rounded-lg border bg-card p-4 shadow-sm".to_string());

      view! {
        <div class=wrapper_class>
          <h2 class="text-lg font-semibold text-card-foreground mb-3">"Session Info"</h2>
          <p class="text-sm text-muted-foreground"><strong class="text-foreground">"Type: "</strong>{session_type}</p>
          <p class="text-sm text-muted-foreground"><strong class="text-foreground">"Track: "</strong>{track}</p>
          <p class="text-sm text-muted-foreground"><strong class="text-foreground">"Cars: "</strong>{car_count}</p>
        </div>
      }
    })}
  }
}
