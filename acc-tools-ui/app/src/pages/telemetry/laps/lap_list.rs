use leptos::prelude::*;
use motec_viewer_lib::LapInfo;

use super::helpers::{LAP_COLORS, fmt_lap_time};

#[component]
pub fn LapSelector(laps: Vec<LapInfo>, selected_laps: RwSignal<Vec<usize>>) -> impl IntoView {
  view! {
    <div class="rounded-lg border bg-card p-3">
      <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider mb-3">
        "Laps"
      </p>
      <div class="flex flex-col gap-1">
        {laps
          .into_iter()
          .map(|lap| {
            let idx = lap.lap_number - 1;
            let color = LAP_COLORS[idx % LAP_COLORS.len()];
            let dur = lap.duration_s.map(fmt_lap_time).unwrap_or_else(|| "—".to_string());
            view! {
              <button
                class=move || {
                  if selected_laps.get().contains(&idx) {
                    "w-full flex items-center gap-2 px-2.5 py-1.5 rounded-md text-xs font-medium border-2 transition-colors tabular-nums text-left"
                  } else {
                    "w-full flex items-center gap-2 px-2.5 py-1.5 rounded-md text-xs font-medium border transition-colors tabular-nums bg-card hover:bg-accent text-muted-foreground text-left"
                  }
                }
                style=move || {
                  if selected_laps.get().contains(&idx) {
                    format!("border-color:{color};background-color:{color}22;color:{color}")
                  } else {
                    String::new()
                  }
                }
                on:click=move |_| {
                  let mut v = selected_laps.get();
                  if let Some(pos) = v.iter().position(|&x| x == idx) {
                    v.remove(pos);
                  } else {
                    v.push(idx);
                  }
                  selected_laps.set(v);
                }
              >
                <span class="font-bold">{format!("L{}", lap.lap_number)}</span>
                <span class="opacity-75">{dur}</span>
              </button>
            }
          })
          .collect_view()}
      </div>
    </div>
  }
}
