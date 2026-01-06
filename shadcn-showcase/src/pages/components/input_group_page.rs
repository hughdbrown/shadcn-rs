//! InputGroup component showcase page

use yew::prelude::*;
use shadcn_rs::{InputGroup, Input, Button};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(InputGroupPage)]
pub fn input_group_page() -> Html {
    let examples = vec![
        Example {
            title: "With Prefix",
            description: "Input with a prefix addon.",
            demo: html! {
                <InputGroup prefix={html! { <span class="px-3 text-muted-foreground">{ "@" }</span> }}>
                    <Input placeholder="username" />
                </InputGroup>
            },
            code: r##"<InputGroup prefix={html! { <span>{ "@" }</span> }}>
    <Input placeholder="username" />
</InputGroup>"##,
        },
        Example {
            title: "With Suffix",
            description: "Input with a suffix addon.",
            demo: html! {
                <InputGroup addon_after={html! { <Button>{ "Search" }</Button> }}>
                    <Input placeholder="Search..." />
                </InputGroup>
            },
            code: r##"<InputGroup addon_after={html! { <Button>{ "Search" }</Button> }}>
    <Input placeholder="Search..." />
</InputGroup>"##,
        },
        Example {
            title: "URL Input",
            description: "Input for entering URLs.",
            demo: html! {
                <InputGroup
                    addon_before={html! { <span class="px-3 bg-muted text-muted-foreground">{ "https://" }</span> }}
                    suffix={html! { <span class="px-3 text-muted-foreground">{ ".com" }</span> }}
                >
                    <Input placeholder="example" />
                </InputGroup>
            },
            code: r##"<InputGroup
    addon_before={html! { <span>{ "https://" }</span> }}
    suffix={html! { <span>{ ".com" }</span> }}
>
    <Input placeholder="example" />
</InputGroup>"##,
        },
    ];

    let props = vec![
        PropDoc { name: "prefix", prop_type: "Option<Html>", default: "-", description: "Prefix content inside input" },
        PropDoc { name: "suffix", prop_type: "Option<Html>", default: "-", description: "Suffix content inside input" },
        PropDoc { name: "addon_before", prop_type: "Option<Html>", default: "-", description: "Add-on before input" },
        PropDoc { name: "addon_after", prop_type: "Option<Html>", default: "-", description: "Add-on after input" },
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Input element" },
    ];

    html! { <ComponentPage name="Input Group" description="Groups an input with addons or buttons." {examples} {props} /> }
}
