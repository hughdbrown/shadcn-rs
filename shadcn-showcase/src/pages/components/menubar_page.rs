//! Menubar component showcase page

use yew::prelude::*;
use shadcn_rs::{Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem, MenubarSeparator};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(MenubarPage)]
pub fn menubar_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A horizontal menu bar.",
            demo: html! {
                <Menubar>
                    <MenubarMenu>
                        <MenubarTrigger>{ "File" }</MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem>{ "New Tab" }</MenubarItem>
                            <MenubarItem>{ "New Window" }</MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem>{ "Share" }</MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem>{ "Print" }</MenubarItem>
                        </MenubarContent>
                    </MenubarMenu>
                    <MenubarMenu>
                        <MenubarTrigger>{ "Edit" }</MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem>{ "Undo" }</MenubarItem>
                            <MenubarItem>{ "Redo" }</MenubarItem>
                            <MenubarSeparator />
                            <MenubarItem>{ "Cut" }</MenubarItem>
                            <MenubarItem>{ "Copy" }</MenubarItem>
                            <MenubarItem>{ "Paste" }</MenubarItem>
                        </MenubarContent>
                    </MenubarMenu>
                    <MenubarMenu>
                        <MenubarTrigger>{ "View" }</MenubarTrigger>
                        <MenubarContent>
                            <MenubarItem>{ "Zoom In" }</MenubarItem>
                            <MenubarItem>{ "Zoom Out" }</MenubarItem>
                        </MenubarContent>
                    </MenubarMenu>
                </Menubar>
            },
            code: r#"<Menubar>
    <MenubarMenu>
        <MenubarTrigger>{ "File" }</MenubarTrigger>
        <MenubarContent>
            <MenubarItem>{ "New" }</MenubarItem>
            <MenubarItem>{ "Open" }</MenubarItem>
        </MenubarContent>
    </MenubarMenu>
</Menubar>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Menu items" },
    ];

    html! { <ComponentPage name="Menubar" description="A visually persistent menu for quick access." {examples} {props} /> }
}
