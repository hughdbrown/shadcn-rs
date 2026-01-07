//! Button component showcase page

use shadcn_rs::{Button, Size, Variant};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Button showcase page
#[function_component(ButtonPage)]
pub fn button_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "The default button style.",
            demo: html! {
                <Button>{ "Button" }</Button>
            },
            code: r#"<Button>{ "Button" }</Button>"#,
        },
        Example {
            title: "Variants",
            description: "Different button variants for various contexts.",
            demo: html! {
                <div class="flex gap-2 flex-wrap">
                    <Button variant={Variant::Default}>{ "Default" }</Button>
                    <Button variant={Variant::Primary}>{ "Primary" }</Button>
                    <Button variant={Variant::Secondary}>{ "Secondary" }</Button>
                    <Button variant={Variant::Destructive}>{ "Destructive" }</Button>
                    <Button variant={Variant::Outline}>{ "Outline" }</Button>
                    <Button variant={Variant::Ghost}>{ "Ghost" }</Button>
                    <Button variant={Variant::Link}>{ "Link" }</Button>
                </div>
            },
            code: r#"<Button variant={Variant::Default}>{ "Default" }</Button>
<Button variant={Variant::Primary}>{ "Primary" }</Button>
<Button variant={Variant::Secondary}>{ "Secondary" }</Button>
<Button variant={Variant::Destructive}>{ "Destructive" }</Button>
<Button variant={Variant::Outline}>{ "Outline" }</Button>
<Button variant={Variant::Ghost}>{ "Ghost" }</Button>
<Button variant={Variant::Link}>{ "Link" }</Button>"#,
        },
        Example {
            title: "Sizes",
            description: "Buttons in different sizes.",
            demo: html! {
                <div class="flex gap-2 items-center flex-wrap">
                    <Button size={Size::Sm}>{ "Small" }</Button>
                    <Button size={Size::Md}>{ "Medium" }</Button>
                    <Button size={Size::Lg}>{ "Large" }</Button>
                </div>
            },
            code: r#"<Button size={Size::Sm}>{ "Small" }</Button>
<Button size={Size::Md}>{ "Medium" }</Button>
<Button size={Size::Lg}>{ "Large" }</Button>"#,
        },
        Example {
            title: "Disabled",
            description: "Disabled button state.",
            demo: html! {
                <Button disabled={true}>{ "Disabled" }</Button>
            },
            code: r#"<Button disabled={true}>{ "Disabled" }</Button>"#,
        },
        Example {
            title: "Loading",
            description: "Button with loading state.",
            demo: html! {
                <Button loading={true}>{ "Loading..." }</Button>
            },
            code: r#"<Button loading={true}>{ "Loading..." }</Button>"#,
        },
        Example {
            title: "With Icon",
            description: "Button with an icon.",
            demo: html! {
                <div class="flex gap-2">
                    <Button>
                        <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <path d="M5 12h14M12 5l7 7-7 7"/>
                        </svg>
                        { "Continue" }
                    </Button>
                </div>
            },
            code: r#"<Button>
    <Icon name="arrow-right" />
    { "Continue" }
</Button>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "variant",
            prop_type: "Variant",
            default: "Default",
            description: "The visual style variant of the button",
        },
        PropDoc {
            name: "size",
            prop_type: "Size",
            default: "Md",
            description: "The size of the button",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the button is disabled",
        },
        PropDoc {
            name: "loading",
            prop_type: "bool",
            default: "false",
            description: "Whether to show a loading spinner",
        },
        PropDoc {
            name: "button_type",
            prop_type: "&str",
            default: "\"button\"",
            description: "The HTML button type (button, submit, reset)",
        },
        PropDoc {
            name: "onclick",
            prop_type: "Callback<MouseEvent>",
            default: "-",
            description: "Click event handler",
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
            description: "Button content",
        },
    ];

    html! {
        <ComponentPage
            name="Button"
            description="Displays a button or a component that looks like a button."
            {examples}
            {props}
        />
    }
}
