//! ScrollArea component showcase page

use yew::prelude::*;
use shadcn_rs::ScrollArea;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ScrollAreaPage)]
pub fn scroll_area_page() -> Html {
    let examples = vec![
        Example {
            title: "Vertical",
            description: "Vertical scrolling area.",
            demo: html! {
                <ScrollArea class="h-[200px] w-[350px] rounded-md border p-4">
                    <div>
                        { for (1..=20).map(|i| html! {
                            <div class="py-2 border-b last:border-0">{ format!("Item {}", i) }</div>
                        })}
                    </div>
                </ScrollArea>
            },
            code: r#"<ScrollArea class="h-[200px] w-[350px]">
    { items.iter().map(|item| html! { <div>{ item }</div> }) }
</ScrollArea>"#,
        },
        Example {
            title: "Horizontal",
            description: "Horizontal scrolling area.",
            demo: html! {
                <ScrollArea class="w-[350px] whitespace-nowrap rounded-md border">
                    <div class="flex w-max space-x-4 p-4">
                        { for (1..=10).map(|i| html! {
                            <div class="w-[150px] h-[100px] rounded-md bg-muted flex items-center justify-center shrink-0">
                                { format!("Item {}", i) }
                            </div>
                        })}
                    </div>
                </ScrollArea>
            },
            code: r#"<ScrollArea class="w-[350px]">
    <div class="flex">{ /* horizontal items */ }</div>
</ScrollArea>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Scrollable content" },
    ];

    html! { <ComponentPage name="Scroll Area" description="Augments native scroll functionality with custom scrollbars." {examples} {props} /> }
}
