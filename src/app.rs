use std::borrow::Borrow;

use leptos::leptos_dom::ev::SubmitEvent;
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
    move || {
        let tz_opt: Option<Tz> = selected_tz.get().and_then(|x| x.parse::<Tz>().ok());
        view! {
            <>
                { tz_opt
                    .map(|tz|
                        view! {
                            <span>{Utc::now().with_timezone(tz.borrow()).to_string()}</span>
                    })
                }
            </>
        }
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (greet_msg, set_greet_msg) = create_signal(String::new());

    let update_name = move |ev| {
        let v = event_target_value(&ev);
        set_name.set(v);
    };

    let greet = move |ev: SubmitEvent| {
        ev.prevent_default();
        spawn_local(async move {
            let name = name.get_untracked();
            if name.is_empty() {
                return;
            }

            let args = to_value(&GreetArgs { name: &name }).unwrap();
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            let new_msg = invoke("greet", args).await.as_string().unwrap();
            set_greet_msg.set(new_msg);
        });
    };

    let (selected_tz, set_tz) = create_signal(None::<String>);

    let on_tz_select = move |tz| {
        set_tz.set(tz);
    };

    let tzs = TZ_VARIANTS
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>();

    view! {
        <main class="container">
            <div class="row">
                <a href="https://tauri.app" target="_blank">
                    <img src="public/tauri.svg" class="logo tauri" alt="Tauri logo"/>
                </a>
                <a href="https://docs.rs/leptos/" target="_blank">
                    <img src="public/leptos.svg" class="logo leptos" alt="Leptos logo"/>
                </a>
            </div>

            <p>"Click on the Tauri and Leptos logos to learn more."</p>

            <p>
                "Recommended IDE setup: "
                <a href="https://code.visualstudio.com/" target="_blank">"VS Code"</a>
                " + "
                <a href="https://github.com/tauri-apps/tauri-vscode" target="_blank">"Tauri"</a>
                " + "
                <a href="https://github.com/rust-lang/rust-analyzer" target="_blank">"rust-analyzer"</a>
            </p>

            <form class="row" on:submit=greet>
                <input
                    id="greet-input"
                    placeholder="Enter a name..."
                    on:input=update_name
                />
                <button type="submit">"Greet"</button>
            </form>

            <p><b>{ move || greet_msg.get() }</b></p>

            <span>{move || selected_tz.get().unwrap_or("a".to_string())}</span>
            <TimeComp selected_tz />
            <FilterableDropdown items=tzs on_click=on_tz_select selected_item=selected_tz />
        </main>
    }
}
