use acc_session_viewer_lib::event::{Championship, Event, EventSummary, EventType};
use acc_tools_ui_components::components::ui::date_picker::{
  DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow,
  DatePickerTable, DatePickerTitle, DatePickerWeekDay,
};
use leptos::prelude::*;
use leptos_router::hooks::use_navigate;
use time::{Date, Month};

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

  // Sentinel – won't match any visible calendar date
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
              .map(|week| {
                view! {
                  <DatePickerRow>
                    {week
                      .into_iter()
                      .map(|d| {
                        let day = d.day;
                        let disabled = d.disabled;
                        let on_click = move |day: u8| {
                          if disabled {
                            return;
                          }
                          let Ok(new_date) = Date::from_calendar_date(year, month, day) else {
                            return;
                          };
                          match (start_date.get(), end_date.get()) {
                            (None, _) | (_, None) => {
                              start_date.set(Some(new_date));
                              end_date.set(Some(new_date));
                            }
                            (Some(s), Some(e)) => {
                              let ds = (new_date - s).whole_days().abs();
                              let de = (new_date - e).whole_days().abs();
                              if ds <= de {
                                start_date.set(Some(new_date));
                              } else {
                                end_date.set(Some(new_date));
                              }
                              let s2 = start_date.get().unwrap_or(new_date);
                              let e2 = end_date.get().unwrap_or(new_date);
                              if s2 > e2 {
                                start_date.set(Some(e2));
                                end_date.set(Some(s2));
                              }
                            }
                          }
                        };
                        view! {
                          <DatePickerCell
                            day=day
                            year=year
                            month=month
                            disabled=disabled
                            start_date=start_sig
                            end_date=end_sig
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
    </DatePicker>
  }
}

// ── EventCreateDialog ─────────────────────────────────────────────────────────

#[component]
fn EventCreateDialog(open: RwSignal<bool>, refresh: RwSignal<u64>) -> impl IntoView {
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

    let event = Event {
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

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    leptos::task::spawn_local(async move {
      match crate::common::db::save_event(&event).await {
        Ok(_) => {
          open.set(false);
          refresh.update(|n| *n += 1);
        }
        Err(e) => error.set(Some(format!("Failed to save: {e}"))),
      }
    });
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    let _ = (event, refresh);
  };

  view! {
    // Backdrop
    <div class="fixed inset-0 z-40 bg-black/50" on:click=move |_| open.set(false) />

    // Dialog panel
    <div
      class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-50 bg-background border rounded-2xl shadow-lg p-6 w-full max-w-lg space-y-6"
      on:click=|ev| ev.stop_propagation()
    >
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold">"New Event"</h2>
        <button
          on:click=move |_| open.set(false)
          class="rounded-md p-1 text-muted-foreground hover:text-foreground hover:bg-muted transition-colors"
        >
          "✕"
        </button>
      </div>

      {move || {
        error.get().map(|e| {
          view! {
            <div class="bg-destructive/10 border border-destructive text-destructive px-4 py-3 rounded-md text-sm">
              {e}
            </div>
          }
        })
      }}

      // Title
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

      // Event Type
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
          <option value="RaceWeekend" selected=move || event_type.get() == EventType::RaceWeekend>
            "Race Weekend"
          </option>
          <option value="TestDay" selected=move || event_type.get() == EventType::TestDay>
            "Test Day"
          </option>
          <option value="TrackDay" selected=move || event_type.get() == EventType::TrackDay>
            "Track Day"
          </option>
          <option
            value="FreePractice"
            selected=move || event_type.get() == EventType::FreePractice
          >
            "Free Practice"
          </option>
        </select>
      </div>

      // Date range picker
      <div class="space-y-2">
        <p class="text-sm font-medium">"Event Dates"</p>
        <RangeDatePicker start_date=start_date_opt end_date=end_date_opt />
        <p class="text-xs text-muted-foreground">
          {move || match (start_date_opt.get(), end_date_opt.get()) {
            (Some(s), Some(e)) if s == e => s.format(fmt).unwrap_or_default(),
            (Some(s), Some(e)) => {
              format!(
                "{} – {}",
                s.format(fmt).unwrap_or_default(),
                e.format(fmt).unwrap_or_default(),
              )
            }
            _ => "No dates selected".to_string(),
          }}
        </p>
      </div>

      <button
        on:click=on_submit
        class="w-full inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
      >
        "Create Event"
      </button>
    </div>
  }
}

// ── EventListPage ─────────────────────────────────────────────────────────────

// ── ChampionshipCreateDialog ───────────────────────────────────────────────────

#[component]
fn ChampionshipCreateDialog(open: RwSignal<bool>, refresh: RwSignal<u64>) -> impl IntoView {
  let title: RwSignal<String> = RwSignal::new(String::new());
  let error: RwSignal<Option<String>> = RwSignal::new(None);

  let on_submit = move |_| {
    let t = title.get();
    if t.trim().is_empty() {
      error.set(Some("Title is required.".to_string()));
      return;
    }
    let championship = Championship {
      title: t,
      events: vec![],
    };

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    leptos::task::spawn_local(async move {
      match crate::common::db::save_championship(&championship).await {
        Ok(_) => {
          open.set(false);
          refresh.update(|n| *n += 1);
        }
        Err(e) => error.set(Some(format!("Failed to save: {e}"))),
      }
    });
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    let _ = (championship, refresh);
  };

  view! {
    <div class="fixed inset-0 z-40 bg-black/50" on:click=move |_| open.set(false) />
    <div
      class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-50 bg-background border rounded-2xl shadow-lg p-6 w-full max-w-sm space-y-6"
      on:click=|ev| ev.stop_propagation()
    >
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold">"New Championship"</h2>
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
          placeholder="e.g. GT World Challenge 2026"
          prop:value=move || title.get()
          on:input=move |ev| title.set(event_target_value(&ev))
          class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
        />
      </div>

      <button
        on:click=on_submit
        class="w-full inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
      >
        "Create Championship"
      </button>
    </div>
  }
}

// ── EventListPage ─────────────────────────────────────────────────────────────

#[component]
pub fn EventListPage() -> impl IntoView {
  let event_state = expect_context::<EventState>();
  let championship_state = expect_context::<ChampionshipState>();
  let navigate_sv = StoredValue::new(use_navigate());
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let _ = (event_state, championship_state, navigate_sv);

  let event_open: RwSignal<bool> = RwSignal::new(false);
  let champ_open: RwSignal<bool> = RwSignal::new(false);
  let events: RwSignal<Vec<EventSummary>> = RwSignal::new(vec![]);
  let championships: RwSignal<Vec<acc_session_viewer_lib::event::ChampionshipSummary>> =
    RwSignal::new(vec![]);
  let load_error: RwSignal<Option<String>> = RwSignal::new(None);
  let refresh: RwSignal<u64> = RwSignal::new(0);

  Effect::new(move |_| {
    let _ = refresh.get();
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    leptos::task::spawn_local(async move {
      let (ev_res, ch_res) = futures::join!(
        crate::common::db::list_events(),
        crate::common::db::list_championships(),
      );
      match ev_res {
        Ok(list) => events.set(list),
        Err(e) => load_error.set(Some(e)),
      }
      match ch_res {
        Ok(list) => championships.set(list),
        Err(e) => load_error.set(Some(e)),
      }
    });
  });

  view! {
    <main class="container mx-auto max-w-4xl px-4 py-8 space-y-8">

      {move || load_error.get().map(|e| view! {
        <div class="bg-destructive/10 border border-destructive text-destructive px-4 py-3 rounded-md text-sm">{e}</div>
      })}

      // ── Championships ─────────────────────────────────────────────────────
      <section class="space-y-4">
        <div class="flex items-center justify-between">
          <h2 class="text-2xl font-bold">"Championships"</h2>
          <button
            on:click=move |_| champ_open.set(true)
            class="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
          >
            "+ New Championship"
          </button>
        </div>

        <Show
          when=move || !championships.get().is_empty()
          fallback=|| view! {
            <p class="text-muted-foreground text-sm">"No championships yet."</p>
          }
        >
          <div class="rounded-md border divide-y">
            <For
              each=move || championships.get()
              key=|c| c.id.clone()
              children=move |summary| {
                let id_sv = StoredValue::new(summary.id.clone());
                #[cfg(not(any(feature = "csr", feature = "hydrate")))]
                let _ = id_sv;

                let on_open = move |_| {
                  #[cfg(any(feature = "csr", feature = "hydrate"))]
                  {
                    let id = id_sv.get_value();
                    let nav = navigate_sv.get_value();
                    leptos::task::spawn_local(async move {
                      match crate::common::db::get_championship(&id).await {
                        Ok(Some(c)) => {
                          championship_state.championship.set(Some((id, c)));
                          nav("/championship/view", Default::default());
                        }
                        Ok(None) => load_error.set(Some("Championship not found.".to_string())),
                        Err(e) => load_error.set(Some(format!("Failed to load: {e}"))),
                      }
                    });
                  }
                };

                let n = summary.event_count;
                view! {
                  <div
                    class="flex items-center justify-between px-4 py-3 cursor-pointer hover:bg-muted/50 transition-colors"
                    on:click=on_open
                  >
                    <div class="flex flex-col gap-0.5">
                      <span class="font-medium text-sm">{summary.title.clone()}</span>
                      <span class="text-xs text-muted-foreground">
                        {n}" event"{if n == 1 { "" } else { "s" }}
                      </span>
                    </div>
                    <button
                      class="inline-flex items-center rounded-md bg-destructive px-3 py-1.5 text-xs font-medium text-destructive-foreground shadow-sm transition-colors hover:bg-destructive/90"
                      on:click=move |ev| {
                        ev.stop_propagation();
                        #[cfg(any(feature = "csr", feature = "hydrate"))]
                        {
                          let id = id_sv.get_value();
                          leptos::task::spawn_local(async move {
                            match crate::common::db::delete_championship(&id).await {
                              Ok(()) => refresh.update(|n| *n += 1),
                              Err(e) => load_error.set(Some(format!("Delete failed: {e}"))),
                            }
                          });
                        }
                      }
                    >
                      "Delete"
                    </button>
                  </div>
                }
              }
            />
          </div>
        </Show>
      </section>

      // ── Standalone events ─────────────────────────────────────────────────
      <section class="space-y-4">
        <div class="flex items-center justify-between">
          <h2 class="text-2xl font-bold">"Events"</h2>
          <button
            on:click=move |_| event_open.set(true)
            class="inline-flex items-center gap-2 rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
          >
            "+ New Event"
          </button>
        </div>

        <Show
          when=move || !events.get().is_empty()
          fallback=|| view! {
            <p class="text-muted-foreground text-sm">"No standalone events yet."</p>
          }
        >
          <div class="rounded-md border divide-y">
            <For
              each=move || events.get()
              key=|e| e.id.clone()
              children=move |summary| {
                let id_sv = StoredValue::new(summary.id.clone());
                #[cfg(not(any(feature = "csr", feature = "hydrate")))]
                let _ = id_sv;

                let on_open = move |_| {
                  #[cfg(any(feature = "csr", feature = "hydrate"))]
                  {
                    let id = id_sv.get_value();
                    let nav = navigate_sv.get_value();
                    leptos::task::spawn_local(async move {
                      match crate::common::db::get_event(&id).await {
                        Ok(Some(event)) => {
                          event_state.event.set(Some(event));
                          event_state.slot_sessions.update(|m| m.clear());
                          event_state.error.set(None);
                          event_state.championship_ctx.set(None);
                          nav("/event/view", Default::default());
                        }
                        Ok(None) => load_error.set(Some("Event not found.".to_string())),
                        Err(e) => load_error.set(Some(format!("Failed to load: {e}"))),
                      }
                    });
                  }
                };

                let date_label = if summary.start_date.is_empty() {
                  "No date".to_string()
                } else if summary.start_date == summary.end_date {
                  summary.start_date.clone()
                } else {
                  format!("{} – {}", summary.start_date, summary.end_date)
                };

                view! {
                  <div
                    class="flex items-center justify-between px-4 py-3 cursor-pointer hover:bg-muted/50 transition-colors"
                    on:click=on_open
                  >
                    <div class="flex flex-col gap-0.5">
                      <span class="font-medium text-sm">{summary.title.clone()}</span>
                      <span class="text-xs text-muted-foreground">
                        {summary.event_type.clone()}" · "{date_label}" · "
                        {summary.slot_count}" session"{if summary.slot_count == 1 { "" } else { "s" }}
                      </span>
                    </div>
                    <button
                      class="inline-flex items-center rounded-md bg-destructive px-3 py-1.5 text-xs font-medium text-destructive-foreground shadow-sm transition-colors hover:bg-destructive/90"
                      on:click=move |ev| {
                        ev.stop_propagation();
                        #[cfg(any(feature = "csr", feature = "hydrate"))]
                        {
                          let id = id_sv.get_value();
                          leptos::task::spawn_local(async move {
                            match crate::common::db::delete_event(&id).await {
                              Ok(()) => refresh.update(|n| *n += 1),
                              Err(e) => load_error.set(Some(format!("Delete failed: {e}"))),
                            }
                          });
                        }
                      }
                    >
                      "Delete"
                    </button>
                  </div>
                }
              }
            />
          </div>
        </Show>
      </section>

      <Show when=move || event_open.get()>
        <EventCreateDialog open=event_open refresh=refresh />
      </Show>
      <Show when=move || champ_open.get()>
        <ChampionshipCreateDialog open=champ_open refresh=refresh />
      </Show>
    </main>
  }
}
