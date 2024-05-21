use leptos::*;

use chrono_tz::{TZ_VARIANTS, Tz};

#[component]
fn SelectOption(is: & 'static Tz, selected_tz: ReadSignal<Option<Tz>>) -> impl IntoView {
    view! {
        <option
            value=is.to_string()
            selected=move || selected_tz.get().map(|x| x == *is).unwrap_or(false)
        >
            {is.to_string()}
        </option>
    }
}

#[component]
pub fn TZDropdown<F>(
    selected_tz: ReadSignal<Option<Tz>>,
    on_click: F,
) -> impl IntoView where F: Fn(Option<Tz>) -> () + 'static {
    let (search_term, set_search_term) = create_signal(String::new());

    let update_search_term = move |ev| {
        let v = event_target_value(&ev);
        leptos::logging::log!("new_input: {}", v);
        set_search_term.set(v);
    };

    let on_select_change = move |ev| {
        let new_value = event_target_value(&ev);
        let tz: Option<Tz> = new_value.parse().ok();
        on_click(tz);
    };

    view! {
        <>
            <input
                id="asdf"
                on:input=update_search_term
            />
            <select on:change=on_select_change>
                { move || 
                    TZ_VARIANTS
                        .iter()
                        .filter(|x| {
                            let search_term_str: String = search_term.get().to_lowercase();
                            if search_term_str.len() == 0 {
                                return true
                            }
                            let tz_str = format!("{}", x);
                            tz_str.to_lowercase().contains(&*search_term_str)
                        })
                        .map(|x| {
                            view! {
                                <SelectOption selected_tz is={x} />
                            }
                        })
                        .collect_view()
                }
            </select>
        </>
    }
}