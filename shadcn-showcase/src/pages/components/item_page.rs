//! Item component showcase page

use shadcn_rs::Item;
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ItemPage)]
pub fn item_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic list item.",
            demo: html! {
                <div class="border rounded-lg">
                    <Item>{ "List Item" }</Item>
                    <Item>{ "Another Item" }</Item>
                    <Item>{ "Third Item" }</Item>
                </div>
            },
            code: r#"<Item>{ "List Item" }</Item>"#,
        },
        Example {
            title: "Selectable",
            description: "Items that can be selected.",
            demo: html! {
                <div class="border rounded-lg">
                    <Item selected={true}>{ "Selected Item" }</Item>
                    <Item>{ "Regular Item" }</Item>
                    <Item disabled={true}>{ "Disabled Item" }</Item>
                </div>
            },
            code: r#"<Item selected={true}>{ "Selected" }</Item>
<Item disabled={true}>{ "Disabled" }</Item>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "selected",
            prop_type: "bool",
            default: "false",
            description: "Whether the item is selected",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the item is disabled",
        },
        PropDoc {
            name: "onclick",
            prop_type: "Callback<MouseEvent>",
            default: "-",
            description: "Click handler",
        },
        PropDoc {
            name: "children",
            prop_type: "Children",
            default: "-",
            description: "Item content",
        },
    ];

    html! { <ComponentPage name="Item" description="A generic item component for lists and menus." {examples} {props} /> }
}
