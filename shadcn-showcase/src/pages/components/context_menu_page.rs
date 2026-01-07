//! ContextMenu component showcase page

use shadcn_rs::{
    ContextMenu, ContextMenuContent, ContextMenuItem, ContextMenuSeparator, ContextMenuTrigger,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ContextMenuPage)]
pub fn context_menu_page() -> Html {
    let examples = vec![Example {
        title: "Default",
        description: "Right-click to open the context menu.",
        demo: html! {
            <ContextMenu>
                <ContextMenuTrigger>
                    <div class="flex h-[150px] w-[300px] items-center justify-center rounded-md border border-dashed text-sm">
                        { "Right click here" }
                    </div>
                </ContextMenuTrigger>
                <ContextMenuContent>
                    <ContextMenuItem>{ "Back" }</ContextMenuItem>
                    <ContextMenuItem disabled={true}>{ "Forward" }</ContextMenuItem>
                    <ContextMenuItem>{ "Reload" }</ContextMenuItem>
                    <ContextMenuSeparator />
                    <ContextMenuItem>{ "Save Page As..." }</ContextMenuItem>
                    <ContextMenuItem>{ "Print..." }</ContextMenuItem>
                </ContextMenuContent>
            </ContextMenu>
        },
        code: r#"<ContextMenu>
    <ContextMenuTrigger>
        <div>{ "Right click here" }</div>
    </ContextMenuTrigger>
    <ContextMenuContent>
        <ContextMenuItem>{ "Back" }</ContextMenuItem>
        <ContextMenuItem>{ "Forward" }</ContextMenuItem>
        <ContextMenuSeparator />
        <ContextMenuItem>{ "Save" }</ContextMenuItem>
    </ContextMenuContent>
</ContextMenu>"#,
    }];

    let props = vec![
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
            description: "Menu content",
        },
    ];

    html! { <ComponentPage name="Context Menu" description="Displays a menu when right-clicking." {examples} {props} /> }
}
