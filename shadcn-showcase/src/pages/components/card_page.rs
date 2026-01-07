//! Card component showcase page

use shadcn_rs::{
    Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input, Label,
    Variant,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Card showcase page
#[function_component(CardPage)]
pub fn card_page() -> Html {
    let examples = vec![
        Example {
            title: "Login Form",
            description: "A card with a login form, similar to shadcn/ui example.",
            demo: html! {
                <Card class="w-[350px]">
                    <CardHeader>
                        <CardTitle>{ "Login" }</CardTitle>
                        <CardDescription>{ "Enter your email below to login to your account." }</CardDescription>
                    </CardHeader>
                    <CardContent>
                        <div class="grid gap-4">
                            <div class="grid gap-2">
                                <Label html_for="email">{ "Email" }</Label>
                                <Input id="email" r#type="email" placeholder="m@example.com" />
                            </div>
                            <div class="grid gap-2">
                                <Label html_for="password">{ "Password" }</Label>
                                <Input id="password" r#type="password" />
                            </div>
                        </div>
                    </CardContent>
                    <CardFooter class="flex-col gap-2">
                        <Button class="w-full">{ "Sign in" }</Button>
                        <Button variant={Variant::Outline} class="w-full">{ "Sign in with Google" }</Button>
                    </CardFooter>
                </Card>
            },
            code: r#"<Card>
    <CardHeader>
        <CardTitle>{ "Login" }</CardTitle>
        <CardDescription>{ "Enter your email below to login." }</CardDescription>
    </CardHeader>
    <CardContent>
        <div class="grid gap-4">
            <div class="grid gap-2">
                <Label html_for="email">{ "Email" }</Label>
                <Input id="email" r#type="email" placeholder="m@example.com" />
            </div>
            <div class="grid gap-2">
                <Label html_for="password">{ "Password" }</Label>
                <Input id="password" r#type="password" />
            </div>
        </div>
    </CardContent>
    <CardFooter class="flex-col gap-2">
        <Button class="w-full">{ "Sign in" }</Button>
        <Button variant={Variant::Outline} class="w-full">
            { "Sign in with Google" }
        </Button>
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
