use leptos::*;

use chrono_tz::Tz;

#[component]
fn SelectOption(is: String, selected_item: String) -> impl IntoView {
    let tz_str: String = is.to_string();
    view! {
        <option value=is.to_string() selected={is == selected_item} >
            {tz_str}
        </option>
    }
}

#[component]
pub fn FilterableDropdown(
    editable: bool,
    items: Vec<String>,
    selected_item: Option<String>,
    #[prop(into)] on_click: Callback<Option<String>>
) -> impl IntoView  {
    let (search_term, set_search_term) = create_signal(String::new());

    let update_search_term = move |ev| {
        let v = event_target_value(&ev);
        leptos::logging::log!("new_input: {}", v);
        set_search_term.set(v);
    };

    let on_select_change = move |ev| {
        let new_value = event_target_value(&ev);
        let tz: Option<Tz> = new_value.parse().ok();
        on_click.call(tz.map(|x| x.to_string()));
    };

    let container_class = if editable {
        "time-selector"
    } else {
        "time-selector-hidden"
    };

    let selected_item_val = selected_item.clone().unwrap_or("".to_string());

    view! {
        <div class={container_class}>
            <div>
                <input on:input=update_search_term />
            </div>
            {move ||
                view!{
                    <select on:change=on_select_change>
                        {
                            if selected_item.clone().unwrap_or("".to_string()).clone().contains(&search_term.get()) {
                                None
                            } else {
                                Some( view! {<SelectOption selected_item={selected_item_val.to_owned()} is={"".to_string()} />} )
                            }
                        }
                        {
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
                                        <SelectOption selected_item={selected_item_val.to_owned()} is={x.to_string()} />
                                    }
                                })
                                .collect_view()
                        }
                    </select>
                }
            }
        </div>
    }
}