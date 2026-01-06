//! Label component showcase page

use yew::prelude::*;
use shadcn_rs::{Label, Input, Checkbox};

use crate::components::{ComponentPage, Example, PropDoc};

/// Label showcase page
#[function_component(LabelPage)]
pub fn label_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic form label.",
            demo: html! {
                <div class="grid gap-2">
                    <Label html_for="email">{ "Email" }</Label>
                    <Input id="email" placeholder="you@example.com" />
                </div>
            },
            code: r#"<Label html_for="email">{ "Email" }</Label>
<Input id="email" placeholder="you@example.com" />"#,
        },
        Example {
            title: "Required",
            description: "Label with required indicator.",
            demo: html! {
                <div class="grid gap-2">
                    <Label html_for="name" required={true}>{ "Name" }</Label>
                    <Input id="name" placeholder="Enter your name" required={true} />
                </div>
            },
            code: r#"<Label html_for="name" required={true}>{ "Name" }</Label>
<Input id="name" required={true} />"#,
        },
        Example {
            title: "With Checkbox",
            description: "Label used with a checkbox.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Checkbox id="terms" />
                    <Label html_for="terms">{ "Accept terms and conditions" }</Label>
                </div>
            },
            code: r#"<div class="flex items-center space-x-2">
    <Checkbox id="terms" />
    <Label html_for="terms">{ "Accept terms and conditions" }</Label>
</div>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "html_for",
            prop_type: "Option<String>",
            default: "-",
            description: "The ID of the form element this label is for",
        },
        PropDoc {
            name: "required",
            prop_type: "bool",
            default: "false",
            description: "Whether to show a required indicator",
        },
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "Additional CSS classes",
        },
        PropDoc {
            name: "children",
            prop_type: "Children",
            default: "-",
            description: "Label text content",
        },
    ];

    html! {
        <ComponentPage
            name="Label"
            description="Renders an accessible label associated with controls."
            {examples}
            {props}
        />
    }
}
