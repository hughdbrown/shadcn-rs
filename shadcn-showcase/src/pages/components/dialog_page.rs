//! Dialog component showcase page

use yew::prelude::*;
use shadcn_rs::{Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter, Button, Variant, Input, Label};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(DialogPage)]
pub fn dialog_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic dialog.",
            demo: html! {
                <Dialog>
                    <DialogTrigger>
                        <Button variant={Variant::Outline}>{ "Open Dialog" }</Button>
                    </DialogTrigger>
                    <DialogContent>
                        <DialogHeader>
                            <DialogTitle>{ "Edit Profile" }</DialogTitle>
                            <DialogDescription>
                                { "Make changes to your profile here. Click save when you're done." }
                            </DialogDescription>
                        </DialogHeader>
                        <div class="grid gap-4 py-4">
                            <div class="grid grid-cols-4 items-center gap-4">
                                <Label html_for="name" class="text-right">{ "Name" }</Label>
                                <Input id="name" value="Pedro Duarte" class="col-span-3" />
                            </div>
                            <div class="grid grid-cols-4 items-center gap-4">
                                <Label html_for="username" class="text-right">{ "Username" }</Label>
                                <Input id="username" value="@peduarte" class="col-span-3" />
                            </div>
                        </div>
                        <DialogFooter>
                            <Button variant={Variant::Primary}>{ "Save changes" }</Button>
                        </DialogFooter>
                    </DialogContent>
                </Dialog>
            },
            code: r#"<Dialog>
    <DialogTrigger>
        <Button>{ "Open Dialog" }</Button>
    </DialogTrigger>
    <DialogContent>
        <DialogHeader>
            <DialogTitle>{ "Edit Profile" }</DialogTitle>
            <DialogDescription>
                { "Make changes to your profile." }
            </DialogDescription>
        </DialogHeader>
        { /* Content */ }
        <DialogFooter>
            <Button>{ "Save" }</Button>
        </DialogFooter>
    </DialogContent>
</Dialog>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "open", prop_type: "bool", default: "false", description: "Controlled open state" },
        PropDoc { name: "default_open", prop_type: "bool", default: "false", description: "Default open state" },
        PropDoc { name: "on_open_change", prop_type: "Callback<bool>", default: "-", description: "Open state change handler" },
    ];

    html! { <ComponentPage name="Dialog" description="A modal dialog that interrupts the user with important content." {examples} {props} /> }
}
