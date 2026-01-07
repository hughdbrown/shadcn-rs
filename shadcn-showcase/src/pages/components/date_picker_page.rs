//! DatePicker component showcase page

use shadcn_rs::DatePicker;
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(DatePickerPage)]
pub fn date_picker_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A date picker with popover calendar.",
            demo: html! {
                <DatePicker placeholder="Pick a date" />
            },
            code: r#"<DatePicker placeholder="Pick a date" />"#,
        },
        Example {
            title: "With Label",
            description: "Date picker with a form label.",
            demo: html! {
                <div class="grid gap-2">
                    <label class="text-sm font-medium">{ "Date of birth" }</label>
                    <DatePicker placeholder="Select your date of birth" />
                </div>
            },
            code: r#"<div class="grid gap-2">
    <label>{ "Date of birth" }</label>
    <DatePicker placeholder="Select date" />
</div>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "value",
            prop_type: "Option<NaiveDate>",
            default: "-",
            description: "Controlled selected date",
        },
        PropDoc {
            name: "placeholder",
            prop_type: "Option<String>",
            default: "-",
            description: "Placeholder text",
        },
        PropDoc {
            name: "on_change",
            prop_type: "Callback<Option<NaiveDate>>",
            default: "-",
            description: "Date change handler",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Disable the picker",
        },
        PropDoc {
            name: "format",
            prop_type: "&str",
            default: "\"%Y-%m-%d\"",
            description: "Date format string",
        },
    ];

    html! { <ComponentPage name="Date Picker" description="A date picker component with calendar popup." {examples} {props} /> }
}
