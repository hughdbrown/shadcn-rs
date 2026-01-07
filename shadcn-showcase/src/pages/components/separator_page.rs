//! Separator component showcase page

use shadcn_rs::{Separator, SeparatorOrientation};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Separator showcase page
#[function_component(SeparatorPage)]
pub fn separator_page() -> Html {
    let examples = vec![
        Example {
            title: "Horizontal",
            description: "A horizontal separator (default).",
            demo: html! {
                <div class="w-full">
                    <div class="space-y-1">
                        <h4 class="text-sm font-medium leading-none">{ "shadcn-rs" }</h4>
                        <p class="text-sm text-muted-foreground">
                            { "UI components for Rust/WebAssembly" }
                        </p>
                    </div>
                    <Separator class="my-4" />
                    <div class="flex h-5 items-center space-x-4 text-sm">
                        <div>{ "Docs" }</div>
                        <Separator orientation={SeparatorOrientation::Vertical} />
                        <div>{ "Source" }</div>
                        <Separator orientation={SeparatorOrientation::Vertical} />
                        <div>{ "Support" }</div>
                    </div>
                </div>
            },
            code: r##"<div class="space-y-1">
    <h4>{ "shadcn-rs" }</h4>
    <p>{ "UI components for Rust/WebAssembly" }</p>
</div>
<Separator class="my-4" />
<div class="flex items-center space-x-4">
    <div>{ "Docs" }</div>
    <Separator orientation={SeparatorOrientation::Vertical} />
    <div>{ "Source" }</div>
</div>"##,
        },
        Example {
            title: "Vertical",
            description: "A vertical separator.",
            demo: html! {
                <div class="flex h-10 items-center space-x-4">
                    <span>{ "Home" }</span>
                    <Separator orientation={SeparatorOrientation::Vertical} />
                    <span>{ "About" }</span>
                    <Separator orientation={SeparatorOrientation::Vertical} />
                    <span>{ "Contact" }</span>
                </div>
            },
            code: r##"<div class="flex items-center space-x-4">
    <span>{ "Home" }</span>
    <Separator orientation={SeparatorOrientation::Vertical} />
    <span>{ "About" }</span>
    <Separator orientation={SeparatorOrientation::Vertical} />
    <span>{ "Contact" }</span>
</div>"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "orientation",
            prop_type: "SeparatorOrientation",
            default: "Horizontal",
            description: "The orientation of the separator (Horizontal or Vertical)",
        },
        PropDoc {
            name: "decorative",
            prop_type: "bool",
            default: "true",
            description: "Whether the separator is purely decorative",
        },
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "Additional CSS classes",
        },
    ];

    html! {
        <ComponentPage
            name="Separator"
            description="Visually or semantically separates content."
            {examples}
            {props}
        />
    }
}
