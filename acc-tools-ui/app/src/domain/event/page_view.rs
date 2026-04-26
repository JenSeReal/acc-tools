use std::sync::atomic::{AtomicU32, Ordering};

use acc_session_viewer_lib::car::car_model::CarModel;
use acc_session_viewer_lib::event::ScheduleSlot;
use acc_session_viewer_lib::loaded_session::LoadedSession;
use acc_session_viewer_lib::session_type::model::SessionType;
use acc_tools_ui_components::components::ui::date_picker::{
  DatePicker, DatePickerCell, DatePickerHeader, DatePickerNavButton, DatePickerRow,
  DatePickerTable, DatePickerTitle, DatePickerWeekDay,
};
use acc_tools_ui_components::{CarSelector, DriverStatsTable, FileUpload, SessionInfo};
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

// ── SingleDatePicker ──────────────────────────────────────────────────────────

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
  let fallback = Date::from_calendar_date(init_year, init_month, 1).unwrap_or_else(|_| now.date());
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
                            day=day year=year month=month disabled=disabled
                            start_date=selected_date end_date=selected_date
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

// ── SingleDateTimePicker ──────────────────────────────────────────────────────

#[component]
fn SingleDateTimePicker(selected: RwSignal<String>, label: &'static str) -> impl IntoView {
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
    <div class="space-y-1">
      <p class="text-xs font-medium text-muted-foreground">{label}</p>
      <SingleDatePicker selected=date_str />
      <div class="flex items-center gap-2 border-t px-1 pt-2 mt-1">
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

// ── AddSessionDialog ──────────────────────────────────────────────────────────

fn next_slot_id() -> String {
  static COUNTER: AtomicU32 = AtomicU32::new(0);
  format!("slot-{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

#[component]
fn AddSessionDialog(open: RwSignal<bool>, state: EventState) -> impl IntoView {
  let label: RwSignal<String> = RwSignal::new(String::new());
  let description: RwSignal<String> = RwSignal::new(String::new());
  let session_type: RwSignal<SessionType> = RwSignal::new(SessionType::FreePractice);
  let start_at: RwSignal<String> = RwSignal::new(String::new());
  let end_at: RwSignal<String> = RwSignal::new(String::new());
  let error: RwSignal<Option<String>> = RwSignal::new(None);

  let on_submit = move |_| {
    let lbl = label.get();
    if lbl.trim().is_empty() {
      error.set(Some("Title is required.".to_string()));
      return;
    }

    let new_slot = ScheduleSlot {
      id: next_slot_id(),
      label: lbl,
      description: description.get(),
      session_type: session_type.get(),
      start_at: start_at.get(),
      end_at: end_at.get(),
      session: None,
    };

    state.event.update(|opt| {
      if let Some(e) = opt {
        e.slots.push(new_slot);
      }
    });

    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
      let Some(event) = state.event.get() else {
        return;
      };
      leptos::task::spawn_local(async move {
        let _ = crate::common::db::save_event(&event).await;
      });
    }

    // Reset fields and close
    label.set(String::new());
    description.set(String::new());
    session_type.set(SessionType::FreePractice);
    start_at.set(String::new());
    end_at.set(String::new());
    error.set(None);
    open.set(false);
  };

  view! {
    <div class="fixed inset-0 z-40 bg-black/50" on:click=move |_| open.set(false) />
    <div
      class="fixed top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 z-50 bg-background border rounded-2xl shadow-lg p-6 w-full max-w-2xl max-h-[90vh] overflow-y-auto space-y-6"
      on:click=|ev| ev.stop_propagation()
    >
      <div class="flex items-center justify-between">
        <h2 class="text-xl font-semibold">"Add Session"</h2>
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

      // Title
      <div class="space-y-1">
        <p class="text-sm font-medium">"Title"</p>
        <input
          type="text"
          placeholder="e.g. Free Practice 1"
          prop:value=move || label.get()
          on:input=move |ev| label.set(event_target_value(&ev))
          class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring"
        />
      </div>

      // Description
      <div class="space-y-1">
        <p class="text-sm font-medium">"Description"</p>
        <textarea
          placeholder="Optional notes about this session"
          prop:value=move || description.get()
          on:input=move |ev| description.set(event_target_value(&ev))
          rows="2"
          class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus:outline-none focus:ring-2 focus:ring-ring resize-none"
        />
      </div>

      // Session Type
      <div class="space-y-1">
        <p class="text-sm font-medium">"Session Type"</p>
        <select
          on:change=move |ev| {
            session_type.set(match event_target_value(&ev).as_str() {
              "Q" => SessionType::Qualifying,
              "R" => SessionType::Race,
              _ => SessionType::FreePractice,
            });
          }
          class="w-full rounded-md border border-input bg-background px-3 py-2 text-sm shadow-sm focus:outline-none focus:ring-2 focus:ring-ring"
        >
          <option value="FP" selected=move || session_type.get() == SessionType::FreePractice>
            "Free Practice"
          </option>
          <option value="Q" selected=move || session_type.get() == SessionType::Qualifying>
            "Qualifying"
          </option>
          <option value="R" selected=move || session_type.get() == SessionType::Race>
            "Race"
          </option>
        </select>
      </div>

      // Date/time pickers
      <div class="grid grid-cols-2 gap-6">
        <SingleDateTimePicker selected=start_at label="Start" />
        <SingleDateTimePicker selected=end_at label="End" />
      </div>

      <button
        on:click=on_submit
        class="w-full inline-flex items-center justify-center rounded-md bg-primary px-4 py-2 text-sm font-medium text-primary-foreground shadow-sm transition-colors hover:bg-primary/90"
      >
        "Add Session"
      </button>
    </div>
  }
}

// ── EventViewPage ─────────────────────────────────────────────────────────────

#[component]
pub fn EventViewPage() -> impl IntoView {
  let state = expect_context::<EventState>();
  let cs = expect_context::<ChampionshipState>();
  #[cfg(not(any(feature = "csr", feature = "hydrate")))]
  let _ = cs;

  Effect::new(move || {
    if state.event.get().is_none() {
      let navigate = use_navigate();
      navigate("/event", Default::default());
    }
  });

  let add_open: RwSignal<bool> = RwSignal::new(false);
  let save_status: RwSignal<Option<String>> = RwSignal::new(None);

  // Back target depends on whether we came from a championship.
  let on_back = move |_| {
    let ctx = state.championship_ctx.get();
    state.event.set(None);
    state.slot_sessions.update(|m| m.clear());
    state.error.set(None);
    state.championship_ctx.set(None);
    let navigate = use_navigate();
    if ctx.is_some() {
      navigate("/championship/view", Default::default());
    } else {
      navigate("/event", Default::default());
    }
  };

  let on_save = move |_| {
    #[cfg(any(feature = "csr", feature = "hydrate"))]
    {
      let Some(mut event) = state.event.get() else {
        return;
      };
      let sessions = state.slot_sessions.get();
      for slot in &mut event.slots {
        slot.session = sessions.get(&slot.id).cloned();
      }
      save_status.set(Some("Saving…".to_string()));

      if let Some(ctx) = state.championship_ctx.get() {
        // Save back into the championship record.
        cs.championship.update(|opt| {
          if let Some((_, c)) = opt
            && let Some(e) = c.events.get_mut(ctx.event_idx)
          {
            *e = event.clone();
          }
        });
        if let Some((champ_id, championship)) = cs.championship.get() {
          leptos::task::spawn_local(async move {
            let _ = crate::common::db::delete_championship(&champ_id).await;
            match crate::common::db::save_championship(&championship).await {
              Ok(new_id) => {
                cs.championship.update(|opt| {
                  if let Some((id, _)) = opt {
                    *id = new_id;
                  }
                });
                save_status.set(Some("Saved!".to_string()));
              }
              Err(e) => save_status.set(Some(format!("Error: {e}"))),
            }
          });
        }
      } else {
        leptos::task::spawn_local(async move {
          match crate::common::db::save_event(&event).await {
            Ok(_id) => save_status.set(Some("Saved!".to_string())),
            Err(e) => save_status.set(Some(format!("Error: {e}"))),
          }
        });
      }
    }
    #[cfg(not(any(feature = "csr", feature = "hydrate")))]
    save_status.set(Some("Save not available in SSR mode.".to_string()));
  };

  view! {
    <main class="container mx-auto max-w-6xl px-4 py-8 space-y-8">
      <Show when=move || state.event.get().is_some()>

        // ── Header ────────────────────────────────────────────────────────────
        <div class="flex items-center justify-between">
          <div>
            <h1 class="text-2xl font-bold">
              {move || state.event.get().map(|e| e.title.clone()).unwrap_or_default()}
            </h1>
            <p class="text-sm text-muted-foreground mt-0.5">
              {move || state.event.get().map(|e| {
                format!("{} · {} – {}", e.event_type.label(), e.start_date, e.end_date)
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
              "+ Add Session"
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

        // ── Slots ─────────────────────────────────────────────────────────────
        <div class="space-y-8">
          <For
            each=move || {
              state.event.get().map(|e| {
                let mut s = e.slots.clone();
                s.sort_by(|a, b| a.start_at.cmp(&b.start_at));
                s
              }).unwrap_or_default()
            }
            key=|s| s.id.clone()
            children=move |s| view! { <SlotView schedule_slot=s state=state /> }
          />
          <Show when=move || state.event.get().map(|e| e.slots.is_empty()).unwrap_or(true)>
            <p class="text-sm text-muted-foreground">
              "No sessions yet. Click \"+ Add Session\" to create one."
            </p>
          </Show>
        </div>

        // ── Add Session dialog ────────────────────────────────────────────────
        <Show when=move || add_open.get()>
          <AddSessionDialog open=add_open state=state />
        </Show>

      </Show>
    </main>
  }
}

// ── SlotView ──────────────────────────────────────────────────────────────────

#[component]
fn SlotView(schedule_slot: ScheduleSlot, state: EventState) -> impl IntoView {
  let slot_id = schedule_slot.id.clone();

  let session: RwSignal<Option<LoadedSession>> = RwSignal::new(schedule_slot.session.clone());
  let car_model: RwSignal<Option<CarModel>> = RwSignal::new(None);
  let error_write = state.error.write_only();

  let time_label = if schedule_slot.start_at.is_empty() {
    String::new()
  } else if schedule_slot.end_at.is_empty() {
    format!(" · {}", schedule_slot.start_at.replace('T', " "))
  } else {
    format!(
      " · {} – {}",
      schedule_slot.start_at.replace('T', " "),
      schedule_slot.end_at.replace('T', " "),
    )
  };

  let type_label = format!("{:?}{}", schedule_slot.session_type, time_label);

  view! {
    <section class="space-y-4">
      <div class="flex items-baseline gap-2 border-b pb-2">
        <h2 class="text-xl font-semibold">{schedule_slot.label.clone()}</h2>
        <span class="text-sm text-muted-foreground">{type_label}</span>
      </div>
      {(!schedule_slot.description.is_empty()).then(|| view! {
        <p class="text-sm text-muted-foreground">{schedule_slot.description.clone()}</p>
      })}

      <Show
        when=move || session.get().is_some()
        fallback=move || {
          let slot_id_inner = slot_id.clone();
          let on_loaded = move |loaded: LoadedSession| {
            session.set(Some(loaded.clone()));
            state.slot_sessions.update(|m| { m.insert(slot_id_inner.clone(), loaded); });
          };
          view! {
            <div class="flex items-center gap-3">
              <FileUpload on_session_loaded=on_loaded on_error=error_write />
              <span class="text-sm text-muted-foreground">"No session uploaded yet"</span>
            </div>
          }
        }
      >
        <SessionInfo session=session.read_only() />
        <CarSelector session=session.read_only() selected_car_model=car_model.write_only() />
        <DriverStatsTable session=session.read_only() selected_car_model=car_model.read_only() />
      </Show>
    </section>
  }
}
