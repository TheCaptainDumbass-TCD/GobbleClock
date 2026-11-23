use chrono::Datelike;
use gc_core::{cook_pounds, format_day, HistoryEntry};
use leptos::prelude::*;

use crate::planner::weight_text;
use crate::Ctx;

fn update(ctx: Ctx, id: u64, f: impl FnOnce(&mut HistoryEntry)) {
    let mut history = ctx.history.get_untracked();
    if let Some(e) = history.iter_mut().find(|e| e.id == id) {
        f(e);
        ctx.store_history(history);
    }
}

#[component]
pub fn HistoryPanel() -> impl IntoView {
    let ctx = use_context::<Ctx>().expect("ctx");
    let confirm = RwSignal::new(None::<u64>);

    let card = move |e: HistoryEntry| {
        let id = e.id;
        let unit = ctx.settings.with(|s| s.weight_unit);
        let entry = e.clone();
        let carve = e.date.and_time(e.carve_time);
        let rating = e.rating;
        let stars = (1..=5u8)
            .map(|n| {
                let on = rating >= n;
                view! {
                    <button
                        class="star"
                        class:on=move || on
                        title=format!("{n} of 5")
                        on:click=move |_| update(ctx, id, |x| x.rating = if x.rating == n { 0 } else { n })
                    >
                        "★"
                    </button>
                }
            })
            .collect_view();
        view! {
            <article class="card year">
                <div class="row between">
                    <h2>{e.date.year()}</h2>
                    <div class="stars">{stars}</div>
                </div>
                <p class="muted">
                    {format!(
                        "{}. {}, timed as {} lb. Carved at {}.{}{}",
                        format_day(e.date),
                        weight_text(e.weight_lb, unit),
                        cook_pounds(e.weight_lb),
                        gc_core::format_time(carve),
                        if e.fresh { " Fresh turkey." } else { "" },
                        if e.breast { " Turkey breast only." } else { "" },
                    )}
                </p>
                <p class="small muted">{e.cover_mode.label()}</p>
                <textarea
                    placeholder="How did it turn out? Anything to change next year?"
                    prop:value=e.notes.clone()
                    on:change=move |ev| {
                        let text = event_target_value(&ev);
                        update(ctx, id, |x| x.notes = text);
                    }
                ></textarea>
                <div class="actions">
                    <button
                        class="btn primary"
                        title="Fill in the planner with this turkey for this year"
                        on:click=move |_| ctx.plan_from(&entry)
                    >
                        "Plan with this turkey"
                    </button>
                    <Show
                        when=move || confirm.get() == Some(id)
                        fallback=move || {
                            view! {
                                <button class="btn danger-outline" on:click=move |_| confirm.set(Some(id))>
                                    "Delete"
                                </button>
                            }
                        }
                    >
                        <span class="confirm">
                            "Delete this year?"
                            <button
                                class="btn danger"
                                on:click=move |_| {
                                    let mut h = ctx.history.get_untracked();
                                    h.retain(|x| x.id != id);
                                    ctx.store_history(h);
                                    confirm.set(None);
                                }
                            >
                                "Yes, delete"
                            </button>
                            <button class="btn" on:click=move |_| confirm.set(None)>
                                "No"
                            </button>
                        </span>
                    </Show>
                </div>
            </article>
        }
    };

    view! {
        <div class="history">
            <p class="muted">
                "Every turkey you save lands here, newest first. Rate it and leave notes so next year is even better."
            </p>
            {move || {
                let mut h = ctx.history.get();
                if h.is_empty() {
                    return view! { <div class="card empty"><p>"No turkeys saved yet."</p></div> }.into_any();
                }
                h.sort_by_key(|e| std::cmp::Reverse(e.date));
                view! { <div class="years">{h.into_iter().map(card).collect_view()}</div> }.into_any()
            }}
        </div>
    }
}
