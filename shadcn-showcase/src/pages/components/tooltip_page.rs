//! Tooltip component showcase page

use shadcn_rs::{Button, Position, Tooltip, TooltipContent, TooltipTrigger};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(TooltipPage)]
pub fn tooltip_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic tooltip.",
            demo: html! {
                <Tooltip>
                    <TooltipTrigger>
                        <Button variant={shadcn_rs::Variant::Outline}>{ "Hover me" }</Button>
                    </TooltipTrigger>
                    <TooltipContent>
                        <p>{ "Add to library" }</p>
                    </TooltipContent>
                </Tooltip>
            },
            code: r#"<Tooltip>
    <TooltipTrigger>
        <Button>{ "Hover me" }</Button>
    </TooltipTrigger>
    <TooltipContent>
        <p>{ "Tooltip text" }</p>
    </TooltipContent>
</Tooltip>"#,
        },
        Example {
            title: "Positions",
            description: "Tooltips in different positions.",
            demo: html! {
                <div class="flex gap-4">
                    <Tooltip>
                        <TooltipTrigger>
                            <Button variant={shadcn_rs::Variant::Outline}>{ "Top" }</Button>
                        </TooltipTrigger>
                        <TooltipContent position={Position::Top}>{ "Top tooltip" }</TooltipContent>
                    </Tooltip>
                    <Tooltip>
                        <TooltipTrigger>
                            <Button variant={shadcn_rs::Variant::Outline}>{ "Right" }</Button>
                        </TooltipTrigger>
                        <TooltipContent position={Position::Right}>{ "Right tooltip" }</TooltipContent>
                    </Tooltip>
                    <Tooltip>
                        <TooltipTrigger>
                            <Button variant={shadcn_rs::Variant::Outline}>{ "Bottom" }</Button>
                        </TooltipTrigger>
                        <TooltipContent position={Position::Bottom}>{ "Bottom tooltip" }</TooltipContent>
                    </Tooltip>
                    <Tooltip>
                        <TooltipTrigger>
                            <Button variant={shadcn_rs::Variant::Outline}>{ "Left" }</Button>
                        </TooltipTrigger>
                        <TooltipContent position={Position::Left}>{ "Left tooltip" }</TooltipContent>
                    </Tooltip>
                </div>
            },
            code: r#"<Tooltip>
    <TooltipTrigger>...</TooltipTrigger>
    <TooltipContent position={Position::Top}>{ "Tooltip" }</TooltipContent>
</Tooltip>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "delay_duration",
            prop_type: "Option<u32>",
            default: "provider or 200",
            description: "Hover delay before showing (ms); focus shows immediately",
        },
        PropDoc {
            name: "open",
            prop_type: "Option<bool>",
            default: "None",
            description: "Controlled open state (Some = controlled)",
        },
        PropDoc {
            name: "default_open",
            prop_type: "bool",
            default: "false",
            description: "Initial open state when uncontrolled",
        },
        PropDoc {
            name: "on_open_change",
            prop_type: "Callback<bool>",
            default: "-",
            description: "Called on hover, focus, leave, blur and Escape",
        },
        PropDoc {
            name: "position",
            prop_type: "Position",
            default: "Top",
            description: "TooltipContent: side of the trigger",
        },
    ];

    html! { <ComponentPage name="Tooltip" description="A popup that displays information on hover or keyboard focus." {examples} {props} /> }
}
