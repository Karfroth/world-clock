use leptos::*;

use chrono_tz::Tz;

#[component]
fn SelectOption(is: String, selected_item: ReadSignal<Option<String>>) -> impl IntoView {
    let tz_str: String = is.to_string();
    view! { move ||
        <option
            value=is.to_string()
            selected=selected_item.get().map(|x| &*x == &*tz_str).unwrap_or(false)
        >
            {tz_str}
        </option>
    }
}

#[component]
pub fn FilterableDropdown<F>(
    editable: ReadSignal<bool>,
    items: Vec<String>,
    selected_item: ReadSignal<Option<String>>,
    on_click: F
) -> impl IntoView where F: Fn(Option<String>) -> () + 'static {
    let (search_term, set_search_term) = create_signal(String::new());

    let update_search_term = move |ev| {
        let v = event_target_value(&ev);
        leptos::logging::log!("new_input: {}", v);
        set_search_term.set(v);
    };

    let on_select_change = move |ev| {
        let new_value = event_target_value(&ev);
        let tz: Option<Tz> = new_value.parse().ok();
        on_click(tz.map(|x| x.to_string()));
    };

    let container_class = move || if editable.get() {
        "time-selector"
    } else {
        "time-selector-hidden"
    };

    view! {
        <div class={container_class}>
            <div>
                <input on:input=update_search_term />
            </div>
            <select on:change=on_select_change>
                { move ||
                    if selected_item.get().map(|x| x.contains(&search_term.get())).unwrap_or(false) {
                        None
                    } else {
                        Some( view! {<SelectOption selected_item is={"".to_string()} />} )
                    }
                }
                { move ||
                    items
                        .iter()
                        .filter(|tz_str| {
                            let search_term_str: String = search_term.get().to_lowercase();
                            if search_term_str.len() == 0 {
                                return true
                            }
                            // let tz_str = format!("{}", x);
                            tz_str.to_lowercase().contains(&*search_term_str)
                        })
                        .map(|x| {
                            view! {
                                <SelectOption selected_item is={x.to_string()} />
                            }
                        })
                        .collect_view()
                }
            </select>
        </div>
    }
}