//! Calendar component showcase page

use shadcn_rs::{Calendar, CalendarMode};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(CalendarPage)]
pub fn calendar_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A date picker calendar.",
            demo: html! {
                <Calendar class="rounded-md border" />
            },
            code: r#"<Calendar />"#,
        },
        Example {
            title: "With Selection",
            description: "Calendar with date selection.",
            demo: html! {
                <Calendar
                    mode={CalendarMode::Single}
                    class="rounded-md border"
                />
            },
            code: r#"<Calendar mode="single" selected={selected} on_select={on_select} />"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "mode",
            prop_type: "&str",
            default: "\"single\"",
            description: "Selection mode (single, multiple, range)",
        },
        PropDoc {
            name: "selected",
            prop_type: "Option<NaiveDate>",
            default: "-",
            description: "Selected date(s)",
        },
        PropDoc {
            name: "on_select",
            prop_type: "Callback<NaiveDate>",
            default: "-",
            description: "Selection handler",
        },
        PropDoc {
            name: "disabled_dates",
            prop_type: "Vec<NaiveDate>",
            default: "[]",
            description: "Dates to disable",
        },
        PropDoc {
            name: "min_date",
            prop_type: "Option<NaiveDate>",
            default: "-",
            description: "Minimum selectable date",
        },
        PropDoc {
            name: "max_date",
            prop_type: "Option<NaiveDate>",
            default: "-",
            description: "Maximum selectable date",
        },
    ];

    html! { <ComponentPage name="Calendar" description="A date field component that allows users to enter and edit date." {examples} {props} /> }
}
