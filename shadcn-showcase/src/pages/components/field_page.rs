//! Field component showcase page

use shadcn_rs::{Field, Input};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(FieldPage)]
pub fn field_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A form field with label and input.",
            demo: html! {
                <Field label="Email" id="email">
                    <Input id="email" r#type="email" placeholder="you@example.com" />
                </Field>
            },
            code: r##"<Field label="Email" id="email">
    <Input id="email" r#type="email" />
</Field>"##,
        },
        Example {
            title: "Required",
            description: "Field with required indicator.",
            demo: html! {
                <Field label="Name" id="name" required={true}>
                    <Input id="name" required={true} />
                </Field>
            },
            code: r##"<Field label="Name" required={true}>
    <Input required={true} />
</Field>"##,
        },
        Example {
            title: "With Error",
            description: "Field with error message.",
            demo: html! {
                <Field label="Password" id="password" error={Some("Password must be at least 8 characters".to_string())}>
                    <Input id="password" r#type="password" />
                </Field>
            },
            code: r##"<Field label="Password" error={Some("Error message".to_string())}>
    <Input r#type="password" />
</Field>"##,
        },
        Example {
            title: "With Help Text",
            description: "Field with description.",
            demo: html! {
                <Field label="Username" id="username" help_text="This is your public display name.">
                    <Input id="username" placeholder="shadcn" />
                </Field>
            },
            code: r##"<Field label="Username" help_text="Help text">
    <Input />
</Field>"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "label",
            prop_type: "AttrValue",
            default: "-",
            description: "Field label text",
        },
        PropDoc {
            name: "id",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Input element ID",
        },
        PropDoc {
            name: "required",
            prop_type: "bool",
            default: "false",
            description: "Show required indicator",
        },
        PropDoc {
            name: "error",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Error message",
        },
        PropDoc {
            name: "help_text",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Help text",
        },
    ];

    html! { <ComponentPage name="Field" description="A wrapper for form fields with label, input, and error handling." {examples} {props} /> }
}
