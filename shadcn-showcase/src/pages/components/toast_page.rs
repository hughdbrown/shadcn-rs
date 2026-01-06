//! Toast component showcase page

use yew::prelude::*;
use shadcn_rs::{Button, Variant};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ToastPage)]
pub fn toast_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "Show toast notifications.",
            demo: html! {
                <Button variant={Variant::Outline} onclick={Callback::from(|_| {
                    // toast.show("Event has been created")
                })}>
                    { "Show Toast" }
                </Button>
            },
            code: r#"use shadcn_rs::toast;

toast.show("Event has been created");

// With options
toast.show_with_options(ToastOptions {
    title: "Scheduled: Catch up".to_string(),
    description: Some("Friday, February 10, 2024".to_string()),
    variant: ToastVariant::Default,
    duration: 5000,
});"#,
        },
        Example {
            title: "Variants",
            description: "Different toast styles.",
            demo: html! {
                <div class="flex gap-2">
                    <Button variant={Variant::Outline}>{ "Default" }</Button>
                    <Button variant={Variant::Outline}>{ "Success" }</Button>
                    <Button variant={Variant::Outline}>{ "Error" }</Button>
                </div>
            },
            code: r#"toast.success("Changes saved successfully");
toast.error("Something went wrong");
toast.warning("Please review your input");"#,
        },
        Example {
            title: "With Action",
            description: "Toast with an action button.",
            demo: html! {
                <Button variant={Variant::Outline}>
                    { "Show with Action" }
                </Button>
            },
            code: r#"toast.show_with_options(ToastOptions {
    title: "Undo?".to_string(),
    action: Some(ToastAction {
        label: "Undo".to_string(),
        on_click: Callback::from(|_| { /* undo */ }),
    }),
    ..Default::default()
});"#,
        },
    ];

    let props = vec![
        PropDoc { name: "title", prop_type: "String", default: "-", description: "Toast title" },
        PropDoc { name: "description", prop_type: "Option<String>", default: "-", description: "Toast description" },
        PropDoc { name: "variant", prop_type: "ToastVariant", default: "Default", description: "Visual variant" },
        PropDoc { name: "duration", prop_type: "u32", default: "5000", description: "Auto-dismiss time (ms)" },
        PropDoc { name: "action", prop_type: "Option<ToastAction>", default: "-", description: "Action button" },
    ];

    html! { <ComponentPage name="Toast" description="A succinct message that is displayed temporarily." {examples} {props} /> }
}
