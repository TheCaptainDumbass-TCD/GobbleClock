use gc_core::{Settings, TempUnit, WeightUnit};
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::{api, Ctx};

pub const GITHUB: &str = "https://github.com/TheCaptainDumbass-TCD/GobbleClock";
pub const WEBSITE: &str = "https://thecaptaindumbass.com/gobbleclock/";

fn change(ctx: Ctx, f: impl FnOnce(&mut Settings)) {
    let mut s = ctx.settings.get_untracked();
    f(&mut s);
    ctx.store_settings(s);
}

fn number_field(ctx: Ctx, label: &'static str, unit: &'static str, get: fn(&Settings) -> u32, set: fn(&mut Settings, u32)) -> impl IntoView {
    view! {
        <label>
            {label}
            <div class="with-unit">
                <input
                    type="number"
                    min="0"
                    max="600"
                    prop:value=move || ctx.settings.with(get).to_string()
                    on:change=move |ev| {
                        if let Ok(n) = event_target_value(&ev).trim().parse::<u32>() {
                            change(ctx, |s| set(s, n.min(600)));
                        }
                    }
                />
                <span>{unit}</span>
            </div>
        </label>
    }
}

#[component]
pub fn SettingsPanel() -> impl IntoView {
    let ctx = use_context::<Ctx>().expect("ctx");
    let info = LocalResource::new(api::app_info);
    let custom = move || ctx.settings.with(|s| *s != s.with_default_method());

    view! {
        <div class="settings-grid">
            <section class="card form">
                <h2>"Preferences"</h2>
                <label>
                    "Temperatures"
                    <select on:change=move |ev| {
                        let c = event_target_value(&ev) == "c";
                        change(ctx, |s| s.temp_unit = if c { TempUnit::C } else { TempUnit::F });
                    }>
                        <option value="f" selected=move || ctx.settings.with(|s| s.temp_unit == TempUnit::F)>
                            "Fahrenheit (°F)"
                        </option>
                        <option value="c" selected=move || ctx.settings.with(|s| s.temp_unit == TempUnit::C)>
                            "Celsius (°C)"
                        </option>
                    </select>
                </label>
                <label>
                    "Weight"
                    <select on:change=move |ev| {
                        let kg = event_target_value(&ev) == "kg";
                        ctx.weight.set(String::new());
                        change(ctx, |s| s.weight_unit = if kg { WeightUnit::Kg } else { WeightUnit::Lb });
                    }>
                        <option value="lb" selected=move || ctx.settings.with(|s| s.weight_unit == WeightUnit::Lb)>
                            "Pounds (lb)"
                        </option>
                        <option value="kg" selected=move || ctx.settings.with(|s| s.weight_unit == WeightUnit::Kg)>
                            "Kilograms (kg)"
                        </option>
                    </select>
                </label>
                <label>
                    "Theme"
                    <select on:change=move |ev| {
                        let t = event_target_value(&ev);
                        change(ctx, |s| s.theme = t);
                    }>
                        {[("system", "Match the system"), ("light", "Harvest light"), ("dark", "Fireside dark")]
                            .into_iter()
                            .map(|(k, l)| {
                                view! {
                                    <option value=k selected=move || ctx.settings.with(|s| s.theme == k)>
                                        {l}
                                    </option>
                                }
                            })
                            .collect_view()}
                    </select>
                </label>
                <label>
                    "Usual carving time"
                    <input
                        type="time"
                        step="300"
                        prop:value=move || ctx.settings.with(|s| s.default_carve_time.clone())
                        on:change=move |ev| {
                            let t = event_target_value(&ev);
                            change(ctx, |s| s.default_carve_time = t);
                        }
                    />
                </label>
                <label class="toggle">
                    <input
                        type="checkbox"
                        prop:checked=move || ctx.settings.with(|s| s.sound)
                        on:change=move |ev| {
                            let on = event_target_checked(&ev);
                            change(ctx, |s| s.sound = on);
                        }
                    />
                    "Play a sound with each reminder"
                </label>
                <div class="actions">
                    <button
                        class="btn"
                        title="Send a sample reminder now"
                        on:click=move |_| {
                            spawn_local(async move {
                                if let Err(e) = api::test_reminder().await {
                                    ctx.fail(e);
                                }
                            })
                        }
                    >
                        "Test reminder"
                    </button>
                </div>
            </section>
            <details class="card form method">
                <summary>
                    <h2>"Advanced: cooking method"</h2>
                    <span class="muted small">
                        {move || if custom() { "Custom times in use" } else { "Using the recipe times" }}
                    </span>
                </summary>
                <p class="notice caution small">
                    "The recipe defaults from howtocookathanksgivingturkey.com are recommended: 475°F for 20 minutes, then 250°F at 20 minutes per pound with the oven door closed the whole time so the turkey is never shocked. Any change here rebuilds every time in the plan."
                </p>
                {number_field(ctx, "Out of the refrigerator before cooking", "min", |s| s.warmup_minutes, |s, n| s.warmup_minutes = n)}
                {number_field(ctx, "Prep while the oven preheats", "min", |s| s.prep_minutes, |s, n| s.prep_minutes = n)}
                {number_field(ctx, "Hot start temperature", "°F", |s| s.sear_temp_f, |s, n| s.sear_temp_f = n)}
                {number_field(ctx, "Hot start time", "min", |s| s.sear_minutes, |s, n| s.sear_minutes = n)}
                {number_field(ctx, "Slow cook temperature", "°F", |s| s.low_temp_f, |s, n| s.low_temp_f = n)}
                {number_field(ctx, "Slow cook time per pound", "min", |s| s.minutes_per_pound, |s, n| s.minutes_per_pound = n.max(1))}
                {number_field(ctx, "Put the lid on after turning down", "min", |s| s.cover_after_minutes, |s, n| s.cover_after_minutes = n)}
                {number_field(ctx, "Uncovered at the end to brown", "min", |s| s.brown_minutes, |s, n| s.brown_minutes = n)}
                {number_field(ctx, "Rest before carving", "min", |s| s.rest_minutes, |s, n| s.rest_minutes = n)}
                {number_field(ctx, "Wiggle room when turned on", "min", |s| s.buffer_minutes, |s, n| s.buffer_minutes = n)}
                <div class="actions">
                    <button
                        class="btn"
                        disabled=move || !custom()
                        title=move || if custom() { "Put every method time back to the recipe" } else { "Already using the recipe times" }
                        on:click=move |_| {
                            let s = ctx.settings.get_untracked().with_default_method();
                            ctx.store_settings(s);
                        }
                    >
                        "Reset to recipe"
                    </button>
                </div>
            </details>
            <section class="card about">
                <h2>"About"</h2>
                {move || {
                    info.get()
                        .and_then(|r| r.ok())
                        .map(|i| {
                            view! {
                                <p>{format!("{} version {}", i.name, i.version)}</p>
                                <p class="small muted">{format!("Your data: {}", i.data_dir)}</p>
                            }
                        })
                }}
                <div class="actions">
                    <button
                        class="btn"
                        title=GITHUB
                        on:click=move |_| {
                            spawn_local(async move {
                                if let Err(e) = api::open_link(GITHUB).await {
                                    ctx.fail(format!("Could not open the GitHub page: {e}"));
                                }
                            })
                        }
                    >
                        "Open the GitHub page"
                    </button>
                    <button
                        class="btn"
                        title=WEBSITE
                        on:click=move |_| {
                            spawn_local(async move {
                                if let Err(e) = api::open_link(WEBSITE).await {
                                    ctx.fail(format!("Could not open the website: {e}"));
                                }
                            })
                        }
                    >
                        "Open the website"
                    </button>
                </div>
                <p class="small muted">
                    "Cooking method from howtocookathanksgivingturkey.com. Copyright (C) 2026 Dennis Ayotte. GPL-3.0-or-later."
                </p>
            </section>
        </div>
    }
}
