use std::borrow::Borrow;

use leptos::leptos_dom::ev::SubmitEvent;
use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use chrono::{TimeZone, Utc};
use chrono_tz::{TZ_VARIANTS, Tz};

use crate::dropdown::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[component]
pub fn TimeComp(selected_tz: ReadSignal<Option<String>>) -> impl IntoView {
    move || {
        let tz_opt: Option<Tz> = selected_tz.get().and_then(|x| x.parse::<Tz>().ok());
        view! {
            <div>
                { tz_opt
                    .map(|tz|
                        view! {
                            <span>{Utc::now().with_timezone(tz.borrow()).to_string()}</span>
                    })
                }
            </div>
        }
    }
}

#[component]
fn Cell() -> impl IntoView {
    let tz_str = iana_time_zone::get_timezone().ok();
    let (selected_tz, set_tz) = create_signal(tz_str);
    let on_tz_select = move |tz| {
        set_tz.set(tz);
    };

    let tzs = TZ_VARIANTS
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>();
    view! {
        <div class="time-cell">
            <div class="time-span">
                <span>{move || selected_tz.get().unwrap_or("a".to_string())}</span>
                <TimeComp selected_tz />
            </div>
            <FilterableDropdown items=tzs on_click=on_tz_select selected_item=selected_tz />
        </div>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="container">
            <div class="wrapper">
                <Cell />
                <Cell />
                <Cell />
                <Cell />
            </div>
        </main>
    }
}
