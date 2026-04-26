use acc_session_viewer_lib::event::{Event, EventType, ScheduleSlot};
use acc_session_viewer_lib::session_type::model::SessionType;
use acc_tools_ui_components::components::ui::date_picker::{
  DatePicker, DatePickerCell, DatePickerHeader, DatePickerMonth, DatePickerNavButton,
  DatePickerRow, DatePickerTable, DatePickerTitle, DatePickerWeekDay,
};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use std::sync::atomic::{AtomicU32, Ordering};
use time::{Date, Month};

use crate::common::state::EventState;

#[derive(Clone)]
struct CalDay {
  day: u8,
  disabled: bool,
}

fn calendar_weeks(year: i32, month: Month) -> Vec<Vec<CalDay>> {
  let Ok(first) = Date::from_calendar_date(year, month, 1) else {
    return vec![];
  };
  // number_from_monday: Mon=1 … Sun=7; shift to Mon=0 … Sun=6
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
    days.push(CalDay { day: (prev_days - offset + i + 1) as u8, disabled: true });
  }
  for d in 1..=cur_days {
    days.push(CalDay { day: d as u8, disabled: false });
  }
  let trailing = (7 - days.len() % 7) % 7;
  for d in 1..=trailing {
    days.push(CalDay { day: d as u8, disabled: true });
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

#[component]
fn SingleDatePicker(selected: RwSignal<String>) -> impl IntoView {
  use time::macros::format_description;
  let fmt = format_description!("[year]-[month]-[day]");

  let now = time::OffsetDateTime::now_utc();
  let (init_year, init_month) = {
    let s = selected.get_untracked();
    if let Ok(d) = Date::parse(&s, fmt) {
      (d.year(), d.month())
    } else {
      (now.year(), now.month())
    }
  };

  let view_year = RwSignal::new(init_year);
  let view_month = RwSignal::new(init_month);

  let fallback = Date::from_calendar_date(init_year, init_month, 1)
    .unwrap_or_else(|_| now.date());
  let selected_date: RwSignal<Date> =
    RwSignal::new(Date::parse(&selected.get_untracked(), fmt).unwrap_or(fallback));

  let prev_month = move |_| {
    let m = view_month.get();
    if m == Month::January {
      view_month.set(Month::December);
      view_year.update(|y| *y -= 1);
    } else {
      view_month.set(m.previous());
    }
  };
  let next_month = move |_| {
    let m = view_month.get();
    if m == Month::December {
      view_month.set(Month::January);
      view_year.update(|y| *y += 1);
    } else {
      view_month.set(m.next());
    }
  };

  view! {
    <DatePicker>
      <DatePickerHeader>
        <DatePickerNavButton on:click=prev_month>"‹"</DatePickerNavButton>
        <DatePickerTitle>
          {move || format!("{} {}", month_name(view_month.get()), view_year.get())}
        </DatePickerTitle>
        <DatePickerNavButton on:click=next_month>"›"</DatePickerNavButton>
      </DatePickerHeader>
      <DatePickerMonth>
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
              let year = view_year.get();
              let month = view_month.get();
              calendar_weeks(year, month)
                .into_iter()
                .map(|week| {
                  view! {
                    <DatePickerRow>
                      {week
                        .into_iter()
                        .map(|d| {
                          let day = d.day;
                          let disabled = d.disabled;
                          let on_click = move |day: u8| {
                            if let Ok(date) = Date::from_calendar_date(year, month, day) {
                              selected_date.set(date);
                              selected.set(date.format(fmt).unwrap_or_default());
                            }
                          };
                          view! {
                            <DatePickerCell
                              day=day
                              year=year
                              month=month
                              disabled=disabled
                              start_date=selected_date
                              end_date=selected_date
                              on_click=on_click
                            />
                          }
                        })
                        .collect_view()}
                    </DatePickerRow>
                  }
                })
                .collect_view()
            }}
          </tbody>
        </DatePickerTable>
      </DatePickerMonth>
    </DatePicker>
  }
}

#[component]
fn SingleDateTimePicker(selected: RwSignal<String>) -> impl IntoView {
  let init = selected.get_untracked();
  let (init_date, init_time) = if let Some(pos) = init.find('T') {
    (init[..pos].to_string(), init[pos + 1..].to_string())
  } else {
    (init, String::new())
  };

  let date_str: RwSignal<String> = RwSignal::new(init_date);
  let time_str: RwSignal<String> = RwSignal::new(init_time);

  Effect::new(move |_| {
    let d = date_str.get();
    let t = time_str.get();
    selected.set(if d.is_empty() {
      String::new()
    } else if t.is_empty() {
      d
    } else {
      format!("{}T{}", d, t)
    });
  });

  view! {
    <div class="space-y-0">
      <SingleDatePicker selected=date_str />
      <div class="flex items-center gap-2 border-t px-1 pt-2">
        <span class="text-xs text-muted-foreground">"Time"</span>
        <input
          type="time"
          prop:value=move || time_str.get()
          on:input=move |ev| time_str.set(event_target_value(&ev))
          class="rounded-md border border-input bg-background px-2 py-1 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-ring"
        />
      </div>
    </div>
  }
}

fn next_id() -> String {
  static COUNTER: AtomicU32 = AtomicU32::new(0);
  COUNTER.fetch_add(1, Ordering::Relaxed).to_string()
}

#[derive(Clone)]
struct SlotDraft {
  id: String,
  label: RwSignal<String>,
  session_type: RwSignal<SessionType>,
  start_at: RwSignal<String>,
  end_at: RwSignal<String>,
}

fn new_slot(label: &str, session_type: SessionType) -> SlotDraft {
  SlotDraft {
    id: next_id(),
    label: RwSignal::new(label.to_string()),
    session_type: RwSignal::new(session_type),
    start_at: RwSignal::new(String::new()),
    end_at: RwSignal::new(String::new()),
  }
}

#[component]
pub fn EventCreatePage() -> impl IntoView {
  let state = expect_context::<EventState>();
  let navigate = use_navigate();

  let title: RwSignal<String> = RwSignal::new(String::new());
  let event_type: RwSignal<EventType> = RwSignal::new(EventType::default());
  let start_date: RwSignal<String> = RwSignal::new(String::new());
  let end_date: RwSignal<String> = RwSignal::new(String::new());

  let slots: RwSignal<Vec<SlotDraft>> = RwSignal::new(vec![
    new_slot("Free Practice", SessionType::FreePractice),
    new_slot("Qualifying", SessionType::Qualifying),
    new_slot("Race", SessionType::Race),
  ]);

  let add_slot = move |_| {
    slots.update(|v| v.push(new_slot("", SessionType::Race)));
  };

  let on_submit = move |_| {
    let t = title.get();
    if t.trim().is_empty() {
      state.error.set(Some("Title is required.".to_string()));
      return;
    }

    let built_slots: Vec<ScheduleSlot> = slots
      .get()
      .iter()
      .map(|d| ScheduleSlot {
        id: d.id.clone(),
        label: d.label.get(),
        session_type: d.session_type.get(),
        start_at: d.start_at.get(),
        end_at: d.end_at.get(),
        session: None,
      })
      .collect();

    let event = Event {
      title: t,
      event_type: event_type.get(),
      start_date: start_date.get(),
      end_date: end_date.get(),
      slots: built_slots,
    };

    state.event.set(Some(event));
    state.slot_sessions.update(|m| m.clear());
    state.error.set(None);
    navigate("/event/view", Default::default());
  };

  view! {
    <main class="container mx-auto max-w-2xl px-4 py-8">
      <h1 class="text-3xl font-bold text-center mb-8">"New Event"</h1>

      {move || state.error.get().map(|e| view! {
        <div class="bg-destructive/10 border border-destructive text-destructive px-4 py-3 rounded-md mb-6">{e}</div>
      })}

      <div class="rounded-lg border bg-card p-6 shadow-sm space-y-6">

        // ── Title ─────────────────────────────────────────────────────────────
        <div class="space-y-1">
          <p class="text-sm font-medium">"Title"</p>
          <input
            type="text"
            placeholder="e.g. Kyalami Round 3"
            prop:value=move || title.get()
            on:input=move |ev| title.set(event_target_value(&ev))
            class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
          />
        </div>

        // ── Event Type ────────────────────────────────────────────────────────
        <div class="space-y-1">
          <p class="text-sm font-medium">"Event Type"</p>
          <select
            on:change=move |ev| {
              let et = match event_target_value(&ev).as_str() {
                "TestDay" => EventType::TestDay,
                "FreePractice" => EventType::FreePractice,
                "Championship" => EventType::Championship,
                _ => EventType::RaceWeekend,
              };
              event_type.set(et);
            }
            class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-ring"
          >
            <option value="RaceWeekend" selected=move || event_type.get() == EventType::RaceWeekend>"Race Weekend"</option>
            <option value="TestDay" selected=move || event_type.get() == EventType::TestDay>"Test Day"</option>
            <option value="FreePractice" selected=move || event_type.get() == EventType::FreePractice>"Free Practice"</option>
            <option value="Championship" selected=move || event_type.get() == EventType::Championship>"Championship"</option>
          </select>
        </div>

        // ── Dates ─────────────────────────────────────────────────────────────
        <div class="grid grid-cols-2 gap-4">
          <div class="space-y-2">
            <p class="text-sm font-medium">"Start Date"</p>
            <SingleDatePicker selected=start_date />
            <p class="text-xs text-muted-foreground">
              {move || if start_date.get().is_empty() { "No date selected".to_string() } else { start_date.get() }}
            </p>
          </div>
          <div class="space-y-2">
            <p class="text-sm font-medium">"End Date"</p>
            <SingleDatePicker selected=end_date />
            <p class="text-xs text-muted-foreground">
              {move || if end_date.get().is_empty() { "No date selected".to_string() } else { end_date.get() }}
            </p>
          </div>
        </div>

        // ── Schedule Slots ────────────────────────────────────────────────────
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <p class="text-sm font-medium">"Schedule"</p>
            <button
              on:click=add_slot
              class="text-xs rounded-md border border-input bg-background px-2 py-1 hover:bg-muted transition-colors"
            >
              "+ Add slot"
            </button>
          </div>

          <For
            each=move || slots.get()
            key=|s| s.id.clone()
            children=move |slot| {
              let slot_id = slot.id.clone();
              let remove = move |_| {
                let id = slot_id.clone();
                slots.update(|v| v.retain(|s| s.id != id));
              };
              view! {
                <div class="rounded-md border bg-card/50 p-3 space-y-3">
                  <div class="grid grid-cols-[1fr_auto_auto] gap-2 items-center">
                    <input
                      type="text"
                      placeholder="Label (e.g. Free Practice)"
                      prop:value=move || slot.label.get()
                      on:input=move |ev| slot.label.set(event_target_value(&ev))
                      class="rounded-md border border-input bg-background px-2 py-1.5 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
                    />
                    <select
                      on:change=move |ev| {
                        let st = match event_target_value(&ev).as_str() {
                          "Q" => SessionType::Qualifying,
                          "R" => SessionType::Race,
                          _ => SessionType::FreePractice,
                        };
                        slot.session_type.set(st);
                      }
                      class="rounded-md border border-input bg-background px-2 py-1.5 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-ring"
                    >
                      <option value="FP" selected=move || slot.session_type.get() == SessionType::FreePractice>"FP"</option>
                      <option value="Q" selected=move || slot.session_type.get() == SessionType::Qualifying>"Q"</option>
                      <option value="R" selected=move || slot.session_type.get() == SessionType::Race>"Race"</option>
                    </select>
                    <button
                      on:click=remove
                      class="text-muted-foreground hover:text-destructive transition-colors px-1"
                    >
                      "✕"
                    </button>
                  </div>
                  <div class="grid grid-cols-2 gap-4">
                    <div class="space-y-1">
                      <p class="text-xs text-muted-foreground">"Start"</p>
                      <SingleDateTimePicker selected=slot.start_at />
                    </div>
                    <div class="space-y-1">
                      <p class="text-xs text-muted-foreground">"End"</p>
                      <SingleDateTimePicker selected=slot.end_at />
                    </div>
                  </div>
                </div>
              }
            }
          />
        </div>

        <button
          on:click=on_submit
          class="w-full inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
        >
          "Create Event"
        </button>
      </div>

      <SavedEvents />
    </main>
  }
}

#[component]
fn SavedEvents() -> impl IntoView {
  #[cfg(any(feature = "csr", feature = "hydrate"))]
  {
    use crate::common::db::EventSummary;

    let state = expect_context::<EventState>();
    let events: RwSignal<Vec<EventSummary>> = RwSignal::new(Vec::new());
    let load_error: RwSignal<Option<String>> = RwSignal::new(None);

    Effect::new(move || {
      leptos::task::spawn_local(async move {
        match crate::common::db::list_events().await {
          Ok(list) => events.set(list),
          Err(e) => load_error.set(Some(e)),
        }
      });
    });

    let refresh = move || {
      leptos::task::spawn_local(async move {
        match crate::common::db::list_events().await {
          Ok(list) => events.set(list),
          Err(e) => load_error.set(Some(e)),
        }
      });
    };

    let navigate_stored = StoredValue::new(use_navigate());

    view! {
      <div class="mt-10">
        <h2 class="text-xl font-semibold mb-4">"Saved Events"</h2>

        {move || load_error.get().map(|e| view! {
          <div class="bg-destructive/10 border border-destructive text-destructive px-4 py-3 rounded-md mb-4">{e}</div>
        })}

        <Show
          when=move || !events.get().is_empty()
          fallback=|| view! {
            <p class="text-muted-foreground text-sm">"No saved events found."</p>
          }
        >
          <div class="rounded-md border divide-y">
            <For
              each=move || events.get()
              key=|e| e.id.clone()
              children=move |summary| {
                let id_load = summary.id.clone();
                let id_delete = summary.id.clone();

                let on_load = move |_| {
                  let id = id_load.clone();
                  let nav = navigate_stored.get_value();
                  leptos::task::spawn_local(async move {
                    match crate::common::db::get_event(&id).await {
                      Ok(Some(event)) => {
                        state.event.set(Some(event));
                        state.slot_sessions.update(|m| m.clear());
                        state.error.set(None);
                        nav("/event/view", Default::default());
                      }
                      Ok(None) => state.error.set(Some("Event not found in database.".to_string())),
                      Err(e) => state.error.set(Some(format!("Failed to load: {e}"))),
                    }
                  });
                };

                let on_delete = move |_| {
                  let id = id_delete.clone();
                  leptos::task::spawn_local(async move {
                    match crate::common::db::delete_event(&id).await {
                      Ok(()) => refresh(),
                      Err(e) => load_error.set(Some(format!("Failed to delete: {e}"))),
                    }
                  });
                };

                view! {
                  <div class="flex items-center justify-between px-4 py-3">
                    <div class="flex flex-col gap-0.5">
                      <span class="font-medium text-sm">{summary.title.clone()}</span>
                      <span class="text-xs text-muted-foreground">
                        {summary.event_type.clone()}
                        " · "
                        {summary.start_date.clone()}
                        " – "
                        {summary.end_date.clone()}
                        " · "
                        {summary.uploaded_count}
                        "/"
                        {summary.slot_count}
                        " uploaded"
                      </span>
                    </div>
                    <div class="flex items-center gap-2">
                      <button
                        class="inline-flex items-center rounded-md bg-primary px-3 py-1.5 text-xs font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
                        on:click=on_load
                      >
                        "Load"
                      </button>
                      <button
                        class="inline-flex items-center rounded-md bg-destructive px-3 py-1.5 text-xs font-medium text-destructive-foreground shadow-sm transition-colors hover:bg-destructive/90"
                        on:click=on_delete
                      >
                        "Delete"
                      </button>
                    </div>
                  </div>
                }
              }
            />
          </div>
        </Show>
      </div>
    }.into_any()
  }

  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  {
    view! { <div /> }.into_any()
  }
}
