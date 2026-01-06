//! NavigationMenu component showcase page

use yew::prelude::*;
use shadcn_rs::{NavigationMenu, NavigationMenuList, NavigationMenuItem, NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(NavigationMenuPage)]
pub fn navigation_menu_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A navigation menu with dropdown content.",
            demo: html! {
                <NavigationMenu>
                    <NavigationMenuList>
                        <NavigationMenuItem>
                            <NavigationMenuTrigger>{ "Getting Started" }</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <ul class="grid gap-3 p-4 w-[400px]">
                                    <li>
                                        <NavigationMenuLink href="/getting-started/installation">
                                            <div class="font-medium">{ "Installation" }</div>
                                            <p class="text-sm text-muted-foreground">
                                                { "How to install shadcn-rs in your project." }
                                            </p>
                                        </NavigationMenuLink>
                                    </li>
                                    <li>
                                        <NavigationMenuLink href="/getting-started/quick-start">
                                            <div class="font-medium">{ "Quick Start" }</div>
                                            <p class="text-sm text-muted-foreground">
                                                { "Get up and running in minutes." }
                                            </p>
                                        </NavigationMenuLink>
                                    </li>
                                </ul>
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuTrigger>{ "Components" }</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <ul class="grid gap-3 p-4 w-[400px]">
                                    <li>
                                        <NavigationMenuLink href="/components/button">
                                            <div class="font-medium">{ "Button" }</div>
                                            <p class="text-sm text-muted-foreground">
                                                { "Displays a button or component that looks like a button." }
                                            </p>
                                        </NavigationMenuLink>
                                    </li>
                                </ul>
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            },
            code: r#"<NavigationMenu>
    <NavigationMenuList>
        <NavigationMenuItem>
            <NavigationMenuTrigger>{ "Menu" }</NavigationMenuTrigger>
            <NavigationMenuContent>
                { /* Content */ }
            </NavigationMenuContent>
        </NavigationMenuItem>
    </NavigationMenuList>
</NavigationMenu>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Menu content" },
    ];

    html! { <ComponentPage name="Navigation Menu" description="A collection of links for navigating websites." {examples} {props} /> }
}
