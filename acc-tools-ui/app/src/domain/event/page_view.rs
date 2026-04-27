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

// ── ClockTimePicker ───────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum ClockMode {
  Hour,
  Minute,
}

fn clock_pos(index: usize, r: f64) -> (f64, f64) {
  let angle = 2.0 * std::f64::consts::PI * index as f64 / 12.0 - std::f64::consts::FRAC_PI_2;
  (120.0 + r * angle.cos(), 120.0 + r * angle.sin())
}

#[component]
fn ClockTimePicker(value: RwSignal<String>) -> impl IntoView {
  let parse_hm = |s: &str| -> (u8, u8) {
    let mut parts = s.splitn(2, ':');
    let h = parts.next().and_then(|v| v.parse::<u8>().ok()).unwrap_or(0);
    let m = parts.next().and_then(|v| v.parse::<u8>().ok()).unwrap_or(0);
    (h, m)
  };
  let (h0, m0) = parse_hm(&value.get_untracked());
  let is_pm: RwSignal<bool> = RwSignal::new(h0 >= 12);
  let hour: RwSignal<u8> = RwSignal::new(h0 % 12); // 0 = 12 o'clock
  let minute: RwSignal<u8> = RwSignal::new(m0);
  let mode: RwSignal<ClockMode> = RwSignal::new(ClockMode::Hour);

  // Sync to "HH:MM" 24h output
  Effect::new(move |_| {
    let h12 = hour.get();
    let m = minute.get();
    let h24: u8 = match (h12, is_pm.get()) {
      (0, false) => 0,
      (0, true) => 12,
      (h, false) => h,
      (h, true) => h + 12,
    };
    value.set(format!("{:02}:{:02}", h24, m));
  });

  view! {
    <div class="flex flex-col items-center gap-3 select-none">

      // ── Time display ───────────────────────────────────────────────────────
      <div class="flex items-center gap-1.5 rounded-2xl bg-muted/50 px-4 py-3">
        <button
          class=move || if mode.get() == ClockMode::Hour {
            "w-14 text-4xl font-light rounded-xl bg-primary/15 text-primary text-center"
          } else {
            "w-14 text-4xl font-light rounded-xl hover:bg-muted text-muted-foreground text-center"
          }
          on:click=move |_| mode.set(ClockMode::Hour)
        >
          {move || format!("{:02}", if hour.get() == 0 { 12u8 } else { hour.get() })}
        </button>
        <span class="text-4xl font-light text-foreground/40">":"</span>
        <button
          class=move || if mode.get() == ClockMode::Minute {
            "w-14 text-4xl font-light rounded-xl bg-primary/15 text-primary text-center"
          } else {
            "w-14 text-4xl font-light rounded-xl hover:bg-muted text-muted-foreground text-center"
          }
          on:click=move |_| mode.set(ClockMode::Minute)
        >
          {move || format!("{:02}", minute.get())}
        </button>
        <div class="flex flex-col ml-2 rounded-lg border border-input overflow-hidden text-xs font-medium">
          <button
            class=move || if !is_pm.get() {
              "px-2 py-1.5 bg-primary text-primary-foreground"
            } else {
              "px-2 py-1.5 hover:bg-muted"
            }
            on:click=move |_| is_pm.set(false)
          >"AM"</button>
          <div class="border-t border-input" />
          <button
            class=move || if is_pm.get() {
              "px-2 py-1.5 bg-primary text-primary-foreground"
            } else {
              "px-2 py-1.5 hover:bg-muted"
            }
            on:click=move |_| is_pm.set(true)
          >"PM"</button>
        </div>
      </div>

      // ── Clock face ────────────────────────────────────────────────────────
      <svg attr:viewBox="0 0 240 240" class="w-48 h-48">
        // Background
        <circle attr:cx="120" attr:cy="120" attr:r="108" class="fill-muted" />

        // Hand + knobs
        {move || {
          let idx = if mode.get() == ClockMode::Hour {
            hour.get() as usize
          } else {
            (minute.get() / 5) as usize
          };
          let (hx, hy) = clock_pos(idx, 78.0);
          view! {
            <g>
              <line
                attr:x1="120" attr:y1="120"
                attr:x2={hx.to_string()} attr:y2={hy.to_string()}
                class="stroke-primary"
                attr:stroke-width="2"
              />
              <circle attr:cx="120" attr:cy="120" attr:r="4" class="fill-primary" />
              <circle
                attr:cx={hx.to_string()} attr:cy={hy.to_string()}
                attr:r="20"
                class="fill-primary"
              />
            </g>
          }
        }}

        // Numbers (re-render on mode / selection change)
        {move || {
          let m = mode.get();
          let sel = if m == ClockMode::Hour {
            hour.get() as usize
          } else {
            (minute.get() / 5) as usize
          };
          (0usize..12).map(|i| {
            let label = if m == ClockMode::Hour {
              if i == 0 { "12".to_string() } else { i.to_string() }
            } else {
              format!("{:02}", i * 5)
            };
            let (x, y) = clock_pos(i, 78.0);
            let is_sel = sel == i;
            view! {
              <g
                on:click=move |_| {
                  if mode.get() == ClockMode::Hour {
                    hour.set(i as u8);
                    mode.set(ClockMode::Minute);
                  } else {
                    minute.set((i as u8) * 5);
                  }
                }
                class="cursor-pointer"
              >
                <text
                  attr:x={x.to_string()}
                  attr:y={y.to_string()}
                  attr:text-anchor="middle"
                  attr:dominant-baseline="central"
                  attr:font-size="13"
                  class=if is_sel {
                    "fill-primary-foreground font-medium"
                  } else {
                    "fill-foreground"
                  }
                >
                  {label}
                </text>
              </g>
            }
          }).collect_view()
        }}
      </svg>
    </div>
  }
}

// ── SlotSchedulePicker ────────────────────────────────────────────────────────

#[component]
fn SlotSchedulePicker(start_at: RwSignal<String>, end_at: RwSignal<String>) -> impl IntoView {
  use time::macros::format_description;
  let fmt = format_description!("[year]-[month]-[day]");

  let parse_dt = |s: &str| -> (Option<Date>, String) {
    if let Some(i) = s.find('T') {
      (Date::parse(&s[..i], fmt).ok(), s[i + 1..].to_string())
    } else {
      (Date::parse(s, fmt).ok(), String::new())
    }
  };
  let (init_start_date, init_start_time) = parse_dt(&start_at.get_untracked());
  let (init_end_date, init_end_time) = parse_dt(&end_at.get_untracked());

  let now = time::OffsetDateTime::now_utc();
  let sentinel = Date::from_calendar_date(1900, Month::January, 1).unwrap();

  let start_date: RwSignal<Option<Date>> = RwSignal::new(init_start_date);
  let end_date: RwSignal<Option<Date>> = RwSignal::new(init_end_date);
  let start_time: RwSignal<String> = RwSignal::new(init_start_time);
  let end_time: RwSignal<String> = RwSignal::new(init_end_time);

  let display = RwSignal::new(init_start_date.unwrap_or_else(|| now.date()));
  let start_sig = RwSignal::new(init_start_date.unwrap_or(sentinel));
  let end_sig = RwSignal::new(init_end_date.unwrap_or(sentinel));

  // Keep sentinel signals in sync for DatePickerCell rendering
  Effect::new(move |_| start_sig.set(start_date.get().unwrap_or(sentinel)));
  Effect::new(move |_| end_sig.set(end_date.get().unwrap_or(sentinel)));

  // Sync start_at output
  Effect::new(move |_| {
    start_at.set(match start_date.get() {
      None => String::new(),
      Some(d) => {
        let t = start_time.get();
        let ds = d.format(fmt).unwrap_or_default();
        if t.is_empty() {
          ds
        } else {
          format!("{}T{}", ds, t)
        }
      }
    });
  });

  // Sync end_at output
  Effect::new(move |_| {
    end_at.set(match end_date.get() {
      None => String::new(),
      Some(d) => {
        let t = end_time.get();
        let ds = d.format(fmt).unwrap_or_default();
        if t.is_empty() {
          ds
        } else {
          format!("{}T{}", ds, t)
        }
      }
    });
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
    <div class="space-y-3">

      // Range calendar
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
                            if disabled { return; }
                            let Ok(new_date) = Date::from_calendar_date(year, month, day) else { return; };
                            match (start_date.get(), end_date.get()) {
                              (None, _) | (_, None) => {
                                start_date.set(Some(new_date));
                                end_date.set(Some(new_date));
                              }
                              (Some(s), Some(e)) => {
                                let ds = (new_date - s).whole_days().abs();
                                let de = (new_date - e).whole_days().abs();
                                if ds <= de { start_date.set(Some(new_date)); }
                                else { end_date.set(Some(new_date)); }
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
                              day=day year=year month=month disabled=disabled
                              start_date=start_sig end_date=end_sig
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

      <p class="text-xs text-muted-foreground">
        {move || match (start_date.get(), end_date.get()) {
          (Some(s), Some(e)) if s == e => s.format(fmt).unwrap_or_default(),
          (Some(s), Some(e)) => format!(
            "{} – {}",
            s.format(fmt).unwrap_or_default(),
            e.format(fmt).unwrap_or_default(),
          ),
          _ => "No date selected".to_string(),
        }}
      </p>

      // Clock time pickers
      <div class="grid grid-cols-2 gap-4 border-t pt-4">
        <div class="space-y-2">
          <p class="text-xs font-medium text-muted-foreground">"Start time"</p>
          <ClockTimePicker value=start_time />
        </div>
        <div class="space-y-2">
          <p class="text-xs font-medium text-muted-foreground">"End time"</p>
          <ClockTimePicker value=end_time />
        </div>
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
fn AddSessionDialog(
  open: RwSignal<bool>,
  state: EventState,
  toast_msg: RwSignal<Option<String>>,
) -> impl IntoView {
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
    toast_msg.set(Some("Session added successfully.".to_string()));
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
      <SlotSchedulePicker start_at=start_at end_at=end_at />

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
  let toast_msg: RwSignal<Option<String>> = RwSignal::new(None);

  // Auto-dismiss toast after 3 s
  Effect::new(move |_| {
    if toast_msg.get().is_some() {
      #[cfg(any(feature = "csr", feature = "hydrate"))]
      {
        use wasm_bindgen::JsCast;
        use wasm_bindgen::closure::Closure;
        let closure: Closure<dyn Fn()> = Closure::new(move || toast_msg.set(None));
        let _ = web_sys::window()
          .unwrap()
          .set_timeout_with_callback_and_timeout_and_arguments_0(
            closure.as_ref().unchecked_ref::<js_sys::Function>(),
            3000,
          );
        closure.forget();
      }
    }
  });

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
          <AddSessionDialog open=add_open state=state toast_msg=toast_msg />
        </Show>

        // ── Floating action button ────────────────────────────────────────────
        <button
          on:click=move |_| add_open.set(true)
          class="fixed bottom-6 right-6 z-30 inline-flex items-center gap-2 rounded-full bg-primary px-6 py-3 text-sm font-medium text-primary-foreground shadow-lg transition-colors hover:bg-primary/90"
        >
          "+ Add Session"
        </button>

        // ── Toast notification ────────────────────────────────────────────────
        <Show when=move || toast_msg.get().is_some()>
          <div class="fixed bottom-20 right-6 z-50 flex items-center gap-3 rounded-lg border bg-card px-4 py-3 shadow-lg">
            <span class="text-sm font-medium">
              {move || toast_msg.get().unwrap_or_default()}
            </span>
          </div>
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
