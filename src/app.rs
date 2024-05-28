use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::time_cell::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GetTZArgs {
    idx: i32,
}

#[component]
pub fn App() -> impl IntoView {
    let (initial_tzs, set_initial_tzs) = create_signal(None::<Vec<String>>);

    spawn_local(async move {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        let new_msg = invoke("get_tzs", JsValue::undefined()).await;
        let returned = serde_wasm_bindgen::from_value::<Vec<String>>(new_msg).ok();

        set_initial_tzs.set(returned);
    });

    let get_tz = move |idx: usize| {
        initial_tzs.get().and_then(|x| x.get(idx).map(|x| x.clone()))
    };

    view! {
        <main class="container">
            <Show when = move || initial_tzs.get().is_some()>
                <div class="wrapper">
                    <Cell initial_tz={get_tz(0).unwrap_or(iana_time_zone::get_timezone().unwrap())} />
                    <Cell initial_tz={get_tz(1).unwrap_or(iana_time_zone::get_timezone().unwrap())} />
                    <Cell initial_tz={get_tz(2).unwrap_or(iana_time_zone::get_timezone().unwrap())} />
                    <Cell initial_tz={get_tz(3).unwrap_or(iana_time_zone::get_timezone().unwrap())} />
                </div>
            </Show>
        </main>
    }
}
