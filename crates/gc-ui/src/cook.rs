use chrono::NaiveDateTime;
use gc_core::{format_time, HistoryEntry, StepKind};
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::planner::{step_row, thermometer_text, weight_text, when, RowState};
use crate::{api, Ctx, Panel};

fn countdown(from: NaiveDateTime, to: NaiveDateTime) -> String {
    let secs = (to - from).num_seconds().max(0);
    let (h, m, s) = (secs / 3600, secs / 60 % 60, secs % 60);
    if h > 0 {
        format!("{h}:{m:02}:{s:02}")
    } else {
        format!("{m}:{s:02}")
    }
}

#[component]
pub fn CookPanel() -> impl IntoView {
    let ctx = use_context::<Ctx>().expect("ctx");
    let confirm_stop = RwSignal::new(false);

    let stop = move |save: bool| {
        let cook = ctx.active.get_untracked();
        spawn_local(async move {
            match api::stop_cook().await {
                Ok(()) => {
                    ctx.active.set(None);
                    confirm_stop.set(false);
                    if let (true, Some(c)) = (save, cook) {
                        let mut history = ctx.history.get_untracked();
                        let id = history.iter().map(|h| h.id).max().unwrap_or(0) + 1;
                        history.retain(|h| h.date != c.input.carve_at.date());
                        history.push(HistoryEntry {
                            id,
                            date: c.input.carve_at.date(),
                            weight_lb: c.input.weight_lb,
                            carve_time: c.input.carve_at.time(),
                            fresh: c.input.fresh,
                            cover_mode: c.settings.cover_mode,
                            rating: 0,
                            notes: String::new(),
                        });
                        history.sort_by_key(|h| h.date);
                        ctx.store_history(history);
                        ctx.panel.set(Panel::History);
                        ctx.say("Happy Thanksgiving! Rate this year's turkey and add notes for next year.");
                    } else {
                        ctx.say("Cook stopped. Reminders are off.");
                    }
                }
                Err(e) => ctx.fail(e),
            }
        });
    };

    let test = move |_| {
        spawn_local(async move {
            if let Err(e) = api::test_reminder().await {
                ctx.fail(e);
            }
        });
    };

    move || {
        let Some(s) = ctx.active_schedule.get() else {
            return view! {
                <div class="card empty">
                    <img src="icon.svg" alt="" />
                    <h2>"Nothing in the oven yet"</h2>
                    <p>"Plan your turkey, then press Start cooking to get a countdown and a reminder at every step."</p>
                    <button class="btn primary" on:click=move |_| ctx.panel.set(Panel::Plan)>
                        "Go to Plan"
                    </button>
                </div>
            }
            .into_any();
        };
        let settings = ctx.active.with(|a| a.as_ref().map(|c| c.settings.clone())).unwrap_or_default();
        let now = ctx.now.get();
        let day = s.input.carve_at.date();
        let start = s.starts_at();
        let carve = s.input.carve_at;
        let current = s.current_index(now);
        let next = s.steps.iter().position(|st| st.at > now);
        let progress = if now <= start {
            0.0
        } else {
            ((now - start).num_seconds() as f64 / (carve - start).num_seconds().max(1) as f64).min(1.0) * 100.0
        };
        let finished = now >= carve;

        let headline = match (current, next) {
            (None, Some(n)) => view! {
                <p class="eyebrow">"Starts in"</p>
                <p class="big">{countdown(now, s.steps[n].at)}</p>
                <p>{format!("{} at {}", s.steps[n].title, format_time(s.steps[n].at))}</p>
            }
            .into_any(),
            (Some(c), Some(n)) => view! {
                <p class="eyebrow">"Now"</p>
                <h2>{s.steps[c].title.clone()}</h2>
                <p class="muted">{s.steps[c].detail.clone()}</p>
                <p class="eyebrow">"Next in"</p>
                <p class="big">{countdown(now, s.steps[n].at)}</p>
                <p>{format!("{} at {}", s.steps[n].title, when(s.steps[n].at, day))}</p>
            }
            .into_any(),
            _ => view! {
                <p class="eyebrow">"Time to eat"</p>
                <h2>"Carve and enjoy. Happy Thanksgiving!"</h2>
            }
            .into_any(),
        };

        let low = s.step(StepKind::ReduceHeat).map(|st| st.at);
        let remove = s.step(StepKind::Remove).map(|st| st.at);
        let pound = match (low, remove) {
            (Some(l), Some(r)) if now >= l && now < r => {
                let mpp = settings.minutes_per_pound.max(1) as i64;
                let p = (now - l).num_minutes() / mpp + 1;
                Some(format!("Pound {p} of {}", s.cook_pounds))
            }
            _ => None,
        };

        let rows = s
            .steps
            .iter()
            .enumerate()
            .map(|(i, st)| {
                let state = if Some(i) == next {
                    RowState::Next
                } else if st.at <= now && current.is_some_and(|c| s.steps[c].at == st.at) {
                    RowState::Now
                } else if st.at <= now {
                    RowState::Done
                } else {
                    RowState::Plain
                };
                step_row(st, day, &settings, state)
            })
            .collect_view();

        let print_s = s.clone();
        let print_settings = settings.clone();
        view! {
            <div class="cook-grid">
                <section class="card live">
                    {headline}
                    <div class="bar" title=format!("{progress:.0}% of the way to carving")>
                        <div class="fill" style=format!("width: {progress:.1}%")></div>
                    </div>
                    <div class="row between small muted">
                        <span>{format!("Started {}", when(start, day))}</span>
                        {pound.map(|p| view! { <strong class="pound">{p}</strong> })}
                        <span>{format!("Carve {}", when(carve, day))}</span>
                    </div>
                    <div class="notice thermo">
                        <strong>"Check the temperature. "</strong>
                        {thermometer_text(settings.temp_unit)}
                    </div>
                    <p class="muted small">
                        {format!(
                            "{} turkey, timed as {} lb. Keep the oven door closed while it slow cooks.",
                            weight_text(s.input.weight_lb, settings.weight_unit),
                            s.cook_pounds,
                        )}
                    </p>
                    <div class="actions">
                        <button
                            class="btn"
                            on:click=move |_| ctx.print(print_s.clone(), print_settings.clone())
                        >
                            "Print checklist"
                        </button>
                        <button class="btn" title="Send a sample reminder now" on:click=test>
                            "Test reminder"
                        </button>
                        <Show when=move || finished>
                            <button class="btn primary" on:click=move |_| stop(true)>
                                "Finish and save to history"
                            </button>
                        </Show>
                        <Show
                            when=move || confirm_stop.get()
                            fallback=move || {
                                view! {
                                    <button class="btn danger-outline" on:click=move |_| confirm_stop.set(true)>
                                        "Stop cook"
                                    </button>
                                }
                            }
                        >
                            <span class="confirm">
                                "Stop all reminders?"
                                <button class="btn danger" on:click=move |_| stop(false)>
                                    "Yes, stop"
                                </button>
                                <button class="btn" on:click=move |_| confirm_stop.set(false)>
                                    "No"
                                </button>
                            </span>
                        </Show>
                    </div>
                </section>
                <section class="card">
                    <h3>"Steps"</h3>
                    <ul class="steps">{rows}</ul>
                </section>
            </div>
        }
        .into_any()
    }
}
