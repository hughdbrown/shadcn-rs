//! DropdownMenu component showcase page

use yew::prelude::*;
use shadcn_rs::{DropdownMenu, DropdownMenuTrigger, DropdownMenuContent, DropdownMenuItem, DropdownMenuSeparator, DropdownMenuLabel, Button, Variant};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(DropdownMenuPage)]
pub fn dropdown_menu_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic dropdown menu.",
            demo: html! {
                <DropdownMenu>
                    <DropdownMenuTrigger>
                        <Button variant={Variant::Outline}>{ "Open Menu" }</Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent>
                        <DropdownMenuLabel>{ "My Account" }</DropdownMenuLabel>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem>{ "Profile" }</DropdownMenuItem>
                        <DropdownMenuItem>{ "Billing" }</DropdownMenuItem>
                        <DropdownMenuItem>{ "Team" }</DropdownMenuItem>
                        <DropdownMenuItem>{ "Subscription" }</DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem>{ "Log out" }</DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            },
            code: r#"<DropdownMenu>
    <DropdownMenuTrigger>
        <Button>{ "Open Menu" }</Button>
    </DropdownMenuTrigger>
    <DropdownMenuContent>
        <DropdownMenuLabel>{ "My Account" }</DropdownMenuLabel>
        <DropdownMenuSeparator />
        <DropdownMenuItem>{ "Profile" }</DropdownMenuItem>
        <DropdownMenuItem>{ "Settings" }</DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem>{ "Log out" }</DropdownMenuItem>
    </DropdownMenuContent>
</DropdownMenu>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "open", prop_type: "bool", default: "false", description: "Controlled open state" },
        PropDoc { name: "on_open_change", prop_type: "Callback<bool>", default: "-", description: "Open state change handler" },
    ];

    html! { <ComponentPage name="Dropdown Menu" description="Displays a menu to the user triggered by a button." {examples} {props} /> }
}
