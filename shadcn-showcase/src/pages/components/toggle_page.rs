//! Toggle component showcase page

use yew::prelude::*;
use shadcn_rs::{Toggle, ToggleVariant};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(TogglePage)]
pub fn toggle_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic toggle button.",
            demo: html! {
                <Toggle>
                    <strong>{ "B" }</strong>
                </Toggle>
            },
            code: r##"<Toggle>
    <strong>{ "B" }</strong>
</Toggle>"##,
        },
        Example {
            title: "Outline",
            description: "Toggle with outline variant.",
            demo: html! {
                <Toggle variant={ToggleVariant::Outline}>
                    <em>{ "I" }</em>
                </Toggle>
            },
            code: r##"<Toggle variant={ToggleVariant::Outline}>
    <em>{ "I" }</em>
</Toggle>"##,
        },
        Example {
            title: "Disabled",
            description: "Disabled toggle.",
            demo: html! {
                <Toggle disabled={true}>
                    { "Disabled" }
                </Toggle>
            },
            code: r##"<Toggle disabled={true}>{ "Disabled" }</Toggle>"##,
        },
        Example {
            title: "Default Pressed",
            description: "Toggle that starts in pressed state.",
            demo: html! {
                <Toggle default_pressed={true}>
                    <u>{ "U" }</u>
                </Toggle>
            },
            code: r##"<Toggle default_pressed={true}>
    <u>{ "U" }</u>
</Toggle>"##,
        },
    ];

    let props = vec![
        PropDoc { name: "pressed", prop_type: "Option<bool>", default: "-", description: "Controlled pressed state" },
        PropDoc { name: "default_pressed", prop_type: "bool", default: "false", description: "Default pressed state (uncontrolled)" },
        PropDoc { name: "disabled", prop_type: "bool", default: "false", description: "Disabled state" },
        PropDoc { name: "variant", prop_type: "ToggleVariant", default: "Default", description: "Visual variant (Default or Outline)" },
        PropDoc { name: "ontoggle", prop_type: "Option<Callback<MouseEvent>>", default: "-", description: "Toggle event handler" },
    ];

    html! { <ComponentPage name="Toggle" description="A two-state button that can be toggled on or off." {examples} {props} /> }
}
