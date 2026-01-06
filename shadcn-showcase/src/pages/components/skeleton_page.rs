//! Skeleton component showcase page

use yew::prelude::*;
use shadcn_rs::Skeleton;

use crate::components::{ComponentPage, Example, PropDoc};

/// Skeleton showcase page
#[function_component(SkeletonPage)]
pub fn skeleton_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic skeleton loader.",
            demo: html! {
                <div class="flex items-center space-x-4">
                    <Skeleton class="h-12 w-12 rounded-full" />
                    <div class="space-y-2">
                        <Skeleton class="h-4 w-[250px]" />
                        <Skeleton class="h-4 w-[200px]" />
                    </div>
                </div>
            },
            code: r#"<div class="flex items-center space-x-4">
    <Skeleton class="h-12 w-12 rounded-full" />
    <div class="space-y-2">
        <Skeleton class="h-4 w-[250px]" />
        <Skeleton class="h-4 w-[200px]" />
    </div>
</div>"#,
        },
        Example {
            title: "Card Skeleton",
            description: "Skeleton for a card layout.",
            demo: html! {
                <div class="flex flex-col space-y-3">
                    <Skeleton class="h-[125px] w-[250px] rounded-xl" />
                    <div class="space-y-2">
                        <Skeleton class="h-4 w-[250px]" />
                        <Skeleton class="h-4 w-[200px]" />
                    </div>
                </div>
            },
            code: r#"<div class="flex flex-col space-y-3">
    <Skeleton class="h-[125px] w-[250px] rounded-xl" />
    <div class="space-y-2">
        <Skeleton class="h-4 w-[250px]" />
        <Skeleton class="h-4 w-[200px]" />
    </div>
</div>"#,
        },
        Example {
            title: "List Skeleton",
            description: "Skeleton for a list of items.",
            demo: html! {
                <div class="space-y-4">
                    { for (0..3).map(|_| html! {
                        <div class="flex items-center space-x-4">
                            <Skeleton class="h-10 w-10 rounded-full" />
                            <div class="space-y-2 flex-1">
                                <Skeleton class="h-4 w-3/4" />
                                <Skeleton class="h-3 w-1/2" />
                            </div>
                        </div>
                    })}
                </div>
            },
            code: r#"<div class="space-y-4">
    { for (0..3).map(|_| html! {
        <div class="flex items-center space-x-4">
            <Skeleton class="h-10 w-10 rounded-full" />
            <div class="space-y-2 flex-1">
                <Skeleton class="h-4 w-3/4" />
                <Skeleton class="h-3 w-1/2" />
            </div>
        </div>
    })}
</div>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "CSS classes to control size and shape",
        },
    ];

    html! {
        <ComponentPage
            name="Skeleton"
            description="Use to show a placeholder while content is loading."
            {examples}
            {props}
        />
    }
}
