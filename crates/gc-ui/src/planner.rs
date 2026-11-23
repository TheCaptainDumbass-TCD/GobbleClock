use chrono::{NaiveDate, NaiveDateTime};
use gc_core::{
    format_day, format_duration, format_time, lb_to_kg, thermometer_note, trim, CoverMode, Level, Schedule, Settings,
    Step, WeightUnit,
};
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::{api, now, Ctx, Panel};

/// "4:20 PM", with the day in front when it is not the carving day.
pub fn when(at: NaiveDateTime, carve_day: NaiveDate) -> String {
    if at.date() == carve_day {
        format_time(at)
    } else {
        format!("{} {}", at.format("%a"), format_time(at))
    }
}

pub fn weight_text(lb: f64, unit: WeightUnit) -> String {
    match unit {
        WeightUnit::Lb => format!("{} lb", trim(lb)),
        WeightUnit::Kg => format!("{} kg", trim(lb_to_kg(lb))),
    }
}

/// Keeps digits and one decimal point. A comma counts as a decimal point.
pub fn clean_weight(raw: &str) -> String {
    let mut out = String::new();
    let mut dot = false;
    for c in raw.chars() {
        match c {
            '0'..='9' => out.push(c),
            '.' | ',' if !dot => {
                dot = true;
                out.push('.');
            }
            _ => {}
        }
    }
    out
}

#[derive(Clone, Copy, PartialEq)]
pub enum RowState {
    Plain,
    Done,
    Now,
    Next,
}

pub fn step_row(step: &Step, carve_day: NaiveDate, settings: &Settings, state: RowState) -> impl IntoView {
    let t = settings.temp_unit;
    let class = format!(
        "step tone-{}{}",
        step.tone.key(),
        match state {
            RowState::Plain => "",
            RowState::Done => " done",
            RowState::Now => " now",
            RowState::Next => " next",
        }
    );
    let oven = step.oven_f.map(|f| t.show(f));
    let length = step.minutes.map(|m| format_duration(m as i64));
    view! {
        <li class=class>
            <span class="check" aria-hidden="true"></span>
            <span class="when">{when(step.at, carve_day)}</span>
            <div class="what">
                <strong>{step.title.clone()}</strong>
                <span class="detail">{step.detail.clone()}</span>
            </div>
            <div class="tags">
                {oven.map(|o| view! { <span class="tag oven">{o}</span> })}
                {length.map(|l| view! { <span class="tag">{l}</span> })}
            </div>
        </li>
    }
}

fn notice_view(level: Level, text: String) -> impl IntoView {
    let class = match level {
        Level::Info => "notice info",
        Level::Caution => "notice caution",
        Level::Danger => "notice danger",
    };
    view! { <div class=class>{text}</div> }
}

#[component]
pub fn Planner() -> impl IntoView {
    let ctx = use_context::<Ctx>().expect("ctx");
    let unit = move || ctx.settings.with(|s| s.weight_unit);

    let start_block = move || -> Option<&'static str> {
        if ctx.active.with(|a| a.is_some()) {
            return Some("A cook is already running. Stop it on the Cook tab first.");
        }
        match ctx.schedule.get() {
            Err(_) => Some("Enter the weight, day and carving time first."),
            Ok(s) if s.input.carve_at <= now() => Some("That carving time has already passed."),
            Ok(_) => None,
        }
    };
    let plan_ok = move || ctx.schedule.with(|s| s.is_ok());

    let start = move |_| {
        let Ok(input) = ctx.input.get_untracked() else { return };
        spawn_local(async move {
            match api::start_cook(input.clone()).await {
                Ok(()) => {
                    let settings = ctx.settings.get_untracked();
                    ctx.active.set(Some(api::ActiveCook { input, settings }));
                    ctx.panel.set(Panel::Cook);
                    ctx.say("Cook started. Reminders will pop up at every step.");
                }
                Err(e) => ctx.fail(e),
            }
        });
    };

    let export = move |kind: &'static str| {
        let Ok(input) = ctx.input.get_untracked() else { return };
        spawn_local(async move {
            match api::export_schedule(input, kind).await {
                Ok(Some(path)) => ctx.say(format!("Saved {path}")),
                Ok(None) => {}
                Err(e) => ctx.fail(e),
            }
        });
    };

    let print = move |_| {
        if let Ok(s) = ctx.schedule.get_untracked() {
            ctx.print(s, ctx.settings.get_untracked());
        }
    };

    let save_year = move |_| {
        let Ok(input) = ctx.input.get_untracked() else { return };
        let cover_mode = ctx.settings.with_untracked(|s| s.cover_mode);
        let mut history = ctx.history.get_untracked();
        let id = history.iter().map(|h| h.id).max().unwrap_or(0) + 1;
        history.retain(|h| h.date != input.carve_at.date());
        history.push(gc_core::HistoryEntry {
            id,
            date: input.carve_at.date(),
            weight_lb: input.weight_lb,
            carve_time: input.carve_at.time(),
            fresh: input.fresh,
            breast: input.breast,
            cover_mode,
            rating: 0,
            notes: String::new(),
        });
        history.sort_by_key(|h| h.date);
        ctx.store_history(history);
        ctx.say("Saved to History. Rate it and add notes after dinner.");
    };

    let set_buffer = move |on: bool| {
        let mut s = ctx.settings.get_untracked();
        s.buffer_enabled = on;
        ctx.store_settings(s);
    };
    let set_cover = move |key: String| {
        let mut s = ctx.settings.get_untracked();
        s.cover_mode = CoverMode::from_key(&key);
        ctx.store_settings(s);
    };

    view! {
        <div class="plan-grid">
            <section class="card form">
                <h2>"Your turkey"</h2>
                <label>
                    "Weight"
                    <div class="with-unit">
                        <input
                            type="text"
                            inputmode="decimal"
                            autocomplete="off"
                            placeholder=move || if unit() == WeightUnit::Lb { "13.23" } else { "6" }
                            prop:value=move || ctx.weight.get()
                            on:input=move |ev| ctx.weight.set(clean_weight(&event_target_value(&ev)))
                        />
                        <span>{move || if unit() == WeightUnit::Lb { "lb" } else { "kg" }}</span>
                    </div>
                </label>
                <label>
                    "Day you eat"
                    <input
                        type="date"
                        prop:value=move || ctx.date.get()
                        on:input=move |ev| ctx.date.set(event_target_value(&ev))
                    />
                </label>
                <label>
                    "Carving time"
                    <input
                        type="time"
                        step="300"
                        prop:value=move || ctx.time.get()
                        on:input=move |ev| ctx.time.set(event_target_value(&ev))
                    />
                </label>
                <label class="toggle">
                    <input
                        type="checkbox"
                        prop:checked=move || ctx.fresh.get()
                        on:change=move |ev| ctx.fresh.set(event_target_checked(&ev))
                    />
                    "Fresh turkey, never frozen"
                </label>
                <label class="toggle">
                    <input
                        type="checkbox"
                        prop:checked=move || ctx.breast.get()
                        on:change=move |ev| ctx.breast.set(event_target_checked(&ev))
                    />
                    "Turkey breast only, no legs or wings"
                </label>
                <label class="toggle">
                    <input
                        type="checkbox"
                        prop:checked=move || ctx.settings.with(|s| s.buffer_enabled)
                        on:change=move |ev| set_buffer(event_target_checked(&ev))
                    />
                    {move || {
                        format!(
                            "Wiggle room: be ready {} early",
                            format_duration(ctx.settings.with(|s| s.buffer_minutes) as i64),
                        )
                    }}
                </label>
                <label>
                    "Lid"
                    <select on:change=move |ev| set_cover(event_target_value(&ev))>
                        {CoverMode::ALL
                            .into_iter()
                            .map(|m| {
                                view! {
                                    <option
                                        value=m.key()
                                        selected=move || ctx.settings.with(|s| s.cover_mode) == m
                                    >
                                        {m.label()}
                                    </option>
                                }
                            })
                            .collect_view()}
                    </select>
                </label>
                <div class="actions">
                    <button
                        class="btn primary"
                        disabled=move || start_block().is_some()
                        title=move || start_block().unwrap_or("Start live reminders for this plan")
                        on:click=start
                    >
                        "Start cooking"
                    </button>
                    <button
                        class="btn"
                        disabled=move || !plan_ok()
                        title=move || if plan_ok() { "Print a checklist for the refrigerator" } else { "Enter the weight first" }
                        on:click=print
                    >
                        "Print checklist"
                    </button>
                    <button
                        class="btn"
                        disabled=move || !plan_ok()
                        title=move || if plan_ok() { "Save the checklist as a text file" } else { "Enter the weight first" }
                        on:click=move |_| export("txt")
                    >
                        "Save as text"
                    </button>
                    <button
                        class="btn"
                        disabled=move || !plan_ok()
                        title=move || if plan_ok() { "Save a calendar file with an alarm for every step" } else { "Enter the weight first" }
                        on:click=move |_| export("ics")
                    >
                        "Add to calendar"
                    </button>
                    <button
                        class="btn"
                        disabled=move || !plan_ok()
                        title=move || if plan_ok() { "Keep this turkey in History for next year" } else { "Enter the weight first" }
                        on:click=save_year
                    >
                        "Save to history"
                    </button>
                    <button
                        class="btn thaw-btn"
                        title="Refrigerator thaw times and safety tips"
                        on:click=move |_| ctx.thaw_help.set(true)
                    >
                        "Thawing"
                    </button>
                </div>
            </section>
            <section class="results">
                {move || match ctx.schedule.get() {
                    Err(e) => {
                        view! {
                            <div class="card empty">
                                <img src="icon.svg" alt="" />
                                <h2>"Let's plan the bird"</h2>
                                <p>{e}</p>
                                <p class="muted">
                                    "Gobble Clock works backward from carving time: 1 hour out of the refrigerator, 20 minutes of prep, 20 minutes at 475°F, then 20 minutes per pound at 250°F and a 20 minute rest."
                                </p>
                            </div>
                        }
                            .into_any()
                    }
                    Ok(s) => {
                        let settings = ctx.settings.get();
                        schedule_view(s, settings).into_any()
                    }
                }}
            </section>
        </div>
    }
}

fn milestone(label: &'static str, at: Option<NaiveDateTime>, day: NaiveDate) -> impl IntoView {
    view! {
        <div class="milestone">
            <span>{label}</span>
            <strong>{at.map(|a| when(a, day)).unwrap_or_default()}</strong>
        </div>
    }
}

fn schedule_view(s: Schedule, settings: Settings) -> impl IntoView {
    use gc_core::StepKind::*;
    let ctx = expect_context::<Ctx>();
    let day = s.input.carve_at.date();
    let t = settings.temp_unit;
    let at = |k| s.step(k).map(|x| x.at);
    let thaw = s.thaw.clone().map(|th| {
        view! {
            <div class="card thaw">
                <div class="row between">
                    <h3>"Frozen turkey?"</h3>
                    <button class="btn small-btn" on:click=move |_| ctx.thaw_help.set(true)>
                        "Thaw times and safety"
                    </button>
                </div>
                <p>
                    "Move it from the freezer to the refrigerator by "
                    <strong>{format_day(th.latest)}</strong>
                    {format!(". A {} bird needs {} to {} days to thaw.", weight_text(s.input.weight_lb, settings.weight_unit), th.days_min, th.days_max)}
                </p>
            </div>
        }
    });
    let header = format!(
        "{} for {}, timed as {} lb. {} from refrigerator to carving.",
        format_day(day),
        weight_text(s.input.weight_lb, settings.weight_unit),
        s.cook_pounds,
        format_duration(s.total_minutes as i64)
    );
    let pounds = s.pounds.clone();
    view! {
        <div class="card summary">
            <p class="muted">{header}</p>
            <div class="milestones">
                {milestone("Out of the fridge", at(Warmup), day)}
                {milestone("In the oven", at(Sear), day)}
                {milestone("Oven down", at(ReduceHeat), day)}
                {milestone("Out of the oven", at(Remove), day)}
                {milestone("Carve", at(Carve), day)}
            </div>
        </div>
        {s.notices.iter().map(|n| notice_view(n.level, n.text.clone())).collect_view()}
        <div class="notice thermo">
            <strong>"Check the temperature. "</strong>
            {thermometer_note(t, s.input.breast)}
        </div>
        <div class="card">
            <h3>"Checklist"</h3>
            <ul class="steps">
                {s.steps.iter().map(|st| step_row(st, day, &settings, RowState::Plain)).collect_view()}
            </ul>
            <p class="muted small">
                {format!(
                    "Keep the oven door closed while it slow cooks at {}. Every peek lets the heat out.",
                    t.show(settings.low_temp_f),
                )}
            </p>
        </div>
        {thaw}
        <details class="card pounds">
            <summary>{format!("Pound by pound at {} ({} pounds)", t.show(settings.low_temp_f), s.cook_pounds)}</summary>
            <ol>
                {pounds
                    .into_iter()
                    .map(|p| {
                        view! {
                            <li>
                                <span>{format!("Pound {}", p.pound)}</span>
                                <span>{format!("{} to {}", when(p.starts, day), when(p.ends, day))}</span>
                            </li>
                        }
                    })
                    .collect_view()}
            </ol>
        </details>
    }
}
