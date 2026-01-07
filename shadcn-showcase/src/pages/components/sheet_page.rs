//! Sheet component showcase page

use shadcn_rs::{
    Button, Input, Label, Position, Sheet, SheetContent, SheetDescription, SheetFooter,
    SheetHeader, SheetTitle, SheetTrigger, Variant,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(SheetPage)]
pub fn sheet_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A slide-out sheet panel.",
            demo: html! {
                <Sheet>
                    <SheetTrigger>
                        <Button variant={Variant::Outline}>{ "Open Sheet" }</Button>
                    </SheetTrigger>
                    <SheetContent>
                        <SheetHeader>
                            <SheetTitle>{ "Edit Profile" }</SheetTitle>
                            <SheetDescription>
                                { "Make changes to your profile here." }
                            </SheetDescription>
                        </SheetHeader>
                        <div class="grid gap-4 py-4">
                            <div class="grid grid-cols-4 items-center gap-4">
                                <Label html_for="name" class="text-right">{ "Name" }</Label>
                                <Input id="name" class="col-span-3" />
                            </div>
                        </div>
                        <SheetFooter>
                            <Button variant={Variant::Primary}>{ "Save changes" }</Button>
                        </SheetFooter>
                    </SheetContent>
                </Sheet>
            },
            code: r#"<Sheet>
    <SheetTrigger>
        <Button>{ "Open Sheet" }</Button>
    </SheetTrigger>
    <SheetContent>
        <SheetHeader>
            <SheetTitle>{ "Title" }</SheetTitle>
        </SheetHeader>
        { /* Content */ }
    </SheetContent>
</Sheet>"#,
        },
        Example {
            title: "Sides",
            description: "Sheets can slide in from any side.",
            demo: html! {
                <div class="flex gap-2">
                    <Sheet>
                        <SheetTrigger><Button variant={Variant::Outline}>{ "Left" }</Button></SheetTrigger>
                        <SheetContent side={Position::Left}><SheetHeader><SheetTitle>{ "Left Sheet" }</SheetTitle></SheetHeader></SheetContent>
                    </Sheet>
                    <Sheet>
                        <SheetTrigger><Button variant={Variant::Outline}>{ "Right" }</Button></SheetTrigger>
                        <SheetContent side={Position::Right}><SheetHeader><SheetTitle>{ "Right Sheet" }</SheetTitle></SheetHeader></SheetContent>
                    </Sheet>
                    <Sheet>
                        <SheetTrigger><Button variant={Variant::Outline}>{ "Top" }</Button></SheetTrigger>
                        <SheetContent side={Position::Top}><SheetHeader><SheetTitle>{ "Top Sheet" }</SheetTitle></SheetHeader></SheetContent>
                    </Sheet>
                    <Sheet>
                        <SheetTrigger><Button variant={Variant::Outline}>{ "Bottom" }</Button></SheetTrigger>
                        <SheetContent side={Position::Bottom}><SheetHeader><SheetTitle>{ "Bottom Sheet" }</SheetTitle></SheetHeader></SheetContent>
                    </Sheet>
                </div>
            },
            code: r#"<SheetContent side="left">...</SheetContent>
<SheetContent side="right">...</SheetContent>
<SheetContent side="top">...</SheetContent>
<SheetContent side="bottom">...</SheetContent>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "side",
            prop_type: "&str",
            default: "\"right\"",
            description: "Side to slide from",
        },
        PropDoc {
            name: "open",
            prop_type: "bool",
            default: "false",
            description: "Controlled open state",
        },
        PropDoc {
            name: "on_open_change",
            prop_type: "Callback<bool>",
            default: "-",
            description: "Open state change handler",
        },
    ];

    html! { <ComponentPage name="Sheet" description="A slide-out side panel." {examples} {props} /> }
}
