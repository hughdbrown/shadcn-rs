//! ToggleGroup component showcase page

use yew::prelude::*;
use shadcn_rs::{ToggleGroup, ToggleGroupItem, ToggleGroupType, ToggleGroupOrientation};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ToggleGroupPage)]
pub fn toggle_group_page() -> Html {
    let examples = vec![
        Example {
            title: "Single",
            description: "Toggle group with single selection.",
            demo: html! {
                <ToggleGroup r#type={ToggleGroupType::Single}>
                    <ToggleGroupItem value="left">{ "Left" }</ToggleGroupItem>
                    <ToggleGroupItem value="center">{ "Center" }</ToggleGroupItem>
                    <ToggleGroupItem value="right">{ "Right" }</ToggleGroupItem>
                </ToggleGroup>
            },
            code: r##"<ToggleGroup r#type={ToggleGroupType::Single}>
    <ToggleGroupItem value="left">{ "Left" }</ToggleGroupItem>
    <ToggleGroupItem value="center">{ "Center" }</ToggleGroupItem>
    <ToggleGroupItem value="right">{ "Right" }</ToggleGroupItem>
</ToggleGroup>"##,
        },
        Example {
            title: "Multiple",
            description: "Toggle group with multiple selection.",
            demo: html! {
                <ToggleGroup r#type={ToggleGroupType::Multiple}>
                    <ToggleGroupItem value="bold"><strong>{ "B" }</strong></ToggleGroupItem>
                    <ToggleGroupItem value="italic"><em>{ "I" }</em></ToggleGroupItem>
                    <ToggleGroupItem value="underline"><u>{ "U" }</u></ToggleGroupItem>
                </ToggleGroup>
            },
            code: r##"<ToggleGroup r#type={ToggleGroupType::Multiple}>
    <ToggleGroupItem value="bold"><strong>{ "B" }</strong></ToggleGroupItem>
    <ToggleGroupItem value="italic"><em>{ "I" }</em></ToggleGroupItem>
    <ToggleGroupItem value="underline"><u>{ "U" }</u></ToggleGroupItem>
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
                    <ToggleGroupItem value="top">{ "Top" }</ToggleGroupItem>
                    <ToggleGroupItem value="middle">{ "Middle" }</ToggleGroupItem>
                    <ToggleGroupItem value="bottom">{ "Bottom" }</ToggleGroupItem>
                </ToggleGroup>
            },
            code: r##"<ToggleGroup
    r#type={ToggleGroupType::Single}
    orientation={ToggleGroupOrientation::Vertical}
>
    <ToggleGroupItem value="top">{ "Top" }</ToggleGroupItem>
    <ToggleGroupItem value="middle">{ "Middle" }</ToggleGroupItem>
    <ToggleGroupItem value="bottom">{ "Bottom" }</ToggleGroupItem>
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
