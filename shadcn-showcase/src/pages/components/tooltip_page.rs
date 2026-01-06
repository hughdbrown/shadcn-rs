//! Tooltip component showcase page

use yew::prelude::*;
use shadcn_rs::{Tooltip, TooltipTrigger, TooltipContent, Button, Position};

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
    <TooltipContent side="top">{ "Tooltip" }</TooltipContent>
</Tooltip>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "delay_duration", prop_type: "u32", default: "200", description: "Delay before showing (ms)" },
        PropDoc { name: "side", prop_type: "&str", default: "\"top\"", description: "Preferred side" },
    ];

    html! { <ComponentPage name="Tooltip" description="A popup that displays information on hover." {examples} {props} /> }
}
