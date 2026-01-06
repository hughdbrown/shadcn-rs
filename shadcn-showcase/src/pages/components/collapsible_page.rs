//! Collapsible component showcase page

use yew::prelude::*;
use shadcn_rs::{Collapsible, CollapsibleTrigger, CollapsibleContent, Button};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(CollapsiblePage)]
pub fn collapsible_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A collapsible section.",
            demo: html! {
                <Collapsible class="w-[350px] space-y-2">
                    <div class="flex items-center justify-between space-x-4 px-4">
                        <h4 class="text-sm font-semibold">{ "Toggle Section" }</h4>
                        <CollapsibleTrigger>
                            <Button variant={shadcn_rs::Variant::Ghost} size={shadcn_rs::Size::Sm}>
                                { "Toggle" }
                            </Button>
                        </CollapsibleTrigger>
                    </div>
                    <div class="rounded-md border px-4 py-3 text-sm">
                        { "Always visible content" }
                    </div>
                    <CollapsibleContent>
                        <div class="rounded-md border px-4 py-3 text-sm">
                            { "Collapsible content that can be hidden" }
                        </div>
                    </CollapsibleContent>
                </Collapsible>
            },
            code: r#"<Collapsible>
    <CollapsibleTrigger>
        <Button>{ "Toggle" }</Button>
    </CollapsibleTrigger>
    <CollapsibleContent>
        { "Hidden content" }
    </CollapsibleContent>
</Collapsible>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "open", prop_type: "bool", default: "false", description: "Controlled open state" },
        PropDoc { name: "default_open", prop_type: "bool", default: "false", description: "Default open state" },
        PropDoc { name: "on_open_change", prop_type: "Callback<bool>", default: "-", description: "Open state change handler" },
    ];

    html! { <ComponentPage name="Collapsible" description="An interactive component that expands/collapses content." {examples} {props} /> }
}
