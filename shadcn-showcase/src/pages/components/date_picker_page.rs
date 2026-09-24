//! DatePicker component showcase page

use shadcn_rs::DatePicker;
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(DatePickerPage)]
pub fn date_picker_page() -> Html {
    let selected = use_state(|| Some(String::from("2026-04-17")));
    let bounded = use_state(|| None::<String>);

    let examples = vec![
        Example {
            title: "Controlled Date Picker",
            description: "The picker now uses the custom calendar popup rather than the native browser date input.",
            demo: html! {
                <div class="space-y-3">
                    <p class="text-sm text-muted-foreground">
                        { format!("Value: {}", selected.as_deref().unwrap_or("none")) }
                    </p>
                    <DatePicker
                        value={(*selected).clone()}
                        onchange={{
                            let selected = selected.clone();
                            Callback::from(move |value: String| selected.set(Some(value)))
                        }}
                        placeholder="Pick a release date"
                    />
                </div>
            },
            code: r#"<DatePicker
    value={selected.clone()}
    onchange={Callback::from(|value: String| { /* update state */ })}
    placeholder="Pick a release date"
/>"#,
        },
        Example {
            title: "Formatting And Bounds",
            description: "You can keep the stored ISO value while changing the display format and limiting the selection window.",
            demo: html! {
                <div class="space-y-3">
                    <p class="text-sm text-muted-foreground">
                        { format!("Bounded value: {}", bounded.as_deref().unwrap_or("none")) }
                    </p>
                    <DatePicker
                        value={(*bounded).clone()}
                        onchange={{
                            let bounded = bounded.clone();
                            Callback::from(move |value: String| bounded.set(Some(value)))
                        }}
                        placeholder="Choose an April 2026 date"
                        min_date={Some(AttrValue::from("2026-04-10"))}
                        max_date={Some(AttrValue::from("2026-04-30"))}
                        format="DD/MM/YYYY"
                    />
                </div>
            },
            code: r#"<DatePicker
    min_date={Some("2026-04-10".into())}
    max_date={Some("2026-04-30".into())}
    format="DD/MM/YYYY"
/>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "value",
            prop_type: "Option<AttrValue>",
            default: "None",
            description: "Controlled ISO date string (`YYYY-MM-DD`).",
        },
        PropDoc {
            name: "onchange",
            prop_type: "Option<Callback<String>>",
            default: "None",
            description: "Receives the selected ISO date string when the calendar selection changes.",
        },
        PropDoc {
            name: "min_date",
            prop_type: "Option<AttrValue>",
            default: "None",
            description: "Earliest selectable date, forwarded into the embedded calendar.",
        },
        PropDoc {
            name: "max_date",
            prop_type: "Option<AttrValue>",
            default: "None",
            description: "Latest selectable date, forwarded into the embedded calendar.",
        },
        PropDoc {
            name: "format",
            prop_type: "AttrValue",
            default: "\"MM/DD/YYYY\"",
            description: "Display-only formatting for the trigger text.",
        },
    ];

    html! {
        <ComponentPage
            name="Date Picker"
            description="A calendar-backed date picker that keeps ISO values while letting the trigger display a formatted date."
            {examples}
            {props}
        />
    }
}
