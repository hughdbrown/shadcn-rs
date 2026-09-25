//! Menubar component showcase page

use shadcn_rs::{
    Menubar, MenubarCheckboxItem, MenubarContent, MenubarItem, MenubarMenu, MenubarRadioGroup,
    MenubarRadioItem, MenubarSeparator, MenubarTrigger,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(MenubarStateDemo)]
fn menubar_state_demo() -> Html {
    let show_bookmarks = use_state(|| true);
    let profile = use_state(|| AttrValue::from("benoit"));

    let on_bookmarks = {
        let show_bookmarks = show_bookmarks.clone();
        Callback::from(move |checked: bool| show_bookmarks.set(checked))
    };
    let on_profile = {
        let profile = profile.clone();
        Callback::from(move |value: AttrValue| profile.set(value))
    };

    html! {
        <div class="space-y-2">
            <Menubar>
                <MenubarMenu>
                    <MenubarTrigger>{ "View" }</MenubarTrigger>
                    <MenubarContent>
                        <MenubarCheckboxItem checked={*show_bookmarks} onchange={on_bookmarks}>
                            { "Always Show Bookmarks Bar" }
                        </MenubarCheckboxItem>
                        <MenubarSeparator />
                        <MenubarItem>{ "Reload" }</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu>
                    <MenubarTrigger>{ "Profiles" }</MenubarTrigger>
                    <MenubarContent>
                        <MenubarRadioGroup value={(*profile).clone()} onchange={on_profile}>
                            <MenubarRadioItem value="andy">{ "Andy" }</MenubarRadioItem>
                            <MenubarRadioItem value="benoit">{ "Benoit" }</MenubarRadioItem>
                            <MenubarRadioItem value="luis">{ "Luis" }</MenubarRadioItem>
                        </MenubarRadioGroup>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
            <p class="text-sm text-muted-foreground">
                { format!("Bookmarks bar: {}, profile: {}", if *show_bookmarks { "shown" } else { "hidden" }, *profile) }
            </p>
        </div>
    }
}

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
        Example {
            title: "Checkbox and radio items",
            description: "Checkbox items toggle and radio items follow the group's value; both keep the menu open.",
            demo: html! { <MenubarStateDemo /> },
            code: r#"<MenubarCheckboxItem checked={*show_bookmarks} onchange={on_bookmarks}>
    { "Always Show Bookmarks Bar" }
</MenubarCheckboxItem>
<MenubarRadioGroup value={(*profile).clone()} onchange={on_profile}>
    <MenubarRadioItem value="andy">{ "Andy" }</MenubarRadioItem>
    <MenubarRadioItem value="benoit">{ "Benoit" }</MenubarRadioItem>
</MenubarRadioGroup>"#,
        },
    ];

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
            description: "Menu items",
        },
    ];

    html! { <ComponentPage name="Menubar" description="A visually persistent menu for quick access." {examples} {props} /> }
}
