//! Textarea component showcase page

use yew::prelude::*;
use shadcn_rs::Textarea;

use crate::components::{ComponentPage, Example, PropDoc};

/// Textarea showcase page
#[function_component(TextareaPage)]
pub fn textarea_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic textarea.",
            demo: html! {
                <Textarea placeholder="Type your message here." />
            },
            code: r#"<Textarea placeholder="Type your message here." />"#,
        },
        Example {
            title: "Disabled",
            description: "A disabled textarea.",
            demo: html! {
                <Textarea placeholder="Disabled textarea" disabled={true} />
            },
            code: r#"<Textarea placeholder="Disabled textarea" disabled={true} />"#,
        },
        Example {
            title: "With Label",
            description: "Textarea with a label.",
            demo: html! {
                <div class="grid gap-2">
                    <label html_for="message" class="text-sm font-medium">{ "Your message" }</label>
                    <Textarea id="message" placeholder="Type your message here." />
                    <p class="text-sm text-muted-foreground">
                        { "Your message will be sent to support." }
                    </p>
                </div>
            },
            code: r#"<div class="grid gap-2">
    <Label html_for="message">{ "Your message" }</Label>
    <Textarea id="message" placeholder="Type your message here." />
    <p class="text-sm text-muted-foreground">
        { "Your message will be sent to support." }
    </p>
</div>"#,
        },
        Example {
            title: "With Text",
            description: "Textarea with pre-filled content.",
            demo: html! {
                <Textarea value="This textarea has some initial content that you can edit." />
            },
            code: r#"<Textarea value="Initial content..." />"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "value",
            prop_type: "Option<String>",
            default: "-",
            description: "Controlled value",
        },
        PropDoc {
            name: "placeholder",
            prop_type: "Option<String>",
            default: "-",
            description: "Placeholder text",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the textarea is disabled",
        },
        PropDoc {
            name: "rows",
            prop_type: "u32",
            default: "3",
            description: "Number of visible text rows",
        },
        PropDoc {
            name: "oninput",
            prop_type: "Callback<InputEvent>",
            default: "-",
            description: "Input change handler",
        },
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "Additional CSS classes",
        },
    ];

    html! {
        <ComponentPage
            name="Textarea"
            description="Displays a form textarea for multi-line text input."
            {examples}
            {props}
        />
    }
}
