//! HoverCard component showcase page

use yew::prelude::*;
use shadcn_rs::{HoverCard, HoverCardTrigger, HoverCardContent, Avatar};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(HoverCardPage)]
pub fn hover_card_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A card that appears on hover.",
            demo: html! {
                <HoverCard>
                    <HoverCardTrigger>
                        <a href="javascript:void(0)" class="underline text-primary">{ "Hover over me" }</a>
                    </HoverCardTrigger>
                    <HoverCardContent class="w-80">
                        <div class="flex justify-between space-x-4">
                            <Avatar src="https://github.com/shadcn.png" initials="SC" />
                            <div class="space-y-1">
                                <h4 class="text-sm font-semibold">{ "shadcn" }</h4>
                                <p class="text-sm">
                                    { "The creator of shadcn/ui" }
                                </p>
                                <div class="flex items-center pt-2">
                                    <span class="text-xs text-muted-foreground">
                                        { "Joined December 2021" }
                                    </span>
                                </div>
                            </div>
                        </div>
                    </HoverCardContent>
                </HoverCard>
            },
            code: r##"<HoverCard>
    <HoverCardTrigger>
        <a href="#">{ "Hover over me" }</a>
    </HoverCardTrigger>
    <HoverCardContent>
        { /* Rich content */ }
    </HoverCardContent>
</HoverCard>"##,
        },
    ];

    let props = vec![
        PropDoc { name: "open_delay", prop_type: "u32", default: "200", description: "Delay before showing (ms)" },
        PropDoc { name: "close_delay", prop_type: "u32", default: "300", description: "Delay before hiding (ms)" },
    ];

    html! { <ComponentPage name="Hover Card" description="For sighted users to preview content available behind a link." {examples} {props} /> }
}
