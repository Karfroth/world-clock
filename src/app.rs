use leptos::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use crate::time_cell::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

async fn get_cell_ids() -> Option<Vec<String>> {
    let new_msg = invoke("get_cell_ids", JsValue::undefined()).await;
    let returned = serde_wasm_bindgen::from_value::<Vec<String>>(new_msg).ok();
    returned
}

#[component]
pub fn App() -> impl IntoView {
    let (cell_ids, set_cell_ids) = create_signal(None::<Vec<String>>);

    spawn_local(async move {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        let cell_ids_vec = get_cell_ids().await;

        let asdf = cell_ids_vec.clone().unwrap_or(vec!{});

        logging::log!{"Recieved ids: {:?}", asdf};

        set_cell_ids.set(cell_ids_vec);
    });

    let get_tz = move |idx: usize| {
        cell_ids.get().and_then(|x| x.get(idx).map(|x| x.clone()))
    };

    view! {
        <main class="container">
            <Show when = move || cell_ids.get().is_some()>
                <div class="wrapper">
                    <Cell id={get_tz(0).unwrap()} />
                    <Cell id={get_tz(1).unwrap()} />
                    <Cell id={get_tz(2).unwrap()} />
                    <Cell id={get_tz(3).unwrap()} />
                </div>
            </Show>
        </main>
    }
}
