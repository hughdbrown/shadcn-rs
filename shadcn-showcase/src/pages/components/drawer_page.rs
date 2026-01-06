//! Drawer component showcase page

use yew::prelude::*;
use shadcn_rs::{Drawer, DrawerTrigger, DrawerContent, DrawerHeader, DrawerTitle, DrawerDescription, DrawerFooter, Button, Variant};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(DrawerPage)]
pub fn drawer_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A mobile-friendly bottom drawer.",
            demo: html! {
                <Drawer>
                    <DrawerTrigger>
                        <Button variant={Variant::Outline}>{ "Open Drawer" }</Button>
                    </DrawerTrigger>
                    <DrawerContent>
                        <DrawerHeader>
                            <DrawerTitle>{ "Move Goal" }</DrawerTitle>
                            <DrawerDescription>{ "Set your daily activity goal." }</DrawerDescription>
                        </DrawerHeader>
                        <div class="p-4">
                            <p>{ "Drawer content goes here..." }</p>
                        </div>
                        <DrawerFooter>
                            <Button>{ "Submit" }</Button>
                            <Button variant={Variant::Outline}>{ "Cancel" }</Button>
                        </DrawerFooter>
                    </DrawerContent>
                </Drawer>
            },
            code: r#"<Drawer>
    <DrawerTrigger>
        <Button>{ "Open Drawer" }</Button>
    </DrawerTrigger>
    <DrawerContent>
        <DrawerHeader>
            <DrawerTitle>{ "Title" }</DrawerTitle>
            <DrawerDescription>{ "Description" }</DrawerDescription>
        </DrawerHeader>
        { /* Content */ }
        <DrawerFooter>
            <Button>{ "Submit" }</Button>
        </DrawerFooter>
    </DrawerContent>
</Drawer>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "open", prop_type: "bool", default: "false", description: "Controlled open state" },
        PropDoc { name: "on_open_change", prop_type: "Callback<bool>", default: "-", description: "Open state change handler" },
    ];

    html! { <ComponentPage name="Drawer" description="A mobile-friendly drawer component with swipe gestures." {examples} {props} /> }
}
