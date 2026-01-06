//! Kbd component showcase page

use yew::prelude::*;
use shadcn_rs::Kbd;

use crate::components::{ComponentPage, Example, PropDoc};

/// Kbd showcase page
#[function_component(KbdPage)]
pub fn kbd_page() -> Html {
    let examples = vec![
        Example {
            title: "Single Key",
            description: "Display a single keyboard key.",
            demo: html! {
                <Kbd>{ "K" }</Kbd>
            },
            code: r#"<Kbd>{ "K" }</Kbd>"#,
        },
        Example {
            title: "Key Combination",
            description: "Display a keyboard shortcut combination.",
            demo: html! {
                <div class="flex items-center gap-1">
                    <Kbd>{ "Ctrl" }</Kbd>
                    <span>{ "+" }</span>
                    <Kbd>{ "S" }</Kbd>
                </div>
            },
            code: r#"<div class="flex items-center gap-1">
    <Kbd>{ "Ctrl" }</Kbd>
    <span>{ "+" }</span>
    <Kbd>{ "S" }</Kbd>
</div>"#,
        },
        Example {
            title: "Common Shortcuts",
            description: "Examples of common keyboard shortcuts.",
            demo: html! {
                <div class="flex flex-col gap-2">
                    <div class="flex items-center justify-between p-2 rounded bg-muted">
                        <span>{ "Copy" }</span>
                        <div class="flex items-center gap-1">
                            <Kbd>{ "Cmd" }</Kbd>
                            <span>{ "+" }</span>
                            <Kbd>{ "C" }</Kbd>
                        </div>
                    </div>
                    <div class="flex items-center justify-between p-2 rounded bg-muted">
                        <span>{ "Paste" }</span>
                        <div class="flex items-center gap-1">
                            <Kbd>{ "Cmd" }</Kbd>
                            <span>{ "+" }</span>
                            <Kbd>{ "V" }</Kbd>
                        </div>
                    </div>
                    <div class="flex items-center justify-between p-2 rounded bg-muted">
                        <span>{ "Undo" }</span>
                        <div class="flex items-center gap-1">
                            <Kbd>{ "Cmd" }</Kbd>
                            <span>{ "+" }</span>
                            <Kbd>{ "Z" }</Kbd>
                        </div>
                    </div>
                </div>
            },
            code: r#"<div class="flex items-center gap-1">
    <Kbd>{ "Cmd" }</Kbd>
    <span>{ "+" }</span>
    <Kbd>{ "C" }</Kbd>
</div>"#,
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
            description: "Key text content",
        },
    ];

    html! {
        <ComponentPage
            name="Kbd"
            description="Displays a keyboard key or shortcut."
            {examples}
            {props}
        />
    }
}
