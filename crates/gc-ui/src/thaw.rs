use gc_core::{format_day, lb_to_kg, WeightUnit, FRIDGE_MAX_F, THAW_TABLE};
use leptos::prelude::*;

use crate::planner::weight_text;
use crate::Ctx;

fn range(from: u32, to: u32, unit: WeightUnit) -> String {
    match unit {
        WeightUnit::Lb => format!("{from} to {to} lb"),
        WeightUnit::Kg => format!("{:.1} to {:.1} kg", lb_to_kg(from as f64), lb_to_kg(to as f64)),
    }
}

/// Refrigerator thaw times and safety tips, opened from the planner.
#[component]
pub fn ThawHelp() -> impl IntoView {
    let ctx = expect_context::<Ctx>();
    let unit = move || ctx.settings.with(|s| s.weight_unit);
    let fridge = move || ctx.settings.with(|s| s.temp_unit.show(FRIDGE_MAX_F));
    let yours = move || {
        ctx.schedule.get().ok().and_then(|s| {
            s.thaw.map(|th| {
                format!(
                    "Your {} turkey needs {} to {} days. Move it from the freezer to the refrigerator by {}.",
                    weight_text(s.input.weight_lb, unit()),
                    th.days_min,
                    th.days_max,
                    format_day(th.latest)
                )
            })
        })
    };
    view! {
        <div class="modal-back" on:click=move |_| ctx.thaw_help.set(false)>
            <div class="modal wide" on:click=|ev| ev.stop_propagation()>
                <h2>"Thawing in the refrigerator"</h2>
                {move || yours().map(|y| view! { <p class="notice info">{y}</p> })}
                <table class="thaw-table">
                    <thead>
                        <tr>
                            <th>"Turkey weight"</th>
                            <th>"Days in the refrigerator"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {move || {
                            THAW_TABLE
                                .iter()
                                .map(|&(a, b, d1, d2)| {
                                    view! {
                                        <tr>
                                            <td>{range(a, b, unit())}</td>
                                            <td>{format!("{d1} to {d2} days")}</td>
                                        </tr>
                                    }
                                })
                                .collect_view()
                        }}
                    </tbody>
                </table>
                <p class="small muted">"Times are from the recipe website and are a rough guide. A colder refrigerator or a lower shelf takes longer."</p>
                <h3>"Thaw it safely"</h3>
                <ul class="tips">
                    <li>{move || format!("Thaw only in the refrigerator, at {} or colder. Never on the counter.", fridge())}</li>
                    <li>"The recipe does not use cold water or microwave thawing because of the risk of bacteria."</li>
                    <li>"Leave it in its wrapper on a tray or in a pan on the bottom shelf so juices cannot drip onto other food."</li>
                    <li>"It must be completely thawed for the slow cook to work. There should be no ice inside and the legs should move freely."</li>
                    <li>"If it is still frozen on the day, do not use the slow cook method."</li>
                    <li>"Once thawed, cook it within 1 to 2 days."</li>
                    <li>"Want to skip thawing? Buy a fresh, never frozen turkey and check Fresh turkey in the planner."</li>
                </ul>
                <div class="row end">
                    <button class="btn primary" on:click=move |_| ctx.thaw_help.set(false)>
                        "Got it"
                    </button>
                </div>
            </div>
        </div>
    }
}
