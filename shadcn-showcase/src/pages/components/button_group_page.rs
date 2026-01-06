//! ButtonGroup component showcase page

use yew::prelude::*;
use shadcn_rs::{ButtonGroup, Button, Variant, ButtonGroupOrientation};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ButtonGroupPage)]
pub fn button_group_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A group of connected buttons.",
            demo: html! {
                <ButtonGroup>
                    <Button variant={Variant::Outline}>{ "Left" }</Button>
                    <Button variant={Variant::Outline}>{ "Center" }</Button>
                    <Button variant={Variant::Outline}>{ "Right" }</Button>
                </ButtonGroup>
            },
            code: r##"<ButtonGroup>
    <Button variant={Variant::Outline}>{ "Left" }</Button>
    <Button variant={Variant::Outline}>{ "Center" }</Button>
    <Button variant={Variant::Outline}>{ "Right" }</Button>
</ButtonGroup>"##,
        },
        Example {
            title: "Vertical",
            description: "Vertically stacked button group.",
            demo: html! {
                <ButtonGroup orientation={ButtonGroupOrientation::Vertical}>
                    <Button variant={Variant::Outline}>{ "Top" }</Button>
                    <Button variant={Variant::Outline}>{ "Middle" }</Button>
                    <Button variant={Variant::Outline}>{ "Bottom" }</Button>
                </ButtonGroup>
            },
            code: r##"<ButtonGroup orientation={ButtonGroupOrientation::Vertical}>
    ...
</ButtonGroup>"##,
        },
    ];

    let props = vec![
        PropDoc { name: "orientation", prop_type: "ButtonGroupOrientation", default: "Horizontal", description: "Layout direction (Horizontal or Vertical)" },
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Button elements" },
    ];

    html! { <ComponentPage name="Button Group" description="Groups multiple buttons together." {examples} {props} /> }
}
