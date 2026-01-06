//! Typography component showcase page

use yew::prelude::*;
use shadcn_rs::{Typography, TypographyVariant};

use crate::components::{ComponentPage, Example, PropDoc};

/// Typography showcase page
#[function_component(TypographyPage)]
pub fn typography_page() -> Html {
    let examples = vec![
        Example {
            title: "Headings",
            description: "Typography heading variants.",
            demo: html! {
                <div class="space-y-4">
                    <Typography variant={TypographyVariant::H1}>{ "Heading 1" }</Typography>
                    <Typography variant={TypographyVariant::H2}>{ "Heading 2" }</Typography>
                    <Typography variant={TypographyVariant::H3}>{ "Heading 3" }</Typography>
                    <Typography variant={TypographyVariant::H4}>{ "Heading 4" }</Typography>
                </div>
            },
            code: r##"<Typography variant={TypographyVariant::H1}>{ "Heading 1" }</Typography>
<Typography variant={TypographyVariant::H2}>{ "Heading 2" }</Typography>
<Typography variant={TypographyVariant::H3}>{ "Heading 3" }</Typography>
<Typography variant={TypographyVariant::H4}>{ "Heading 4" }</Typography>"##,
        },
        Example {
            title: "Paragraph",
            description: "Standard paragraph text.",
            demo: html! {
                <Typography variant={TypographyVariant::P}>
                    { "The quick brown fox jumps over the lazy dog. This is a sample paragraph " }
                    { "demonstrating the default paragraph styling in shadcn-rs." }
                </Typography>
            },
            code: r##"<Typography variant={TypographyVariant::P}>
    { "Your paragraph text here..." }
</Typography>"##,
        },
        Example {
            title: "Lead",
            description: "Lead paragraph for introductions.",
            demo: html! {
                <Typography variant={TypographyVariant::Lead}>
                    { "A modal dialog that interrupts the user with important content and expects a response." }
                </Typography>
            },
            code: r##"<Typography variant={TypographyVariant::Lead}>
    { "Lead text..." }
</Typography>"##,
        },
        Example {
            title: "Muted",
            description: "Muted text for secondary content.",
            demo: html! {
                <Typography variant={TypographyVariant::Muted}>
                    { "Enter your email address to receive updates." }
                </Typography>
            },
            code: r##"<Typography variant={TypographyVariant::Muted}>
    { "Muted text..." }
</Typography>"##,
        },
        Example {
            title: "Small",
            description: "Small text for fine print.",
            demo: html! {
                <Typography variant={TypographyVariant::Small}>
                    { "Email address will not be shared with third parties." }
                </Typography>
            },
            code: r##"<Typography variant={TypographyVariant::Small}>
    { "Small text..." }
</Typography>"##,
        },
        Example {
            title: "Blockquote",
            description: "Blockquote for quoted text.",
            demo: html! {
                <Typography variant={TypographyVariant::Blockquote}>
                    { "\"After all, all programming is just manipulating data.\" - Someone" }
                </Typography>
            },
            code: r##"<Typography variant={TypographyVariant::Blockquote}>
    { "Quoted text..." }
</Typography>"##,
        },
        Example {
            title: "Code",
            description: "Inline code styling.",
            demo: html! {
                <Typography variant={TypographyVariant::P}>
                    { "Use the " }
                    <Typography variant={TypographyVariant::Code}>{ "Button" }</Typography>
                    { " component for clickable actions." }
                </Typography>
            },
            code: r##"<Typography variant={TypographyVariant::Code}>
    { "code" }
</Typography>"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "variant",
            prop_type: "TypographyVariant",
            default: "P",
            description: "The typography variant to render",
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
            description: "Text content",
        },
    ];

    html! {
        <ComponentPage
            name="Typography"
            description="Styles for headings, paragraphs, and other text content."
            {examples}
            {props}
        />
    }
}
