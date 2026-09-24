//! Calendar component showcase page

use shadcn_rs::{Calendar, CalendarMode};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(CalendarPage)]
pub fn calendar_page() -> Html {
    let single_value = use_state(|| Some(String::from("2026-04-17")));
    let multi_value = use_state(|| Some(String::from("2026-04-17,2026-04-18")));
    let range_value = use_state(|| Some(String::from("2026-04-17..2026-04-22")));

    let examples = vec![
        Example {
            title: "Single Selection",
            description: "Single-date mode uses a single ISO date string and can show multiple months at once.",
            demo: html! {
                <div class="space-y-3">
                    <p class="text-sm text-muted-foreground">
                        { format!("Selected: {}", single_value.as_deref().unwrap_or("none")) }
                    </p>
                    <Calendar
                        mode={CalendarMode::Single}
                        selected={(*single_value).clone()}
                        onselect={{
                            let single_value = single_value.clone();
                            Callback::from(move |value: String| single_value.set(Some(value)))
                        }}
                        number_of_months={2}
                        class="rounded-md border"
                    />
                </div>
            },
            code: r#"<Calendar
    mode={CalendarMode::Single}
    selected={Some("2026-04-17".into())}
    onselect={Callback::from(|value| { /* update state */ })}
    number_of_months={2}
/>"#,
        },
        Example {
            title: "Multiple And Range Encodings",
            description: "Multiple mode uses comma-separated ISO dates. Range mode uses `start..end`.",
            demo: html! {
                <div class="grid gap-6 lg:grid-cols-2">
                    <div class="space-y-3">
                        <p class="text-sm text-muted-foreground">
                            { format!("Multiple: {}", multi_value.as_deref().unwrap_or("none")) }
                        </p>
                        <Calendar
                            mode={CalendarMode::Multiple}
                            selected={(*multi_value).clone()}
                            onselect={{
                                let multi_value = multi_value.clone();
                                Callback::from(move |value: String| multi_value.set(Some(value)))
                            }}
                            disabled_dates={vec!["2026-04-20".into()]}
                            class="rounded-md border"
                        />
                    </div>
                    <div class="space-y-3">
                        <p class="text-sm text-muted-foreground">
                            { format!("Range: {}", range_value.as_deref().unwrap_or("none")) }
                        </p>
                        <Calendar
                            mode={CalendarMode::Range}
                            selected={(*range_value).clone()}
                            onselect={{
                                let range_value = range_value.clone();
                                Callback::from(move |value: String| range_value.set(Some(value)))
                            }}
                            min_date={Some(AttrValue::from("2026-04-10"))}
                            max_date={Some(AttrValue::from("2026-04-30"))}
                            show_week_numbers={true}
                            class="rounded-md border"
                        />
                    </div>
                </div>
            },
            code: r#"<Calendar
    mode={CalendarMode::Multiple}
    selected={Some("2026-04-17,2026-04-18".into())}
/>

<Calendar
    mode={CalendarMode::Range}
    selected={Some("2026-04-17..2026-04-22".into())}
/>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "mode",
            prop_type: "CalendarMode",
            default: "CalendarMode::Single",
            description: "Controls how selections are encoded and interpreted.",
        },
        PropDoc {
            name: "selected",
            prop_type: "Option<AttrValue>",
            default: "None",
            description: "Single: `YYYY-MM-DD`, multiple: comma-separated dates, range: `start..end`.",
        },
        PropDoc {
            name: "onselect",
            prop_type: "Option<Callback<String>>",
            default: "None",
            description: "Receives the encoded next selection string.",
        },
        PropDoc {
            name: "number_of_months",
            prop_type: "u8",
            default: "1",
            description: "Renders side-by-side month panels.",
        },
        PropDoc {
            name: "show_week_numbers",
            prop_type: "bool",
            default: "false",
            description: "Displays a week-number column before the weekday columns.",
        },
    ];

    let notes = html! {
        <div class="space-y-3">
            <p>{ "The component now derives its initial visible month from the current selection or the current date instead of using a fixed month/year." }</p>
            <p>{ "Range selection is inclusive and renders the entire selected span, with explicit boundary styling for the start and end dates." }</p>
        </div>
    };

    html! {
        <ComponentPage
            name="Calendar"
            description="A calendar component with single, multiple, and range selection modes."
            {examples}
            {props}
            notes={Some(notes)}
        />
    }
}
