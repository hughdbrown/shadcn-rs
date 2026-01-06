//! ToggleGroup component showcase page

use yew::prelude::*;
use shadcn_rs::{ToggleGroup, Toggle, ToggleGroupType, ToggleGroupOrientation};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ToggleGroupPage)]
pub fn toggle_group_page() -> Html {
    let examples = vec![
        Example {
            title: "Single",
            description: "Toggle group with single selection.",
            demo: html! {
                <ToggleGroup r#type={ToggleGroupType::Single}>
                    <Toggle>{ "Left" }</Toggle>
                    <Toggle>{ "Center" }</Toggle>
                    <Toggle>{ "Right" }</Toggle>
                </ToggleGroup>
            },
            code: r##"<ToggleGroup r#type={ToggleGroupType::Single}>
    <Toggle>{ "Left" }</Toggle>
    <Toggle>{ "Center" }</Toggle>
    <Toggle>{ "Right" }</Toggle>
</ToggleGroup>"##,
        },
        Example {
            title: "Multiple",
            description: "Toggle group with multiple selection.",
            demo: html! {
                <ToggleGroup r#type={ToggleGroupType::Multiple}>
                    <Toggle><strong>{ "B" }</strong></Toggle>
                    <Toggle><em>{ "I" }</em></Toggle>
                    <Toggle><u>{ "U" }</u></Toggle>
                </ToggleGroup>
            },
            code: r##"<ToggleGroup r#type={ToggleGroupType::Multiple}>
    <Toggle><strong>{ "B" }</strong></Toggle>
    <Toggle><em>{ "I" }</em></Toggle>
    <Toggle><u>{ "U" }</u></Toggle>
</ToggleGroup>"##,
        },
        Example {
            title: "Vertical",
            description: "Vertically oriented toggle group.",
            demo: html! {
                <ToggleGroup
                    r#type={ToggleGroupType::Single}
                    orientation={ToggleGroupOrientation::Vertical}
                >
                    <Toggle>{ "Top" }</Toggle>
                    <Toggle>{ "Middle" }</Toggle>
                    <Toggle>{ "Bottom" }</Toggle>
                </ToggleGroup>
            },
            code: r##"<ToggleGroup
    r#type={ToggleGroupType::Single}
    orientation={ToggleGroupOrientation::Vertical}
>
    <Toggle>{ "Top" }</Toggle>
    <Toggle>{ "Middle" }</Toggle>
    <Toggle>{ "Bottom" }</Toggle>
</ToggleGroup>"##,
        },
    ];

    let props = vec![
        PropDoc { name: "r#type", prop_type: "ToggleGroupType", default: "Single", description: "Selection mode (Single/Multiple)" },
        PropDoc { name: "orientation", prop_type: "ToggleGroupOrientation", default: "Horizontal", description: "Layout orientation" },
        PropDoc { name: "value", prop_type: "Option<AttrValue>", default: "-", description: "Controlled value (single)" },
        PropDoc { name: "values", prop_type: "Option<Vec<AttrValue>>", default: "-", description: "Controlled values (multiple)" },
        PropDoc { name: "on_value_change", prop_type: "Option<Callback<AttrValue>>", default: "-", description: "Value change handler" },
        PropDoc { name: "disabled", prop_type: "bool", default: "false", description: "Disabled state" },
    ];

    html! { <ComponentPage name="Toggle Group" description="A set of two-state buttons that can be toggled." {examples} {props} /> }
}
