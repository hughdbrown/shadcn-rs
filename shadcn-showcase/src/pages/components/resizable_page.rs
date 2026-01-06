//! Resizable component showcase page

use yew::prelude::*;
use shadcn_rs::{Resizable, ResizablePanel, ResizableHandle, ResizableOrientation};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ResizablePage)]
pub fn resizable_page() -> Html {
    let examples = vec![
        Example {
            title: "Horizontal",
            description: "Horizontally resizable panels.",
            demo: html! {
                <Resizable orientation={ResizableOrientation::Horizontal} class="min-h-[200px] max-w-md rounded-lg border">
                    <ResizablePanel index={0} default_size={50.0}>
                        <div class="flex h-full items-center justify-center p-6">
                            <span class="font-semibold">{ "One" }</span>
                        </div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel index={1} default_size={50.0}>
                        <div class="flex h-full items-center justify-center p-6">
                            <span class="font-semibold">{ "Two" }</span>
                        </div>
                    </ResizablePanel>
                </Resizable>
            },
            code: r##"<Resizable orientation={ResizableOrientation::Horizontal}>
    <ResizablePanel index={0} default_size={50.0}>
        { "Panel One" }
    </ResizablePanel>
    <ResizableHandle />
    <ResizablePanel index={1} default_size={50.0}>
        { "Panel Two" }
    </ResizablePanel>
</Resizable>"##,
        },
        Example {
            title: "Vertical",
            description: "Vertically resizable panels.",
            demo: html! {
                <Resizable orientation={ResizableOrientation::Vertical} class="min-h-[200px] max-w-md rounded-lg border">
                    <ResizablePanel index={0} default_size={25.0}>
                        <div class="flex h-full items-center justify-center p-6">
                            <span class="font-semibold">{ "Header" }</span>
                        </div>
                    </ResizablePanel>
                    <ResizableHandle />
                    <ResizablePanel index={1} default_size={75.0}>
                        <div class="flex h-full items-center justify-center p-6">
                            <span class="font-semibold">{ "Content" }</span>
                        </div>
                    </ResizablePanel>
                </Resizable>
            },
            code: r##"<Resizable orientation={ResizableOrientation::Vertical}>
    <ResizablePanel index={0} default_size={25.0}>
        { "Header" }
    </ResizablePanel>
    <ResizableHandle />
    <ResizablePanel index={1} default_size={75.0}>
        { "Content" }
    </ResizablePanel>
</Resizable>"##,
        },
    ];

    let props = vec![
        PropDoc { name: "orientation", prop_type: "ResizableOrientation", default: "Horizontal", description: "Panel orientation (Horizontal or Vertical)" },
        PropDoc { name: "index", prop_type: "usize", default: "0", description: "Panel index (0 or 1)" },
        PropDoc { name: "default_size", prop_type: "f64", default: "50.0", description: "Default panel size %" },
        PropDoc { name: "min_size", prop_type: "f64", default: "10.0", description: "Minimum panel size %" },
        PropDoc { name: "max_size", prop_type: "f64", default: "90.0", description: "Maximum panel size %" },
    ];

    html! { <ComponentPage name="Resizable" description="Resizable panel groups with drag handles." {examples} {props} /> }
}
