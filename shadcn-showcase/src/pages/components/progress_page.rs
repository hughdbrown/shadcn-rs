//! Progress component showcase page

use yew::prelude::*;
use shadcn_rs::Progress;

use crate::components::{ComponentPage, Example, PropDoc};

/// Progress showcase page
#[function_component(ProgressPage)]
pub fn progress_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic progress bar.",
            demo: html! {
                <Progress value={60.0} class="w-[300px]" />
            },
            code: r#"<Progress value={60.0} />"#,
        },
        Example {
            title: "Empty",
            description: "Progress at 0%.",
            demo: html! {
                <Progress value={0.0} class="w-[300px]" />
            },
            code: r#"<Progress value={0.0} />"#,
        },
        Example {
            title: "Complete",
            description: "Progress at 100%.",
            demo: html! {
                <Progress value={100.0} class="w-[300px]" />
            },
            code: r#"<Progress value={100.0} />"#,
        },
        Example {
            title: "Indeterminate",
            description: "Progress without a known value.",
            demo: html! {
                <Progress class="w-[300px]" />
            },
            code: r#"<Progress />"#,
        },
        Example {
            title: "With Label",
            description: "Progress with percentage label.",
            demo: html! {
                <div class="space-y-2 w-[300px]">
                    <div class="flex justify-between text-sm">
                        <span>{ "Uploading..." }</span>
                        <span>{ "60%" }</span>
                    </div>
                    <Progress value={60.0} />
                </div>
            },
            code: r#"<div class="space-y-2">
    <div class="flex justify-between text-sm">
        <span>{ "Uploading..." }</span>
        <span>{ format!("{}%", value) }</span>
    </div>
    <Progress value={value} />
</div>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "value",
            prop_type: "Option<f64>",
            default: "-",
            description: "Current progress value (0-100). None for indeterminate.",
        },
        PropDoc {
            name: "max",
            prop_type: "f64",
            default: "100.0",
            description: "Maximum value",
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
            name="Progress"
            description="Displays an indicator showing the completion progress of a task."
            {examples}
            {props}
        />
    }
}
