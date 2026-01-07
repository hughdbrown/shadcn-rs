//! AlertDialog component showcase page

use shadcn_rs::{
    AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription,
    AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger, Button, Variant,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(AlertDialogPage)]
pub fn alert_dialog_page() -> Html {
    let examples = vec![Example {
        title: "Default",
        description: "A confirmation dialog.",
        demo: html! {
            <AlertDialog>
                <AlertDialogTrigger>
                    <Button variant={Variant::Outline}>{ "Delete Account" }</Button>
                </AlertDialogTrigger>
                <AlertDialogContent>
                    <AlertDialogHeader>
                        <AlertDialogTitle>{ "Are you absolutely sure?" }</AlertDialogTitle>
                        <AlertDialogDescription>
                            { "This action cannot be undone. This will permanently delete your " }
                            { "account and remove your data from our servers." }
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogCancel>{ "Cancel" }</AlertDialogCancel>
                        <AlertDialogAction>{ "Continue" }</AlertDialogAction>
                    </AlertDialogFooter>
                </AlertDialogContent>
            </AlertDialog>
        },
        code: r#"<AlertDialog>
    <AlertDialogTrigger>
        <Button>{ "Delete Account" }</Button>
    </AlertDialogTrigger>
    <AlertDialogContent>
        <AlertDialogHeader>
            <AlertDialogTitle>{ "Are you sure?" }</AlertDialogTitle>
            <AlertDialogDescription>
                { "This action cannot be undone." }
            </AlertDialogDescription>
        </AlertDialogHeader>
        <AlertDialogFooter>
            <AlertDialogCancel>{ "Cancel" }</AlertDialogCancel>
            <AlertDialogAction>{ "Continue" }</AlertDialogAction>
        </AlertDialogFooter>
    </AlertDialogContent>
</AlertDialog>"#,
    }];

    let props = vec![
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

    html! { <ComponentPage name="Alert Dialog" description="A modal dialog that requires user acknowledgement." {examples} {props} /> }
}
