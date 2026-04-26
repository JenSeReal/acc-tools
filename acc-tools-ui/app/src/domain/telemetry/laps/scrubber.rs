use leptos::prelude::*;
use motec_viewer_lib::LdSession;

use super::helpers::{LAP_COLORS, fmt_lap_time};

/// Grid of per-channel values at the current cursor position.
/// `laps_info`: (lap_idx, start_time_s) for each selected lap.
/// `channel_indices`: which session channels to display.
#[component]
pub fn CursorValues(
  session: LdSession,
  laps_info: Vec<(usize, f64)>,
  cursor_time: RwSignal<f64>,
  channel_indices: Vec<usize>,
) -> impl IntoView {
  let open = RwSignal::new(false);

  let channels = StoredValue::new(
    channel_indices
      .iter()
      .filter_map(|&i| session.channels.get(i).cloned())
      .collect::<Vec<_>>(),
  );
  let laps_sv = StoredValue::new(laps_info);

  view! {
    <div class="rounded-lg border bg-card">
      <button
        class="w-full flex items-center justify-between p-3 hover:bg-accent/50 transition-colors"
        on:click=move |_| open.update(|v| *v = !*v)
      >
        <p class="text-xs font-semibold text-muted-foreground uppercase tracking-wider">
          "Values at cursor"
        </p>
        <div class="flex items-center gap-3">
          <span class="text-xs text-muted-foreground tabular-nums font-mono">
            {move || format!("T {}", fmt_lap_time(cursor_time.get()))}
          </span>
          <span class="text-xs text-muted-foreground">
            {move || if open.get() { "▲" } else { "▼" }}
          </span>
        </div>
      </button>
      <Show when=move || open.get()>
        <div class="px-3 pb-3">
          <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 gap-2">
          {channels.with_value(|chs| chs.iter().map(|ch| {
            let label = if ch.short_name.is_empty() { ch.name.clone() } else { ch.short_name.clone() };
            let unit = ch.unit.clone();
            let freq = ch.frequency;
            let samples = StoredValue::new(ch.samples.clone());
            view! {
              <div class="rounded border border-border p-2">
                <div class="text-xs text-muted-foreground mb-1">
                  {label}
                  {if unit.is_empty() { String::new() } else { format!(" ({unit})") }}
                </div>
                {move || {
                  let t = cursor_time.get();
                  laps_sv.with_value(|laps| {
                    laps.iter().map(|&(lap_idx, start_s)| {
                      let si = ((start_s + t) * freq as f64) as usize;
                      let val = samples.with_value(|s| {
                        s.get(si).map(|&v| format!("{v:.2}")).unwrap_or_else(|| "—".to_string())
                      });
                      let color = LAP_COLORS[lap_idx % LAP_COLORS.len()];
                      view! {
                        <div class="flex items-center justify-between gap-1">
                          <span class="text-xs tabular-nums" style=format!("color:{color}")>
                            {format!("L{}", lap_idx + 1)}
                          </span>
                          <span class="font-mono font-semibold tabular-nums text-sm" style=format!("color:{color}")>
                            {val}
                          </span>
                        </div>
                      }
                    }).collect_view()
                  })
                }}
              </div>
            }
          }).collect_view())}
          </div>
        </div>
      </Show>
    </div>
  }
}
