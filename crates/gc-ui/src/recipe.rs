use gc_core::{BREAST_DONE_F, THIGH_DONE_F};
use leptos::prelude::*;
use leptos::task::spawn_local;

use crate::{api, Ctx};

pub const SITE: &str = "https://howtocookathanksgivingturkey.com/";
const CHART: &str = "https://howtocookathanksgivingturkey.com/pages/turkey-cooking-time.html";
const CONVECTION: &str = "https://howtocookathanksgivingturkey.com/pages/convection-ovens.html";

fn link(ctx: Ctx, label: &'static str, url: &'static str) -> impl IntoView {
    view! {
        <button
            class="btn"
            title=url
            on:click=move |_| {
                spawn_local(async move {
                    if let Err(e) = api::open_link(url).await {
                        ctx.fail(format!("Could not open the website: {e}"));
                    }
                })
            }
        >
            {label}
        </button>
    }
}

/// Credit to the recipe website and its tips, summarized in our own words.
#[component]
pub fn RecipePanel() -> impl IntoView {
    let ctx = expect_context::<Ctx>();
    let t = move || ctx.settings.with(|s| s.temp_unit);
    let temp = move |f: u32| t().show(f);
    view! {
        <div class="recipe">
            <section class="card credit">
                <h2>"The recipe behind Gobble Clock"</h2>
                <p>
                    "Gobble Clock is built on the slow cook method from "
                    <strong>"How to Cook a Thanksgiving Turkey"</strong>
                    " at howtocookathanksgivingturkey.com. All credit for the method goes to the site's author. Gobble Clock only does the math and the reminders."
                </p>
                <p class="muted">"Visit the site for the full recipe, photos, the roasting time chart, testimonials and answers to common questions."</p>
                <div class="row">
                    {link(ctx, "Open the recipe website", SITE)}
                    {link(ctx, "Roasting time chart", CHART)}
                    {link(ctx, "Convection ovens", CONVECTION)}
                </div>
            </section>
            <div class="tips-grid">
                <section class="card">
                    <h3>"Before the day"</h3>
                    <ul class="tips">
                        <li>"Thaw the turkey completely in the refrigerator. The slow cook does not work on a frozen or partly frozen bird."</li>
                        <li>"A fresh, never frozen turkey skips thawing altogether."</li>
                        <li>"Check your oven with an oven thermometer. The method depends on the oven really being at the set temperature."</li>
                        <li>"Make sure the turkey fits in your roasting pan, with the lid on if you plan to cover it."</li>
                        <li>"Know the real weight. Leave out the giblets and any ice packed inside a fresh turkey."</li>
                        <li>"This method is meant for turkeys of 20 pounds or less."</li>
                    </ul>
                </section>
                <section class="card">
                    <h3>"Getting it ready"</h3>
                    <ul class="tips">
                        <li>"Take out the neck and giblets. It happens more than you think."</li>
                        <li>"Let it sit out of the refrigerator for about an hour so it is close to room temperature."</li>
                        <li>"Work melted butter under the breast skin, not on top. Fresh rosemary and thyme there add great flavor."</li>
                        <li>"A light sprinkle of salt and pepper on the outside is all it needs."</li>
                        <li>"Do not stuff it. Stuffing throws off the time and is a safety risk. Bake it in a separate dish."</li>
                        <li>"Breast up makes the thermometer check easier. Some cooks say breast down is juicier. Either way works."</li>
                        <li>"No water in the pan."</li>
                    </ul>
                </section>
                <section class="card">
                    <h3>"In the oven"</h3>
                    <ul class="tips">
                        <li>{move || format!("Start hot: {} for 20 minutes seals the outside.", temp(475))}</li>
                        <li>{move || format!("Then turn the oven down to {}. Do not take the turkey out and do not open the door. Let the oven cool down by itself.", temp(250))}</li>
                        <li>"Start the 20 minutes per pound timer the moment you turn the oven down."</li>
                        <li>"Keep the door shut. No basting and no peeking. Every opening lets heat out, and a low oven is slow to recover."</li>
                        <li>"Want to cover it? Wait about 15 minutes after turning the oven down, then put the lid on. Uncovered gives crispier skin."</li>
                        <li>"No lid? Foil works fine."</li>
                        <li>{move || format!("Skip the probe thermometer during cooking. Most cannot handle {} and placing one means opening the door.", temp(475))}</li>
                    </ul>
                </section>
                <section class="card">
                    <h3>"Finishing"</h3>
                    <ul class="tips">
                        <li>{move || format!("When the time is up, check it with a meat thermometer: {} in the breast and {} in the thigh.", temp(BREAST_DONE_F), temp(THIGH_DONE_F))}</li>
                        <li>"Fresh turkeys can cook faster. Check about two thirds of the way through the slow cook."</li>
                        <li>"At high elevation it will take longer, so plan extra time and trust the thermometer."</li>
                        <li>"Nervous about timing? Plan for it to be ready an hour early. It is easy to keep a turkey warm."</li>
                        <li>"Let it rest uncovered for at least 20 minutes before carving. The juices settle and it carves cleaner."</li>
                    </ul>
                </section>
            </div>
            <p class="small muted">"These tips are a short summary written for Gobble Clock. The full recipe and the author's own words are on howtocookathanksgivingturkey.com."</p>
        </div>
    }
}
