use core::time;
use std::borrow::Borrow;

use leptos::*;

use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;

use chrono::Utc;
use chrono_tz::{TZ_VARIANTS, Tz};

use crate::dropdown::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GetTZArgs {
    id: String,
}

async fn get_tz(id: String) -> Option<String> {
    let params = GetTZArgs { id };
    let new_msg = invoke("get_tz", to_value(&params).unwrap()).await;
    let returned = serde_wasm_bindgen::from_value::<Vec<String>>(new_msg).ok();
    let tz_opt = returned.and_then(|x| x.get(0).map(|y| y.to_owned()));
    if tz_opt.is_some() {
        tz_opt
    } else {
        iana_time_zone::get_timezone().ok()
    }
}

#[component]
fn TimeComp(selected_tz: Option<String>) -> impl IntoView {
    let get_time = move || {
        selected_tz.clone().and_then(|x| x.parse::<Tz>().ok()).map(|tz|
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
fn InnerCell(initial_tz: Option<String>) -> impl IntoView {
    let tz_str = if initial_tz.is_some() { initial_tz } else { iana_time_zone::get_timezone().ok() };
    let (selected_tz, set_tz) = create_signal(tz_str);
    let on_tz_select = move |tz| {
        set_tz.set(tz);
    };

    move || {
        view! {
            <div class="time-cell">
                <div class="time-span">
                    <span>{selected_tz.get().unwrap_or("a".to_string())}</span>
                    <TimeComp selected_tz={selected_tz.get()} />
                </div>
                <CellEdit on_select=on_tz_select selected_tz />
            </div>
        }
    }
}

#[component]
pub fn Cell(id: String) -> impl IntoView {
    let (initial_tz, set_initial_tz) = create_signal(None::<String>);

    spawn_local(async move {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        let tz = get_tz(id).await;

        set_initial_tz.set(tz);
    });

    view! {
        <Show when=move || initial_tz.get().is_some()>
            <InnerCell initial_tz={initial_tz.get()} />
        </Show>
    }
}