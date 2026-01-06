//! Checkbox component showcase page

use yew::prelude::*;
use shadcn_rs::Checkbox;

use crate::components::{ComponentPage, Example, PropDoc};

/// Checkbox showcase page
#[function_component(CheckboxPage)]
pub fn checkbox_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic checkbox.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Checkbox id="terms" />
                    <label html_for="terms" class="text-sm font-medium leading-none">
                        { "Accept terms and conditions" }
                    </label>
                </div>
            },
            code: r#"<div class="flex items-center space-x-2">
    <Checkbox id="terms" />
    <label html_for="terms">
        { "Accept terms and conditions" }
    </label>
</div>"#,
        },
        Example {
            title: "Checked",
            description: "A pre-checked checkbox.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Checkbox id="checked" checked={true} />
                    <label html_for="checked" class="text-sm font-medium leading-none">
                        { "This is checked" }
                    </label>
                </div>
            },
            code: r#"<Checkbox id="checked" checked={true} />"#,
        },
        Example {
            title: "Disabled",
            description: "A disabled checkbox.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Checkbox id="disabled" disabled={true} />
                    <label html_for="disabled" class="text-sm font-medium leading-none text-muted-foreground">
                        { "Disabled checkbox" }
                    </label>
                </div>
            },
            code: r#"<Checkbox id="disabled" disabled={true} />"#,
        },
        Example {
            title: "With Description",
            description: "Checkbox with a description.",
            demo: html! {
                <div class="items-top flex space-x-2">
                    <Checkbox id="terms2" />
                    <div class="grid gap-1.5 leading-none">
                        <label html_for="terms2" class="text-sm font-medium leading-none">
                            { "Accept terms and conditions" }
                        </label>
                        <p class="text-sm text-muted-foreground">
                            { "You agree to our Terms of Service and Privacy Policy." }
                        </p>
                    </div>
                </div>
            },
            code: r#"<div class="items-top flex space-x-2">
    <Checkbox id="terms2" />
    <div class="grid gap-1.5 leading-none">
        <label html_for="terms2">
            { "Accept terms and conditions" }
        </label>
        <p class="text-sm text-muted-foreground">
            { "You agree to our Terms of Service and Privacy Policy." }
        </p>
    </div>
</div>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "checked",
            prop_type: "bool",
            default: "false",
            description: "Whether the checkbox is checked",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the checkbox is disabled",
        },
        PropDoc {
            name: "indeterminate",
            prop_type: "bool",
            default: "false",
            description: "Whether the checkbox is in indeterminate state",
        },
        PropDoc {
            name: "onchange",
            prop_type: "Callback<bool>",
            default: "-",
            description: "Change event handler",
        },
        PropDoc {
            name: "id",
            prop_type: "Option<String>",
            default: "-",
            description: "HTML id attribute",
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
            name="Checkbox"
            description="A control that allows the user to toggle between checked and not checked."
            {examples}
            {props}
        />
    }
}
