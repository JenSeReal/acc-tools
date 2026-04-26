use acc_session_viewer_lib::event::{Event, EventType};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use time::{Date, Month};

use acc_tools_ui_components::components::ui::date_picker::{
  DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow,
  DatePickerTable, DatePickerTitle, DatePickerWeekDay,
};

#[cfg(any(feature = "csr", feature = "hydrate"))]
use crate::common::state::ChampionshipEventCtx;
use crate::common::state::{ChampionshipState, EventState};

// ── Calendar helpers ──────────────────────────────────────────────────────────

#[derive(Clone)]
struct CalDay {
  day: u8,
  disabled: bool,
}

fn calendar_weeks(year: i32, month: Month) -> Vec<Vec<CalDay>> {
  let Ok(first) = Date::from_calendar_date(year, month, 1) else {
    return vec![];
  };
  let offset = (first.weekday().number_from_monday() as usize) - 1;
  let (prev_month, prev_year) = if month == Month::January {
    (Month::December, year - 1)
  } else {
    (month.previous(), year)
  };
  let prev_days = prev_month.length(prev_year) as usize;
  let cur_days = month.length(year) as usize;
  let mut days: Vec<CalDay> = Vec::new();
  for i in 0..offset {
    days.push(CalDay {
      day: (prev_days - offset + i + 1) as u8,
      disabled: true,
    });
  }
  for d in 1..=cur_days {
    days.push(CalDay {
      day: d as u8,
      disabled: false,
    });
  }
  let trailing = (7 - days.len() % 7) % 7;
  for d in 1..=trailing {
    days.push(CalDay {
      day: d as u8,
      disabled: true,
    });
  }
  days.chunks(7).map(|c| c.to_vec()).collect()
}

fn month_name(m: Month) -> &'static str {
  match m {
    Month::January => "January",
    Month::February => "February",
    Month::March => "March",
    Month::April => "April",
    Month::May => "May",
    Month::June => "June",
    Month::July => "July",
    Month::August => "August",
    Month::September => "September",
    Month::October => "October",
    Month::November => "November",
    Month::December => "December",
  }
}

// ── RangeDatePicker ───────────────────────────────────────────────────────────

#[component]
fn RangeDatePicker(
  start_date: RwSignal<Option<Date>>,
  end_date: RwSignal<Option<Date>>,
) -> impl IntoView {
  let now = time::OffsetDateTime::now_utc();
  let display = RwSignal::new(now.date());

  let sentinel = Date::from_calendar_date(1900, Month::January, 1).unwrap();
  let start_sig = RwSignal::new(sentinel);
  let end_sig = RwSignal::new(sentinel);

  Effect::new(move |_| {
    start_sig.set(start_date.get().unwrap_or(sentinel));
    end_sig.set(end_date.get().unwrap_or(sentinel));
  });

  let go_prev = move |_| {
    let d = display.get();
    let new = if d.month() == Month::January {
      Date::from_calendar_date(d.year() - 1, Month::December, 1)
    } else {
      Date::from_calendar_date(d.year(), d.month().previous(), 1)
    };
    if let Ok(nd) = new {
      display.set(nd);
    }
  };

  let go_next = move |_| {
    let d = display.get();
    let new = if d.month() == Month::December {
      Date::from_calendar_date(d.year() + 1, Month::January, 1)
    } else {
      Date::from_calendar_date(d.year(), d.month().next(), 1)
    };
    if let Ok(nd) = new {
      display.set(nd);
    }
  };

  view! {
    <DatePicker>
      <DatePickerHeader>
        <DatePickerNavButton on:click=go_prev>"‹"</DatePickerNavButton>
        <DatePickerTitle>
          {move || format!("{} {}", month_name(display.get().month()), display.get().year())}
        </DatePickerTitle>
        <DatePickerNavButton on:click=go_next>"›"</DatePickerNavButton>
      </DatePickerHeader>
      <DatePickerTable>
        <thead>
          <DatePickerRow>
            <DatePickerWeekDay>"Mo"</DatePickerWeekDay>
            <DatePickerWeekDay>"Tu"</DatePickerWeekDay>
            <DatePickerWeekDay>"We"</DatePickerWeekDay>
            <DatePickerWeekDay>"Th"</DatePickerWeekDay>
            <DatePickerWeekDay>"Fr"</DatePickerWeekDay>
            <DatePickerWeekDay>"Sa"</DatePickerWeekDay>
            <DatePickerWeekDay>"Su"</DatePickerWeekDay>
          </DatePickerRow>
        </thead>
        <tbody>
          {move || {
            let year = display.get().year();
            let month = display.get().month();
            calendar_weeks(year, month)
              .into_iter()
              .map(|week| view! {
                <DatePickerRow>
                  {week.into_iter().map(|d| {
                    let day = d.day;
                    let disabled = d.disabled;
                    let on_click = move |day: u8| {
                      if disabled { return; }
                      let Ok(new_date) = Date::from_calendar_date(year, month, day) else { return };
                      match (start_date.get(), end_date.get()) {
                        (None, _) | (_, None) => {
                          start_date.set(Some(new_date));
                          end_date.set(Some(new_date));
                        }
                        (Some(s), Some(e)) => {
                          let ds = (new_date - s).whole_days().abs();
                          let de = (new_date - e).whole_days().abs();
                          if ds <= de { start_date.set(Some(new_date)); } else { end_date.set(Some(new_date)); }
                          let s2 = start_date.get().unwrap_or(new_date);
                          let e2 = end_date.get().unwrap_or(new_date);
                          if s2 > e2 { start_date.set(Some(e2)); end_date.set(Some(s2)); }
                        }
                      }
                    };
                    view! {
                      <DatePickerCell
                        day=day year=year month=month disabled=disabled
                        start_date=start_sig end_date=end_sig on_click=on_click
                      />
                    }
                  }).collect_view()}
                </DatePickerRow>
              })
              .collect_view()
          }}
        </tbody>
      </DatePickerTable>
    </DatePicker>
  }
}

// ── AddEventDialog ────────────────────────────────────────────────────────────

#[component]
fn AddEventDialog(open: RwSignal<bool>, championship_state: ChampionshipState) -> impl IntoView {
  use time::macros::format_description;
  let fmt = format_description!("[year]-[month]-[day]");

  let title: RwSignal<String> = RwSignal::new(String::new());
  let event_type: RwSignal<EventType> = RwSignal::new(EventType::default());
  let start_date_opt: RwSignal<Option<Date>> = RwSignal::new(None);
  let end_date_opt: RwSignal<Option<Date>> = RwSignal::new(None);
  let error: RwSignal<Option<String>> = RwSignal::new(None);

  let on_submit = move |_| {
    let t = title.get();
    if t.trim().is_empty() {
      error.set(Some("Title is required.".to_string()));
      return;
    }

    let new_event = Event {
      title: t,
      event_type: event_type.get(),
      start_date: start_date_opt
        .get()
        .and_then(|d| d.format(fmt).ok())
        .unwrap_or_default(),
      end_date: end_date_opt
        .get()
        .and_then(|d| d.format(fmt).ok())
        .unwrap_or_default(),
      slots: vec![],
    };

    championship_state.championship.update(|opt| {
      if let Some((_, c)) = opt {
        c.events.push(new_event);
      }
    });

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
      if let Some((id, championship)) = championship_state.championship.get() {
        leptos::task::spawn_local(async move {
          let _ = crate::common::db::save_championship(&championship).await;
          // Update the stored id with the new one returned — but since we're
          // updating in place we need to persist with the existing id.
          // For now we just re-save; the server upserts by record id.
          let _ = crate::common::db::delete_championship(&id).await;
        });
      }
    }

    title.set(String::new());
    event_type.set(EventType::default());
    start_date_opt.set(None);
    end_date_opt.set(None);
    error.set(None);
    open.set(false);
  };

  view! {
    <div class="fixed inset-0 z-40 bg-black/50" on:click=move |_| open.set(false) />
    <div
      class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-50 bg-background border rounded-2xl shadow-lg p-6 w-full max-w-lg space-y-6"
      on:click=|ev| ev.stop_propagation()
    >
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold">"Add Event"</h2>
        <button
          on:click=move |_| open.set(false)
          class="rounded-md p-1 text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
        >
          "✕"
        </button>
      </div>

      {move || error.get().map(|e| view! {
        <div class="bg-destructive/10 border border-destructive text-destructive px-4 py-3 rounded-md text-sm">{e}</div>
      })}

      <div class="space-y-1">
        <p class="text-sm font-medium">"Title"</p>
        <input
          type="text"
          placeholder="e.g. Round 1 – Monza"
          prop:value=move || title.get()
          on:input=move |ev| title.set(event_target_value(&ev))
          class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
        />
      </div>

      <div class="space-y-1">
        <p class="text-sm font-medium">"Event Type"</p>
        <select
          on:change=move |ev| {
            event_type.set(match event_target_value(&ev).as_str() {
              "TestDay" => EventType::TestDay,
              "TrackDay" => EventType::TrackDay,
              "FreePractice" => EventType::FreePractice,
              _ => EventType::RaceWeekend,
            });
          }
          class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-ring"
        >
          <option value="RaceWeekend" selected=move || event_type.get() == EventType::RaceWeekend>"Race Weekend"</option>
          <option value="TestDay" selected=move || event_type.get() == EventType::TestDay>"Test Day"</option>
          <option value="TrackDay" selected=move || event_type.get() == EventType::TrackDay>"Track Day"</option>
          <option value="FreePractice" selected=move || event_type.get() == EventType::FreePractice>"Free Practice"</option>
        </select>
      </div>

      <div class="space-y-2">
        <p class="text-sm font-medium">"Event Dates"</p>
        <RangeDatePicker start_date=start_date_opt end_date=end_date_opt />
        <p class="text-xs text-muted-foreground">
          {move || match (start_date_opt.get(), end_date_opt.get()) {
            (Some(s), Some(e)) if s == e => s.format(fmt).unwrap_or_default(),
            (Some(s), Some(e)) => format!("{} – {}", s.format(fmt).unwrap_or_default(), e.format(fmt).unwrap_or_default()),
            _ => "No dates selected".to_string(),
          }}
        </p>
      </div>

      <button
        on:click=on_submit
        class="w-full inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
      >
        "Add Event"
      </button>
    </div>
  }
}

// ── ChampionshipViewPage ──────────────────────────────────────────────────────

#[component]
pub fn ChampionshipViewPage() -> impl IntoView {
  let cs = expect_context::<ChampionshipState>();
  let event_state = expect_context::<EventState>();
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let _ = event_state;

  Effect::new(move || {
    if cs.championship.get().is_none() {
      let navigate = use_navigate();
      navigate("/event", Default::default());
    }
  });

  let add_open: RwSignal<bool> = RwSignal::new(false);
  let save_status: RwSignal<Option<String>> = RwSignal::new(None);

  let on_back = move |_| {
    cs.championship.set(None);
    let navigate = use_navigate();
    navigate("/event", Default::default());
  };

  let on_save = move |_| {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
      let Some((id, championship)) = cs.championship.get() else {
        return;
      };
      save_status.set(Some("Saving…".to_string()));
      leptos::task::spawn_local(async move {
        // Delete old record and save with same data (server assigns new id from POST).
        // To truly update we'd need a PUT endpoint; for now delete+create is fine.
        let _ = crate::common::db::delete_championship(&id).await;
        match crate::common::db::save_championship(&championship).await {
          Ok(new_id) => {
            cs.championship.update(|opt| {
              if let Some((stored_id, _)) = opt {
                *stored_id = new_id;
              }
            });
            save_status.set(Some("Saved!".to_string()));
          }
          Err(e) => save_status.set(Some(format!("Error: {e}"))),
        }
      });
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    save_status.set(Some("Save not available in SSR mode.".to_string()));
  };

  let navigate_sv = StoredValue::new(use_navigate());
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let _ = navigate_sv;

  view! {
    <main class="container mx-auto max-w-4xl px-4 py-8 space-y-8">
      <Show when=move || cs.championship.get().is_some()>

        // ── Header ────────────────────────────────────────────────────────────
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold">
              {move || cs.championship.get().map(|(_, c)| c.title.clone()).unwrap_or_default()}
            </h1>
            <p class="text-sm text-muted-foreground mt-0.5">
              {move || cs.championship.get().map(|(_, c)| {
                let n = c.events.len();
                format!("{n} event{}", if n == 1 { "" } else { "s" })
              }).unwrap_or_default()}
            </p>
          </div>

          <div class="flex items-center gap-3">
            {move || save_status.get().map(|msg| {
              let is_error = msg.starts_with("Error:");
              let cls = if is_error { "text-sm text-destructive" } else { "text-sm text-muted-foreground" };
              view! { <span class=cls>{msg}</span> }
            })}
            <button
              on:click=move |_| add_open.set(true)
              class="inline-flex items-center gap-2 rounded-md border border-input bg-background px-4 py-2 text-sm font-medium shadow-sm transition-colors hover:bg-muted"
            >
              "+ Add Event"
            </button>
            <button
              on:click=on_save
              class="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
            >
              "Save"
            </button>
            <button
              on:click=on_back
              class="inline-flex items-center gap-2 rounded-md bg-secondary px-4 py-2 text-sm font-medium text-secondary-foreground shadow-sm transition-colors hover:bg-secondary/80"
            >
              "Back"
            </button>
          </div>
        </div>

        // ── Events list ───────────────────────────────────────────────────────
        <div class="rounded-md border divide-y">
          <For
            each=move || {
              cs.championship.get()
                .map(|(_, c)| c.events.into_iter().enumerate().collect::<Vec<_>>())
                .unwrap_or_default()
            }
            key=|(idx, e)| format!("{idx}-{}", e.title)
            children=move |(idx, event)| {
              #[cfg(not(any(feature = "csr", feature = "hydrate")))]
              let _ = idx;
              let title = event.title.clone();
              let type_label = event.event_type.label();
              let date_label = if event.start_date.is_empty() {
                "No date".to_string()
              } else if event.start_date == event.end_date {
                event.start_date.clone()
              } else {
                format!("{} – {}", event.start_date, event.end_date)
              };
              let slot_count = event.slots.len();

              let on_open = move |_| {
                #[cfg(any(feature = "csr", feature = "hydrate"))]
                {
                  let Some((championship_id, championship)) = cs.championship.get() else { return };
                  let Some(ev) = championship.events.get(idx).cloned() else { return };
                  event_state.event.set(Some(ev));
                  event_state.slot_sessions.update(|m| m.clear());
                  event_state.error.set(None);
                  event_state.championship_ctx.set(Some(ChampionshipEventCtx {
                    id: championship_id,
                    event_idx: idx,
                  }));
                  navigate_sv.get_value()("/event/view", Default::default());
                }
              };

              view! {
                <div class="flex items-center justify-between px-4 py-3">
                  <div class="flex flex-col gap-0.5">
                    <span class="font-medium text-sm">{title}</span>
                    <span class="text-xs text-muted-foreground">
                      {type_label}" · "{date_label}" · "
                      {slot_count}" session"{if slot_count == 1 { "" } else { "s" }}
                    </span>
                  </div>
                  <button
                    on:click=on_open
                    class="inline-flex items-center rounded-md border border-input bg-background px-3 py-1.5 text-xs font-medium shadow-sm transition-colors hover:bg-muted"
                  >
                    "Open"
                  </button>
                </div>
              }
            }
          />
          <Show when=move || cs.championship.get().map(|(_, c)| c.events.is_empty()).unwrap_or(true)>
            <p class="px-4 py-3 text-sm text-muted-foreground">
              "No events yet. Click \"+ Add Event\" to add the first round."
            </p>
          </Show>
        </div>

        // ── Add Event dialog ──────────────────────────────────────────────────
        <Show when=move || add_open.get()>
          <AddEventDialog open=add_open championship_state=cs />
        </Show>

      </Show>
    </main>
  }
}
