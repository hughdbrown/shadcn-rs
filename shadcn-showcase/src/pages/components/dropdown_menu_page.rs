//! DropdownMenu component showcase page

use shadcn_rs::{
    Button, DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuRadioGroup, DropdownMenuRadioItem, DropdownMenuSeparator, DropdownMenuTrigger,
    Variant,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(DropdownRadioDemo)]
fn dropdown_radio_demo() -> Html {
    let position = use_state(|| AttrValue::from("bottom"));
    let on_value_change = {
        let position = position.clone();
        Callback::from(move |value: AttrValue| position.set(value))
    };

    html! {
        <DropdownMenu>
            <DropdownMenuTrigger>
                <Button variant={Variant::Outline}><>{ format!("Panel: {}", *position) }</></Button>
            </DropdownMenuTrigger>
            <DropdownMenuContent>
                <DropdownMenuLabel>{ "Panel Position" }</DropdownMenuLabel>
                <DropdownMenuSeparator />
                <DropdownMenuRadioGroup value={Some((*position).clone())} on_value_change={Some(on_value_change)}>
                    <DropdownMenuRadioItem value="top">{ "Top" }</DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value="bottom">{ "Bottom" }</DropdownMenuRadioItem>
                    <DropdownMenuRadioItem value="right">{ "Right" }</DropdownMenuRadioItem>
                </DropdownMenuRadioGroup>
            </DropdownMenuContent>
        </DropdownMenu>
    }
}

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
        Example {
            title: "Radio Group",
            description: "Radio items read the group's value and report selection through on_value_change.",
            demo: html! { <DropdownRadioDemo /> },
            code: r#"<DropdownMenuRadioGroup value={Some((*position).clone())} on_value_change={Some(on_value_change)}>
    <DropdownMenuRadioItem value="top">{ "Top" }</DropdownMenuRadioItem>
    <DropdownMenuRadioItem value="bottom">{ "Bottom" }</DropdownMenuRadioItem>
    <DropdownMenuRadioItem value="right">{ "Right" }</DropdownMenuRadioItem>
</DropdownMenuRadioGroup>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "open",
            prop_type: "bool",
            default: "false",
            description: "Controlled open state",
        },
        PropDoc {
            name: "on_open_change",
            prop_type: "Callback<bool>",
            default: "-",
            description: "Open state change handler",
        },
    ];

    html! { <ComponentPage name="Dropdown Menu" description="Displays a menu to the user triggered by a button." {examples} {props} /> }
}
