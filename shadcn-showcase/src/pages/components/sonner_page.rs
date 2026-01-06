//! Sonner component showcase page

use yew::prelude::*;
use shadcn_rs::{Button, Variant};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(SonnerPage)]
pub fn sonner_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "An opinionated toast component for Rust.",
            demo: html! {
                <Button variant={Variant::Outline}>
                    { "Show Sonner Toast" }
                </Button>
            },
            code: r#"use shadcn_rs::sonner;

sonner::toast("Event has been created");"#,
        },
        Example {
            title: "Types",
            description: "Different toast types.",
            demo: html! {
                <div class="flex flex-wrap gap-2">
                    <Button variant={Variant::Outline}>{ "Default" }</Button>
                    <Button variant={Variant::Outline}>{ "Success" }</Button>
                    <Button variant={Variant::Outline}>{ "Info" }</Button>
                    <Button variant={Variant::Outline}>{ "Warning" }</Button>
                    <Button variant={Variant::Outline}>{ "Error" }</Button>
                </div>
            },
            code: r#"sonner::toast("Default message");
sonner::success("Success message");
sonner::info("Info message");
sonner::warning("Warning message");
sonner::error("Error message");"#,
        },
        Example {
            title: "Promise",
            description: "Toast that tracks promise state.",
            demo: html! {
                <Button variant={Variant::Outline}>
                    { "Promise Toast" }
                </Button>
            },
            code: r#"sonner::promise(
    async_operation(),
    PromiseOptions {
        loading: "Loading...".to_string(),
        success: "Data loaded!".to_string(),
        error: "Failed to load".to_string(),
    },
);"#,
        },
        Example {
            title: "Rich Content",
            description: "Toast with custom content.",
            demo: html! {
                <Button variant={Variant::Outline}>
                    { "Custom Toast" }
                </Button>
            },
            code: r#"sonner::custom(html! {
    <div class="flex items-center gap-2">
        <Avatar>...</Avatar>
        <div>
            <p>{ "John Doe" }</p>
            <p>{ "Sent you a message" }</p>
        </div>
    </div>
});"#,
        },
    ];

    let props = vec![
        PropDoc { name: "position", prop_type: "&str", default: "\"bottom-right\"", description: "Toast position" },
        PropDoc { name: "expand", prop_type: "bool", default: "false", description: "Expand toasts by default" },
        PropDoc { name: "rich_colors", prop_type: "bool", default: "false", description: "Use rich color scheme" },
        PropDoc { name: "close_button", prop_type: "bool", default: "false", description: "Show close button" },
        PropDoc { name: "duration", prop_type: "u32", default: "4000", description: "Default duration (ms)" },
    ];

    html! { <ComponentPage name="Sonner" description="An opinionated toast component." {examples} {props} /> }
}
