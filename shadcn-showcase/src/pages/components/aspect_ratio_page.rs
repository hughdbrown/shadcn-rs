//! AspectRatio component showcase page

use shadcn_rs::AspectRatio;
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(AspectRatioPage)]
pub fn aspect_ratio_page() -> Html {
    let examples = vec![
        Example {
            title: "16:9 Ratio",
            description: "Common video aspect ratio.",
            demo: html! {
                <div class="w-[450px]">
                    <AspectRatio ratio={16.0 / 9.0}>
                        <div class="bg-muted rounded-md flex items-center justify-center h-full">
                            { "16:9 Content" }
                        </div>
                    </AspectRatio>
                </div>
            },
            code: r#"<AspectRatio ratio={16.0 / 9.0}>
    <img src="..." alt="..." />
</AspectRatio>"#,
        },
        Example {
            title: "1:1 Square",
            description: "Square aspect ratio.",
            demo: html! {
                <div class="w-[200px]">
                    <AspectRatio ratio={1.0}>
                        <div class="bg-muted rounded-md flex items-center justify-center h-full">
                            { "1:1" }
                        </div>
                    </AspectRatio>
                </div>
            },
            code: r#"<AspectRatio ratio={1.0}>
    <div>{ "Square content" }</div>
</AspectRatio>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "ratio",
            prop_type: "f64",
            default: "1.0",
            description: "The aspect ratio (width/height)",
        },
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "Additional CSS classes",
        },
        PropDoc {
            name: "children",
            prop_type: "Children",
            default: "-",
            description: "Content to display",
        },
    ];

    html! { <ComponentPage name="Aspect Ratio" description="Displays content within a desired aspect ratio." {examples} {props} /> }
}
