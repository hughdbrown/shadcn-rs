//! Input component showcase page

use yew::prelude::*;
use shadcn_rs::Input;

use crate::components::{ComponentPage, Example, PropDoc};

/// Input showcase page
#[function_component(InputPage)]
pub fn input_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic text input.",
            demo: html! {
                <Input placeholder="Enter text..." />
            },
            code: r##"<Input placeholder="Enter text..." />"##,
        },
        Example {
            title: "Email",
            description: "Input for email addresses.",
            demo: html! {
                <Input r#type="email" placeholder="email@example.com" />
            },
            code: r##"<Input r#type="email" placeholder="email@example.com" />"##,
        },
        Example {
            title: "Password",
            description: "Password input with hidden characters.",
            demo: html! {
                <Input r#type="password" placeholder="Enter password" />
            },
            code: r##"<Input r#type="password" placeholder="Enter password" />"##,
        },
        Example {
            title: "Disabled",
            description: "A disabled input field.",
            demo: html! {
                <Input disabled={true} placeholder="Disabled" />
            },
            code: r##"<Input disabled={true} placeholder="Disabled" />"##,
        },
        Example {
            title: "With Label",
            description: "Input with an associated label.",
            demo: html! {
                <div class="grid gap-2">
                    <label html_for="email" class="text-sm font-medium">{ "Email" }</label>
                    <Input id="email" r#type="email" placeholder="you@example.com" />
                </div>
            },
            code: r##"<div class="grid gap-2">
    <Label html_for="email">{ "Email" }</Label>
    <Input id="email" r#type="email" placeholder="you@example.com" />
</div>"##,
        },
        Example {
            title: "File Input",
            description: "Input for file uploads.",
            demo: html! {
                <Input r#type="file" />
            },
            code: r##"<Input r#type="file" />"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "r#type",
            prop_type: "AttrValue",
            default: "\"text\"",
            description: "The HTML input type",
        },
        PropDoc {
            name: "placeholder",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Placeholder text",
        },
        PropDoc {
            name: "value",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Controlled value",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the input is disabled",
        },
        PropDoc {
            name: "required",
            prop_type: "bool",
            default: "false",
            description: "Whether the input is required",
        },
        PropDoc {
            name: "oninput",
            prop_type: "Option<Callback<InputEvent>>",
            default: "-",
            description: "Input change handler",
        },
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "Additional CSS classes",
        },
    ];

    html! {
        <ComponentPage
            name="Input"
            description="Displays a form input field."
            {examples}
            {props}
        />
    }
}
