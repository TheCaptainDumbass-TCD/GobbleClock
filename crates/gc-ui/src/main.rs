mod api;
mod cook;
mod history;
mod planner;
mod print;
mod recipe;
mod settings;
mod sound;
mod tauri;
mod thaw;

use std::time::Duration;

use chrono::{Local, NaiveDate, NaiveDateTime, NaiveTime};
use gc_core::{build, kg_to_lb, next_thanksgiving, HistoryEntry, PlanInput, Schedule, Settings, WeightUnit};
use leptos::prelude::*;
use leptos::task::spawn_local;

use api::{ActiveCook, StepDue};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Panel {
    Plan,
    Cook,
    History,
    Recipe,
    Settings,
}

impl Panel {
    const ALL: [Panel; 5] = [Panel::Plan, Panel::Cook, Panel::History, Panel::Recipe, Panel::Settings];

    fn label(self) -> &'static str {
        match self {
            Panel::Plan => "Plan",
            Panel::Cook => "Cook",
            Panel::History => "History",
            Panel::Recipe => "Recipe",
            Panel::Settings => "Settings",
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct Toast {
    pub text: String,
    pub bad: bool,
}

/// Everything the panels share. All fields are signals, so the struct is Copy.
#[derive(Clone, Copy)]
pub struct Ctx {
    pub panel: RwSignal<Panel>,
    pub settings: RwSignal<Settings>,
    pub history: RwSignal<Vec<HistoryEntry>>,
    pub active: RwSignal<Option<ActiveCook>>,
    pub loaded: RwSignal<bool>,
    pub now: RwSignal<NaiveDateTime>,
    pub weight: RwSignal<String>,
    pub date: RwSignal<String>,
    pub time: RwSignal<String>,
    pub fresh: RwSignal<bool>,
    pub breast: RwSignal<bool>,
    pub thaw_help: RwSignal<bool>,
    pub toast: RwSignal<Option<Toast>>,
    pub printing: RwSignal<Option<(Schedule, Settings)>>,
    pub input: Memo<Result<PlanInput, String>>,
    pub schedule: Memo<Result<Schedule, String>>,
    pub active_schedule: Memo<Option<Schedule>>,
}

pub fn now() -> NaiveDateTime {
    Local::now().naive_local()
}

impl Ctx {
    fn new() -> Ctx {
        let settings = RwSignal::new(Settings::default());
        let weight = RwSignal::new(String::new());
        let date = RwSignal::new(Ctx::default_date());
        let time = RwSignal::new("16:20".to_string());
        let fresh = RwSignal::new(false);
        let breast = RwSignal::new(false);
        let active = RwSignal::new(None::<ActiveCook>);
        let input = Memo::new(move |_| {
            parse_input(&weight.get(), &date.get(), &time.get(), fresh.get(), breast.get(), settings.with(|s| s.weight_unit))
        });
        let schedule = Memo::new(move |_| {
            let input = input.get()?;
            settings.with(|s| build(&input, s)).map_err(|e| e.to_string())
        });
        let active_schedule = Memo::new(move |_| active.with(|a| a.as_ref().and_then(|c| build(&c.input, &c.settings).ok())));
        Ctx {
            panel: RwSignal::new(Panel::Plan),
            settings,
            history: RwSignal::new(Vec::new()),
            active,
            loaded: RwSignal::new(false),
            now: RwSignal::new(now()),
            weight,
            date,
            time,
            fresh,
            breast,
            thaw_help: RwSignal::new(false),
            toast: RwSignal::new(None),
            printing: RwSignal::new(None),
            input,
            schedule,
            active_schedule,
        }
    }

    fn default_date() -> String {
        next_thanksgiving(now().date()).format("%Y-%m-%d").to_string()
    }

    /// True when the planner holds anything other than the defaults.
    pub fn plan_changed(self) -> bool {
        let time = self.settings.with(|s| s.default_carve_time.clone());
        !self.weight.get().trim().is_empty()
            || self.date.get() != Self::default_date()
            || self.time.get() != time
            || self.fresh.get()
            || self.breast.get()
    }

    /// Clears the weight and puts the day, carving time and checkboxes back to the defaults.
    pub fn reset_plan(self) {
        self.weight.set(String::new());
        self.date.set(Self::default_date());
        self.time.set(self.settings.with_untracked(|s| s.default_carve_time.clone()));
        self.fresh.set(false);
        self.breast.set(false);
    }

    pub fn say(self, text: impl Into<String>) {
        self.show_toast(text.into(), false);
    }

    pub fn fail(self, text: impl Into<String>) {
        self.show_toast(text.into(), true);
    }

    fn show_toast(self, text: String, bad: bool) {
        let toast = Toast { text, bad };
        self.toast.set(Some(toast.clone()));
        set_timeout(
            move || {
                if self.toast.get_untracked().as_ref() == Some(&toast) {
                    self.toast.set(None);
                }
            },
            Duration::from_secs(if bad { 8 } else { 4 }),
        );
    }

    /// Saves settings and reports any failure.
    pub fn store_settings(self, s: Settings) {
        self.settings.set(s.clone());
        spawn_local(async move {
            if let Err(e) = api::save_settings(s).await {
                self.fail(e);
            }
        });
    }

    pub fn store_history(self, h: Vec<HistoryEntry>) {
        self.history.set(h.clone());
        spawn_local(async move {
            if let Err(e) = api::save_history(h).await {
                self.fail(e);
            }
        });
    }

    /// Fills the planner form from a past year, on this year's Thanksgiving.
    pub fn plan_from(self, e: &HistoryEntry) {
        let w = match self.settings.with_untracked(|s| s.weight_unit) {
            WeightUnit::Lb => e.weight_lb,
            WeightUnit::Kg => gc_core::lb_to_kg(e.weight_lb),
        };
        self.weight.set(gc_core::trim(w));
        self.time.set(e.carve_time.format("%H:%M").to_string());
        self.fresh.set(e.fresh);
        self.breast.set(e.breast);
        self.date.set(next_thanksgiving(now().date()).format("%Y-%m-%d").to_string());
        self.panel.set(Panel::Plan);
    }

    /// Fills a print sheet and opens the system print dialog.
    pub fn print(self, schedule: Schedule, settings: Settings) {
        self.printing.set(Some((schedule, settings)));
        set_timeout(
            move || {
                spawn_local(async move {
                    if let Err(e) = api::print_page().await {
                        self.fail(format!("Could not open printing: {e}"));
                    }
                });
            },
            Duration::from_millis(150),
        );
    }
}

pub fn parse_input(weight: &str, date: &str, time: &str, fresh: bool, breast: bool, unit: WeightUnit) -> Result<PlanInput, String> {
    let w: f64 = weight.trim().parse().map_err(|_| "Enter the turkey weight.".to_string())?;
    let weight_lb = match unit {
        WeightUnit::Lb => w,
        WeightUnit::Kg => kg_to_lb(w),
    };
    let day = NaiveDate::parse_from_str(date, "%Y-%m-%d").map_err(|_| "Pick the day you are eating.".to_string())?;
    let t = NaiveTime::parse_from_str(time, "%H:%M")
        .or_else(|_| NaiveTime::parse_from_str(time, "%H:%M:%S"))
        .map_err(|_| "Pick the time you want to carve.".to_string())?;
    Ok(PlanInput { weight_lb, carve_at: day.and_time(t), fresh, breast })
}

#[component]
fn App() -> impl IntoView {
    let ctx = Ctx::new();
    provide_context(ctx);

    spawn_local(async move {
        match api::load_state().await {
            Ok(data) => {
                ctx.time.set(data.settings.default_carve_time.clone());
                ctx.settings.set(data.settings);
                ctx.history.set(data.history);
                if data.active.is_some() {
                    ctx.panel.set(Panel::Cook);
                }
                ctx.active.set(data.active);
            }
            Err(e) => ctx.fail(format!("Could not load your saved data: {e}")),
        }
        ctx.loaded.set(true);
    });

    set_interval(move || ctx.now.set(now()), Duration::from_secs(1));

    tauri::listen("step-due", move |payload| {
        if let Ok(due) = serde_wasm_bindgen::from_value::<StepDue>(payload) {
            if ctx.settings.with_untracked(|s| s.sound) {
                sound::chime();
            }
            ctx.say(format!("{}. {}", due.title, due.body));
        }
    });

    let quit_prompt = RwSignal::new(false);
    tauri::listen("close-requested", move |_| quit_prompt.set(true));

    Effect::new(move |_| {
        let theme = ctx.settings.with(|s| s.theme.clone());
        if let Some(root) = document().document_element() {
            let _ = root.set_attribute("data-theme", &theme);
        }
    });

    view! {
        <div class="app">
            <header class="top">
                <img class="logo" src="icon.svg" alt="" />
                <div class="brand">
                    <h1>"Gobble Clock"</h1>
                    <p>"Slow roasted turkey, right on time"</p>
                </div>
                <nav>
                    {Panel::ALL
                        .into_iter()
                        .map(|p| {
                            view! {
                                <button
                                    class="tab"
                                    class:active=move || ctx.panel.get() == p
                                    on:click=move |_| ctx.panel.set(p)
                                >
                                    {p.label()}
                                    <Show when=move || p == Panel::Cook && ctx.active.with(|a| a.is_some())>
                                        <span class="live-dot" title="A cook is running"></span>
                                    </Show>
                                </button>
                            }
                        })
                        .collect_view()}
                </nav>
            </header>
            <main>
                {move || match ctx.panel.get() {
                    Panel::Plan => planner::Planner().into_any(),
                    Panel::Cook => cook::CookPanel().into_any(),
                    Panel::History => history::HistoryPanel().into_any(),
                    Panel::Recipe => recipe::RecipePanel().into_any(),
                    Panel::Settings => settings::SettingsPanel().into_any(),
                }}
            </main>
            {move || {
                ctx.toast
                    .get()
                    .map(|t| {
                        view! {
                            <div class="toast" class:bad=t.bad on:click=move |_| ctx.toast.set(None)>
                                {t.text}
                            </div>
                        }
                    })
            }}
            <Show when=move || ctx.thaw_help.get()>
                <thaw::ThawHelp />
            </Show>
            <Show when=move || quit_prompt.get()>
                <div class="modal-back">
                    <div class="modal">
                        <h2>"A turkey is cooking"</h2>
                        <p>"If you quit now, step reminders stop until you open Gobble Clock again."</p>
                        <div class="row end">
                            <button class="btn" on:click=move |_| quit_prompt.set(false)>
                                "Keep cooking"
                            </button>
                            <button
                                class="btn danger"
                                on:click=move |_| {
                                    spawn_local(async {
                                        let _ = api::quit_app().await;
                                    })
                                }
                            >
                                "Quit anyway"
                            </button>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
        <print::PrintSheet />
    }
}

fn main() {
    console_error_panic_hook_set();
    mount_to_body(App);
}

fn console_error_panic_hook_set() {
    std::panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&info.to_string().into());
    }));
}
