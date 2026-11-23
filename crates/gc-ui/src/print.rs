use gc_core::{format_day, format_duration, trim};
use leptos::prelude::*;

use crate::planner::{weight_text, when};
use crate::Ctx;

/// Hidden on screen. When printing, it is the only thing on the page:
/// a one page checklist to hang on the refrigerator.
#[component]
pub fn PrintSheet() -> impl IntoView {
    let ctx = use_context::<Ctx>().expect("ctx");
    move || {
        ctx.printing.get().map(|(s, settings)| {
            let t = settings.temp_unit;
            let day = s.input.carve_at.date();
            let thaw = s.thaw.as_ref().map(|th| {
                format!(
                    "Frozen? Move it to the refrigerator by {} ({} to {} days to thaw).",
                    format_day(th.latest),
                    th.days_min,
                    th.days_max
                )
            });
            view! {
                <div class="print-sheet">
                    <header>
                        <img src="icon.svg" alt="" />
                        <div>
                            <h1>{format!("Turkey Checklist: {}", format_day(day))}</h1>
                            <p>
                                {format!(
                                    "{} {}, timed as {} lb. Carve at {}. {} from refrigerator to table.",
                                    weight_text(s.input.weight_lb, settings.weight_unit),
                                    if s.input.breast { "turkey breast" } else { "turkey" },
                                    s.cook_pounds,
                                    gc_core::format_time(s.input.carve_at),
                                    format_duration(s.total_minutes as i64),
                                )}
                            </p>
                        </div>
                    </header>
                    <div class="print-thermo">
                        <strong>"CHECK THE TEMPERATURE"</strong>
                        <span>{gc_core::thermometer_note(t, s.input.breast)}</span>
                    </div>
                    <table>
                        <thead>
                            <tr>
                                <th class="box-col">"Done"</th>
                                <th>"Time"</th>
                                <th>"Step"</th>
                                <th>"Oven"</th>
                            </tr>
                        </thead>
                        <tbody>
                            {s.steps
                                .iter()
                                .map(|st| {
                                    view! {
                                        <tr class=format!("tone-{}", st.tone.key())>
                                            <td class="box-col"><span class="box"></span></td>
                                            <td class="time">{when(st.at, day)}</td>
                                            <td>
                                                <strong>{st.title.clone()}</strong>
                                                <div class="detail">{st.detail.clone()}</div>
                                            </td>
                                            <td class="oven">{st.oven_f.map(|f| t.show(f)).unwrap_or_default()}</td>
                                        </tr>
                                    }
                                })
                                .collect_view()}
                        </tbody>
                    </table>
                    <ul class="print-notes">
                        <li>
                            {format!(
                                "Keep the oven door closed while it slow cooks at {}. The {} timer starts the moment you turn the oven down.",
                                t.show(settings.low_temp_f),
                                format_duration(s.low_minutes as i64),
                            )}
                        </li>
                        <li>"Never stuff this turkey. Cook the stuffing in its own dish."</li>
                        {thaw.map(|x| view! { <li>{x}</li> })}
                        <li>{format!("Weight entered: {} lb. Under half a pound rounds down, half a pound or more rounds up.", trim(s.input.weight_lb))}</li>
                    </ul>
                    <footer>"Gobble Clock. Happy Thanksgiving!"</footer>
                </div>
            }
        })
    }
}
