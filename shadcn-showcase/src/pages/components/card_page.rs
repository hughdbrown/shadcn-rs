//! Card component showcase page

use yew::prelude::*;
use shadcn_rs::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter, Button, Variant};

use crate::components::{ComponentPage, Example, PropDoc};

/// Card showcase page
#[function_component(CardPage)]
pub fn card_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A simple card with header, content, and footer.",
            demo: html! {
                <Card class="w-[350px]">
                    <CardHeader>
                        <CardTitle>{ "Card Title" }</CardTitle>
                        <CardDescription>{ "Card description goes here." }</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <p>{ "Card content goes here." }</p>
                    </CardContent>
                    <CardFooter>
                        <Button>{ "Action" }</Button>
                    </CardFooter>
                </Card>
            },
            code: r#"<Card>
    <CardHeader>
        <CardTitle>{ "Card Title" }</CardTitle>
        <CardDescription>{ "Card description goes here." }</CardDescription>
    </CardHeader>
    <CardContent>
        <p>{ "Card content goes here." }</p>
    </CardContent>
    <CardFooter>
        <Button>{ "Action" }</Button>
    </CardFooter>
</Card>"#,
        },
        Example {
            title: "Notification Card",
            description: "A card used for notifications.",
            demo: html! {
                <Card class="w-[380px]">
                    <CardHeader>
                        <CardTitle>{ "Notifications" }</CardTitle>
                        <CardDescription>{ "You have 3 unread messages." }</CardDescription>
                    </CardHeader>
                    <CardContent class="grid gap-4">
                        <div class="flex items-center gap-4 p-2 rounded-md bg-muted">
                            <span class="flex h-2 w-2 rounded-full bg-primary"></span>
                            <div class="grid gap-1">
                                <p class="text-sm font-medium">{ "Your call has been confirmed." }</p>
                                <p class="text-xs text-muted-foreground">{ "1 hour ago" }</p>
                            </div>
                        </div>
                        <div class="flex items-center gap-4 p-2 rounded-md bg-muted">
                            <span class="flex h-2 w-2 rounded-full bg-primary"></span>
                            <div class="grid gap-1">
                                <p class="text-sm font-medium">{ "You have a new message!" }</p>
                                <p class="text-xs text-muted-foreground">{ "2 hours ago" }</p>
                            </div>
                        </div>
                    </CardContent>
                    <CardFooter>
                        <Button variant={Variant::Outline} class="w-full">
                            { "Mark all as read" }
                        </Button>
                    </CardFooter>
                </Card>
            },
            code: r#"<Card>
    <CardHeader>
        <CardTitle>{ "Notifications" }</CardTitle>
        <CardDescription>{ "You have 3 unread messages." }</CardDescription>
    </CardHeader>
    <CardContent>
        // Notification items...
    </CardContent>
    <CardFooter>
        <Button variant={Variant::Outline}>
            { "Mark all as read" }
        </Button>
    </CardFooter>
</Card>"#,
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
            description: "Card content (CardHeader, CardContent, CardFooter)",
        },
    ];

    html! {
        <ComponentPage
            name="Card"
            description="Displays a card with header, content, and footer."
            {examples}
            {props}
        />
    }
}
