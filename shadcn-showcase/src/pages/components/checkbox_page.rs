//! Checkbox component showcase page

use shadcn_rs::Checkbox;
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Checkbox showcase page
#[function_component(CheckboxPage)]
pub fn checkbox_page() -> Html {
    let accepted = use_state(|| false);
    let on_checked_change = {
        let accepted = accepted.clone();
        Callback::from(move |value: bool| accepted.set(value))
    };

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
            title: "Default Checked",
            description: "An uncontrolled checkbox that starts checked and toggles on its own.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Checkbox id="checked" default_checked={true} />
                    <label html_for="checked" class="text-sm font-medium leading-none">
                        { "Checked by default" }
                    </label>
                </div>
            },
            code: r#"<Checkbox id="checked" default_checked={true} />"#,
        },
        Example {
            title: "Controlled",
            description: "The parent owns the state; on_checked_change reports the new value.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Checkbox
                        id="controlled"
                        checked={*accepted}
                        on_checked_change={on_checked_change.clone()}
                    />
                    <label html_for="controlled" class="text-sm font-medium leading-none">
                        { format!("Accepted: {}", *accepted) }
                    </label>
                </div>
            },
            code: r#"let accepted = use_state(|| false);
let on_checked_change = {
    let accepted = accepted.clone();
    Callback::from(move |value: bool| accepted.set(value))
};

<Checkbox id="controlled" checked={*accepted} {on_checked_change} />"#,
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
            prop_type: "Option<bool>",
            default: "None",
            description: "Controlled checked state (leave unset for uncontrolled)",
        },
        PropDoc {
            name: "default_checked",
            prop_type: "bool",
            default: "false",
            description: "Initial checked state when uncontrolled",
        },
        PropDoc {
            name: "on_checked_change",
            prop_type: "Option<Callback<bool>>",
            default: "-",
            description: "Called with the new checked value",
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
            prop_type: "Option<Callback<Event>>",
            default: "-",
            description: "Raw change event handler",
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
