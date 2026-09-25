//! Sonner component showcase page
//!
//! Sonner is deprecated in shadcn-rs: upstream folded it into Toast. The
//! demos here use the same `use_toast()` API as the Toast page.

use gloo::timers::future::TimeoutFuture;
use shadcn_rs::{Button, PromiseMessages, ToastOptions, Variant, use_toast};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(SonnerPage)]
pub fn sonner_page() -> Html {
    let toast = use_toast();

    let show_message = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            toast.message("Event has been created");
        })
    };
    let show_type = |kind: &'static str| {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            match kind {
                "success" => toast.success("Success message"),
                "info" => toast.info("Info message"),
                "warning" => toast.warning("Warning message"),
                "error" => toast.error("Error message"),
                _ => toast.message("Default message"),
            };
        })
    };
    let show_promise = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            toast.promise(
                async {
                    TimeoutFuture::new(2_000).await;
                    Ok::<(), String>(())
                },
                PromiseMessages::new("Loading...", "Data loaded!", "Failed to load"),
            );
        })
    };
    let show_custom = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            toast.add(ToastOptions::custom(html! {
                <div class="flex items-center gap-2">
                    <span class="avatar size-sm shape-circle">
                        <span class="avatar-fallback">{ "JD" }</span>
                    </span>
                    <div>
                        <p class="toast-title">{ "John Doe" }</p>
                        <p class="toast-description">{ "Sent you a message" }</p>
                    </div>
                </div>
            }));
        })
    };

    let examples = vec![
        Example {
            title: "Default",
            description: "Sonner is a deprecated alias of Toast: mount <Toaster /> and call use_toast().",
            demo: html! {
                <Button variant={Variant::Outline} onclick={show_message}>
                    { "Show Toast" }
                </Button>
            },
            code: r#"use shadcn_rs::use_toast;

let toast = use_toast();
toast.message("Event has been created");"#,
        },
        Example {
            title: "Types",
            description: "Different toast types.",
            demo: html! {
                <div class="flex flex-wrap gap-2">
                    <Button variant={Variant::Outline} onclick={show_type("default")}>{ "Default" }</Button>
                    <Button variant={Variant::Outline} onclick={show_type("success")}>{ "Success" }</Button>
                    <Button variant={Variant::Outline} onclick={show_type("info")}>{ "Info" }</Button>
                    <Button variant={Variant::Outline} onclick={show_type("warning")}>{ "Warning" }</Button>
                    <Button variant={Variant::Outline} onclick={show_type("error")}>{ "Error" }</Button>
                </div>
            },
            code: r#"toast.message("Default message");
toast.success("Success message");
toast.info("Info message");
toast.warning("Warning message");
toast.error("Error message");"#,
        },
        Example {
            title: "Promise",
            description: "Toast that tracks a future: loading, then success or error.",
            demo: html! {
                <Button variant={Variant::Outline} onclick={show_promise}>
                    { "Promise Toast" }
                </Button>
            },
            code: r#"use gloo::timers::future::TimeoutFuture;
use shadcn_rs::PromiseMessages;

toast.promise(
    async {
        TimeoutFuture::new(2_000).await;
        Ok::<(), String>(())
    },
    PromiseMessages::new("Loading...", "Data loaded!", "Failed to load"),
);"#,
        },
        Example {
            title: "Rich Content",
            description: "Toast with custom content.",
            demo: html! {
                <Button variant={Variant::Outline} onclick={show_custom}>
                    { "Custom Toast" }
                </Button>
            },
            code: r#"toast.add(ToastOptions::custom(html! {
    <div class="flex items-center gap-2">
        <span class="avatar size-sm shape-circle">
            <span class="avatar-fallback">{ "JD" }</span>
        </span>
        <div>
            <p class="toast-title">{ "John Doe" }</p>
            <p class="toast-description">{ "Sent you a message" }</p>
        </div>
    </div>
}));"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "Sonner",
            prop_type: "deprecated",
            default: "-",
            description: "Use Toaster (same position enum: SonnerPosition = ToastPosition)",
        },
        PropDoc {
            name: "SonnerToast",
            prop_type: "deprecated",
            default: "-",
            description: "Use use_toast().add(ToastOptions) (SonnerType = ToastType)",
        },
        PropDoc {
            name: "position",
            prop_type: "ToastPosition",
            default: "BottomRight",
            description: "Toaster position",
        },
        PropDoc {
            name: "rich_colors",
            prop_type: "bool",
            default: "false",
            description: "Use rich color scheme",
        },
        PropDoc {
            name: "close_button",
            prop_type: "bool",
            default: "true",
            description: "Show close button",
        },
        PropDoc {
            name: "duration",
            prop_type: "u32",
            default: "4000",
            description: "Default duration (ms)",
        },
    ];

    html! { <ComponentPage name="Sonner" description="Sonner (deprecated alias of Toast): an opinionated toast component." import_code="use shadcn_rs::{use_toast, Toaster};" {examples} {props} /> }
}
