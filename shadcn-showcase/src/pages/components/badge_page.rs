//! Badge component showcase page

use yew::prelude::*;
use shadcn_rs::{Badge, Variant};

use crate::components::{ComponentPage, Example, PropDoc};

/// Badge showcase page
#[function_component(BadgePage)]
pub fn badge_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "The default badge style.",
            demo: html! {
                <Badge>{ "Badge" }</Badge>
            },
            code: r#"<Badge>{ "Badge" }</Badge>"#,
        },
        Example {
            title: "Variants",
            description: "Different badge variants for various contexts.",
            demo: html! {
                <div class="flex gap-2 flex-wrap">
                    <Badge variant={Variant::Default}>{ "Default" }</Badge>
                    <Badge variant={Variant::Primary}>{ "Primary" }</Badge>
                    <Badge variant={Variant::Secondary}>{ "Secondary" }</Badge>
                    <Badge variant={Variant::Destructive}>{ "Destructive" }</Badge>
                    <Badge variant={Variant::Outline}>{ "Outline" }</Badge>
                </div>
            },
            code: r#"<Badge variant={Variant::Default}>{ "Default" }</Badge>
<Badge variant={Variant::Primary}>{ "Primary" }</Badge>
<Badge variant={Variant::Secondary}>{ "Secondary" }</Badge>
<Badge variant={Variant::Destructive}>{ "Destructive" }</Badge>
<Badge variant={Variant::Outline}>{ "Outline" }</Badge>"#,
        },
        Example {
            title: "With Icon",
            description: "Badge with an icon.",
            demo: html! {
                <Badge>
                    <svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                        <polyline points="20 6 9 17 4 12"/>
                    </svg>
                    { " Verified" }
                </Badge>
            },
            code: r#"<Badge>
    <CheckIcon />
    { " Verified" }
</Badge>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "variant",
            prop_type: "Variant",
            default: "Default",
            description: "The visual style variant of the badge",
        },
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
            description: "Badge content",
        },
    ];

    html! {
        <ComponentPage
            name="Badge"
            description="Displays a badge or a component that looks like a badge."
            {examples}
            {props}
        />
    }
}
