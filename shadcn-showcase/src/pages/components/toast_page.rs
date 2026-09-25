//! Toast component showcase page

use gloo::timers::future::TimeoutFuture;
use shadcn_rs::{Button, PromiseMessages, ToastHandle, ToastOptions, Variant, use_toast};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// A click handler that shows the toast built by `make`.
fn show(toast: &ToastHandle, make: impl Fn() -> ToastOptions + 'static) -> Callback<MouseEvent> {
    let toast = toast.clone();
    Callback::from(move |_: MouseEvent| {
        toast.add(make());
    })
}

#[function_component(ToastPage)]
pub fn toast_page() -> Html {
    let toast = use_toast();

    let show_default = show(&toast, || {
        ToastOptions::new("Event has been created")
            .description("Friday, February 10, 2023 at 5:57 PM")
    });
    let show_success = show(&toast, || {
        ToastOptions::success("Changes saved successfully")
    });
    let show_info = show(&toast, || ToastOptions::info("A new version is available"));
    let show_warning = show(&toast, || ToastOptions::warning("Please review your input"));
    let show_error = show(&toast, || {
        ToastOptions::error("Something went wrong").description("The server returned 500.")
    });
    let show_loading = show(&toast, || {
        ToastOptions::loading("Uploading...").duration(3000)
    });

    let show_action = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            let undo_toast = toast.clone();
            toast.add(ToastOptions::new("Message archived").action(
                "Undo",
                Callback::from(move |()| {
                    undo_toast.info("Message restored");
                }),
            ));
        })
    };

    let show_promise = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            toast.promise(
                async {
                    TimeoutFuture::new(2_000).await;
                    Ok::<&str, String>("Sonner")
                },
                PromiseMessages::new("Loading...", "Data loaded", "Failed to load").success_with(
                    |name: &&str| ToastOptions::success(format!("{name} has been added")),
                ),
            );
        })
    };

    let show_update = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            let id = toast.loading("Connecting...");
            let toast = toast.clone();
            wasm_bindgen_futures::spawn_local(async move {
                TimeoutFuture::new(1_500).await;
                toast.update(
                    id,
                    ToastOptions::success("Connected").description("Latency 42 ms"),
                );
            });
        })
    };

    let examples = vec![
        Example {
            title: "Default",
            description: "Mount one <Toaster /> near the root, then call use_toast() anywhere below it.",
            demo: html! {
                <Button variant={Variant::Outline} onclick={show_default}>
                    { "Show Toast" }
                </Button>
            },
            code: r#"use shadcn_rs::{use_toast, Button, ToastOptions, Toaster, Variant};

// Once, near the root of the app:
html! { <Toaster><App /></Toaster> }

// In any component below it:
let toast = use_toast();
let onclick = Callback::from(move |_: MouseEvent| {
    toast.add(
        ToastOptions::new("Event has been created")
            .description("Friday, February 10, 2023 at 5:57 PM"),
    );
});
html! { <Button variant={Variant::Outline} {onclick}>{ "Show Toast" }</Button> }"#,
        },
        Example {
            title: "Types",
            description: "Success, info, warning, error and loading toasts each get an icon and color.",
            demo: html! {
                <div class="flex flex-wrap gap-2">
                    <Button variant={Variant::Outline} onclick={show_success}>{ "Success" }</Button>
                    <Button variant={Variant::Outline} onclick={show_info}>{ "Info" }</Button>
                    <Button variant={Variant::Outline} onclick={show_warning}>{ "Warning" }</Button>
                    <Button variant={Variant::Outline} onclick={show_error}>{ "Error" }</Button>
                    <Button variant={Variant::Outline} onclick={show_loading}>{ "Loading" }</Button>
                </div>
            },
            code: r#"toast.add(ToastOptions::success("Changes saved successfully"));
toast.add(ToastOptions::info("A new version is available"));
toast.add(ToastOptions::warning("Please review your input"));
toast.add(
    ToastOptions::error("Something went wrong").description("The server returned 500."),
);
// Loading toasts are sticky unless given a duration.
toast.add(ToastOptions::loading("Uploading...").duration(3000));

// Shortcuts for a title-only toast:
toast.success("Saved");
toast.error("Failed");"#,
        },
        Example {
            title: "With Action",
            description: "An action button runs its callback and closes the toast.",
            demo: html! {
                <Button variant={Variant::Outline} onclick={show_action}>
                    { "Show with Action" }
                </Button>
            },
            code: r#"let undo_toast = toast.clone();
toast.add(ToastOptions::new("Message archived").action(
    "Undo",
    Callback::from(move |()| {
        undo_toast.info("Message restored");
    }),
));"#,
        },
        Example {
            title: "Promise",
            description: "Shows a loading toast, then the same toast turns into success or error.",
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
        Ok::<&str, String>("Sonner")
    },
    PromiseMessages::new("Loading...", "Data loaded", "Failed to load").success_with(
        |name: &&str| ToastOptions::success(format!("{name} has been added")),
    ),
);"#,
        },
        Example {
            title: "Update",
            description: "add() returns a ToastId; update() replaces that toast's content.",
            demo: html! {
                <Button variant={Variant::Outline} onclick={show_update}>
                    { "Loading, then Update" }
                </Button>
            },
            code: r#"let id = toast.loading("Connecting...");
let toast = toast.clone();
wasm_bindgen_futures::spawn_local(async move {
    TimeoutFuture::new(1_500).await;
    toast.update(id, ToastOptions::success("Connected").description("Latency 42 ms"));
});"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "position",
            prop_type: "ToastPosition",
            default: "BottomRight",
            description: "Toaster: corner or edge the toasts stack from",
        },
        PropDoc {
            name: "limit",
            prop_type: "usize",
            default: "3",
            description: "Toaster: most toasts shown at once; older ones wait",
        },
        PropDoc {
            name: "duration",
            prop_type: "u32",
            default: "4000",
            description: "Toaster: default auto-dismiss time (ms)",
        },
        PropDoc {
            name: "rich_colors",
            prop_type: "bool",
            default: "false",
            description: "Toaster: tint the whole toast with its type color",
        },
        PropDoc {
            name: "close_button",
            prop_type: "bool",
            default: "true",
            description: "Toaster: show a close button on every toast",
        },
        PropDoc {
            name: "ToastOptions.title / description",
            prop_type: "Option<AttrValue>",
            default: "None",
            description: "Toast text",
        },
        PropDoc {
            name: "ToastOptions.type",
            prop_type: "ToastType",
            default: "Default",
            description: "Default, Success, Info, Warning, Error or Loading",
        },
        PropDoc {
            name: "ToastOptions.duration",
            prop_type: "ToastDuration",
            default: "Auto",
            description: "Auto (toaster default; sticky for Loading), Sticky, or Millis(ms)",
        },
        PropDoc {
            name: "ToastOptions.action",
            prop_type: "Option<ToastAction>",
            default: "None",
            description: "Action button { label, on_click: Callback<()> }",
        },
        PropDoc {
            name: "ToastOptions.content",
            prop_type: "Option<Html>",
            default: "None",
            description: "Custom body replacing icon, title and description",
        },
    ];

    html! { <ComponentPage name="Toast" description="A succinct message that is displayed temporarily." {examples} {props} /> }
}
