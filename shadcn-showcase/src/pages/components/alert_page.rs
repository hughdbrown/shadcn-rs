//! Alert component showcase page

use yew::prelude::*;
use shadcn_rs::{Alert, AlertTitle, AlertDescription, Variant};

use crate::components::{ComponentPage, Example, PropDoc};

/// Alert showcase page
#[function_component(AlertPage)]
pub fn alert_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "The default alert style.",
            demo: html! {
                <Alert>
                    <AlertTitle>{ "Heads up!" }</AlertTitle>
                    <AlertDescription>
                        { "You can add components to your app using the cli." }
                    </AlertDescription>
                </Alert>
            },
            code: r##"<Alert>
    <AlertTitle>{ "Heads up!" }</AlertTitle>
    <AlertDescription>
        { "You can add components to your app using the cli." }
    </AlertDescription>
</Alert>"##,
        },
        Example {
            title: "Primary",
            description: "Primary alert.",
            demo: html! {
                <Alert variant={Variant::Primary}>
                    <AlertTitle>{ "Information" }</AlertTitle>
                    <AlertDescription>
                        { "This is an informational message." }
                    </AlertDescription>
                </Alert>
            },
            code: r##"<Alert variant={Variant::Primary}>
    <AlertTitle>{ "Information" }</AlertTitle>
    <AlertDescription>
        { "This is an informational message." }
    </AlertDescription>
</Alert>"##,
        },
        Example {
            title: "Secondary",
            description: "Secondary alert.",
            demo: html! {
                <Alert variant={Variant::Secondary}>
                    <AlertTitle>{ "Note" }</AlertTitle>
                    <AlertDescription>
                        { "This is a secondary message." }
                    </AlertDescription>
                </Alert>
            },
            code: r##"<Alert variant={Variant::Secondary}>
    <AlertTitle>{ "Note" }</AlertTitle>
    <AlertDescription>
        { "This is a secondary message." }
    </AlertDescription>
</Alert>"##,
        },
        Example {
            title: "Destructive",
            description: "Destructive/error alert.",
            demo: html! {
                <Alert variant={Variant::Destructive}>
                    <AlertTitle>{ "Error" }</AlertTitle>
                    <AlertDescription>
                        { "Your session has expired. Please log in again." }
                    </AlertDescription>
                </Alert>
            },
            code: r##"<Alert variant={Variant::Destructive}>
    <AlertTitle>{ "Error" }</AlertTitle>
    <AlertDescription>
        { "Your session has expired. Please log in again." }
    </AlertDescription>
</Alert>"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "variant",
            prop_type: "Variant",
            default: "Default",
            description: "The visual style variant of the alert",
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
            description: "Alert content (typically AlertTitle and AlertDescription)",
        },
    ];

    html! {
        <ComponentPage
            name="Alert"
            description="Displays a callout for user attention."
            {examples}
            {props}
        />
    }
}
