use core::time;
use std::borrow::Borrow;

use leptos::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

use chrono::Utc;
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
    let get_time = move || {
        selected_tz.get().and_then(|x| x.parse::<Tz>().ok()).map(|tz|
            Utc::now().with_timezone(tz.borrow()).to_string()
        )
    };
    let (time, set_time) = create_signal(get_time().unwrap_or("".to_string()));
    set_interval(move || {
        set_time.set(get_time().unwrap_or("".to_string()))
    }, time::Duration::new(1, 0));
    move || {
        view! {
            <div>
                <span>{time.get()}</span>
            </div>
        }
    }
}

#[component]
fn CellEdit<F: Fn(Option<String>) + 'static>(
    on_select: F,
    selected_tz: ReadSignal<Option<String>>) -> impl IntoView {
    let tzs = TZ_VARIANTS
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>();

    let (editable, set_editable) = create_signal(false);

    let toggle_editable = move |_ev| {
        set_editable.set(!editable.get());
    };

    let button_text = move || {
        if editable.get() {
            "Confirm"
        } else {
            "Edit"
        }
    };

    view! {
        <div>
            <button on:click=toggle_editable>{button_text}</button>
            <FilterableDropdown editable items=tzs on_click=on_select selected_item=selected_tz/>
        </div>
    }
}

#[component]
fn Cell() -> impl IntoView {
    let tz_str = iana_time_zone::get_timezone().ok();
    let (selected_tz, set_tz) = create_signal(tz_str);
    let on_tz_select = move |tz| {
        set_tz.set(tz);
    };

    view! {
        <div class="time-cell">
            <div class="time-span">
                <span>{move || selected_tz.get().unwrap_or("a".to_string())}</span>
                <TimeComp selected_tz />
            </div>
            <CellEdit on_select=on_tz_select selected_tz />
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
