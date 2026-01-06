//! Popover component showcase page

use yew::prelude::*;
use shadcn_rs::{Popover, PopoverTrigger, PopoverContent, Button, Input, Label};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(PopoverPage)]
pub fn popover_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic popover.",
            demo: html! {
                <Popover>
                    <PopoverTrigger>
                        <Button variant={shadcn_rs::Variant::Outline}>{ "Open Popover" }</Button>
                    </PopoverTrigger>
                    <PopoverContent class="w-80">
                        <div class="grid gap-4">
                            <div class="space-y-2">
                                <h4 class="font-medium leading-none">{ "Dimensions" }</h4>
                                <p class="text-sm text-muted-foreground">
                                    { "Set the dimensions for the layer." }
                                </p>
                            </div>
                            <div class="grid gap-2">
                                <div class="grid grid-cols-3 items-center gap-4">
                                    <Label html_for="width">{ "Width" }</Label>
                                    <Input id="width" value="100%" class="col-span-2 h-8" />
                                </div>
                                <div class="grid grid-cols-3 items-center gap-4">
                                    <Label html_for="height">{ "Height" }</Label>
                                    <Input id="height" value="25px" class="col-span-2 h-8" />
                                </div>
                            </div>
                        </div>
                    </PopoverContent>
                </Popover>
            },
            code: r#"<Popover>
    <PopoverTrigger>
        <Button>{ "Open Popover" }</Button>
    </PopoverTrigger>
    <PopoverContent>
        { "Popover content" }
    </PopoverContent>
</Popover>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "open", prop_type: "bool", default: "false", description: "Controlled open state" },
        PropDoc { name: "default_open", prop_type: "bool", default: "false", description: "Default open state" },
        PropDoc { name: "on_open_change", prop_type: "Callback<bool>", default: "-", description: "Open state change handler" },
        PropDoc { name: "side", prop_type: "Position", default: "Bottom", description: "Preferred side" },
        PropDoc { name: "align", prop_type: "Alignment", default: "Center", description: "Alignment" },
    ];

    html! { <ComponentPage name="Popover" description="Displays rich content in a portal, triggered by a button." {examples} {props} /> }
}
