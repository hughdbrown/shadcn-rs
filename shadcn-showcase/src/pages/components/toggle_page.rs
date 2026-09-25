//! Toggle component showcase page

use shadcn_rs::{Toggle, ToggleVariant};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(TogglePage)]
pub fn toggle_page() -> Html {
    let bookmarked = use_state(|| false);
    let on_pressed_change = {
        let bookmarked = bookmarked.clone();
        Callback::from(move |value: bool| bookmarked.set(value))
    };

    let examples = vec![
        Example {
            title: "Default",
            description: "A basic toggle button.",
            demo: html! {
                <Toggle aria_label="Toggle bold">
                    <strong>{ "B" }</strong>
                </Toggle>
            },
            code: r##"<Toggle aria_label="Toggle bold">
    <strong>{ "B" }</strong>
</Toggle>"##,
        },
        Example {
            title: "Outline",
            description: "Toggle with outline variant.",
            demo: html! {
                <Toggle variant={ToggleVariant::Outline} aria_label="Toggle italic">
                    <em>{ "I" }</em>
                </Toggle>
            },
            code: r##"<Toggle variant={ToggleVariant::Outline} aria_label="Toggle italic">
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
                <Toggle default_pressed={true} aria_label="Toggle underline">
                    <u>{ "U" }</u>
                </Toggle>
            },
            code: r##"<Toggle default_pressed={true} aria_label="Toggle underline">
    <u>{ "U" }</u>
</Toggle>"##,
        },
        Example {
            title: "Controlled",
            description: "The parent owns the state; on_pressed_change reports the new value.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Toggle
                        pressed={*bookmarked}
                        on_pressed_change={on_pressed_change.clone()}
                        aria_label="Toggle bookmark"
                    >
                        { "★" }
                    </Toggle>
                    <span class="text-sm text-muted-foreground">
                        { if *bookmarked { "Bookmarked" } else { "Not bookmarked" } }
                    </span>
                </div>
            },
            code: r##"let bookmarked = use_state(|| false);
let on_pressed_change = {
    let bookmarked = bookmarked.clone();
    Callback::from(move |value: bool| bookmarked.set(value))
};

<Toggle pressed={*bookmarked} {on_pressed_change} aria_label="Toggle bookmark">
    { "★" }
</Toggle>"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "pressed",
            prop_type: "Option<bool>",
            default: "-",
            description: "Controlled pressed state",
        },
        PropDoc {
            name: "default_pressed",
            prop_type: "bool",
            default: "false",
            description: "Default pressed state (uncontrolled)",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Disabled state",
        },
        PropDoc {
            name: "variant",
            prop_type: "ToggleVariant",
            default: "Default",
            description: "Visual variant (Default or Outline)",
        },
        PropDoc {
            name: "ontoggle",
            prop_type: "Option<Callback<MouseEvent>>",
            default: "-",
            description: "Raw click handler",
        },
        PropDoc {
            name: "on_pressed_change",
            prop_type: "Option<Callback<bool>>",
            default: "-",
            description: "Called with the new pressed value",
        },
        PropDoc {
            name: "aria_label",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Accessible name for icon-only toggles",
        },
    ];

    html! { <ComponentPage name="Toggle" description="A two-state button that can be toggled on or off." {examples} {props} /> }
}
