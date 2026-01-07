//! Spinner component showcase page

use shadcn_rs::{Size, Spinner};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Spinner showcase page
#[function_component(SpinnerPage)]
pub fn spinner_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic loading spinner.",
            demo: html! {
                <Spinner />
            },
            code: r#"<Spinner />"#,
        },
        Example {
            title: "Sizes",
            description: "Spinners in different sizes.",
            demo: html! {
                <div class="flex items-center gap-4">
                    <Spinner size={Size::Sm} />
                    <Spinner size={Size::Md} />
                    <Spinner size={Size::Lg} />
                </div>
            },
            code: r#"<Spinner size={Size::Sm} />
<Spinner size={Size::Md} />
<Spinner size={Size::Lg} />"#,
        },
        Example {
            title: "With Label",
            description: "Spinner with a loading label.",
            demo: html! {
                <div class="flex items-center gap-2">
                    <Spinner size={Size::Sm} />
                    <span>{ "Loading..." }</span>
                </div>
            },
            code: r#"<div class="flex items-center gap-2">
    <Spinner size={Size::Sm} />
    <span>{ "Loading..." }</span>
</div>"#,
        },
        Example {
            title: "In Button",
            description: "Spinner used inside a button for loading state.",
            demo: html! {
                <button class="btn btn-primary flex items-center gap-2" disabled=true>
                    <Spinner size={Size::Sm} />
                    { "Processing..." }
                </button>
            },
            code: r#"<Button loading={true}>
    { "Processing..." }
</Button>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "size",
            prop_type: "Size",
            default: "Md",
            description: "The size of the spinner",
        },
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "Additional CSS classes",
        },
        PropDoc {
            name: "label",
            prop_type: "Option<String>",
            default: "\"Loading\"",
            description: "Accessible label for screen readers",
        },
    ];

    html! {
        <ComponentPage
            name="Spinner"
            description="A loading spinner indicator."
            {examples}
            {props}
        />
    }
}
