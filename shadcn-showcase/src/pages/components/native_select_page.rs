//! Native Select component showcase page

use shadcn_rs::{
    Button, Label, NativeSelect, NativeSelectOptGroup, NativeSelectOption, Size, Variant,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Native Select showcase page
#[function_component(NativeSelectPage)]
pub fn native_select_page() -> Html {
    let theme = use_state(|| AttrValue::from("dark"));
    let on_theme_change = {
        let theme = theme.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            theme.set(AttrValue::from(select.value()));
        })
    };
    let reset_theme = {
        let theme = theme.clone();
        Callback::from(move |_: MouseEvent| theme.set(AttrValue::from("system")))
    };

    let examples = vec![
        Example {
            title: "Default and Controlled Value",
            description: "default_value preselects an option once; value keeps the select in sync with parent state.",
            demo: html! {
                <div class="space-y-4">
                    <div class="space-y-2">
                        <Label html_for="ns-size">{ "Size (default_value)" }</Label>
                        <NativeSelect id="ns-size" default_value="m">
                            <NativeSelectOption value="s">{ "Small" }</NativeSelectOption>
                            <NativeSelectOption value="m">{ "Medium" }</NativeSelectOption>
                            <NativeSelectOption value="l">{ "Large" }</NativeSelectOption>
                        </NativeSelect>
                    </div>
                    <div class="space-y-2">
                        <Label html_for="ns-theme"><>{ format!("Theme (value = {})", *theme) }</></Label>
                        <NativeSelect id="ns-theme" value={(*theme).clone()} onchange={on_theme_change.clone()}>
                            <NativeSelectOption value="light">{ "Light" }</NativeSelectOption>
                            <NativeSelectOption value="dark">{ "Dark" }</NativeSelectOption>
                            <NativeSelectOption value="system">{ "System" }</NativeSelectOption>
                        </NativeSelect>
                        <Button variant={Variant::Outline} size={Size::Sm} onclick={reset_theme.clone()}>
                            { "Reset to System" }
                        </Button>
                    </div>
                </div>
            },
            code: r#"<NativeSelect default_value="m">
    <NativeSelectOption value="s">{ "Small" }</NativeSelectOption>
    <NativeSelectOption value="m">{ "Medium" }</NativeSelectOption>
    <NativeSelectOption value="l">{ "Large" }</NativeSelectOption>
</NativeSelect>

let theme = use_state(|| AttrValue::from("dark"));
<NativeSelect value={(*theme).clone()} onchange={on_theme_change}>
    // options...
</NativeSelect>"#,
        },
        Example {
            title: "Default",
            description: "A basic native select dropdown.",
            demo: html! {
                <div style="width: 200px;">
                    <NativeSelect>
                        <NativeSelectOption value="" disabled=true selected=true>
                            { "Select status" }
                        </NativeSelectOption>
                        <NativeSelectOption value="todo">{ "Todo" }</NativeSelectOption>
                        <NativeSelectOption value="in-progress">{ "In Progress" }</NativeSelectOption>
                        <NativeSelectOption value="done">{ "Done" }</NativeSelectOption>
                        <NativeSelectOption value="cancelled">{ "Cancelled" }</NativeSelectOption>
                    </NativeSelect>
                </div>
            },
            code: r#"<NativeSelect>
    <NativeSelectOption value="" disabled=true selected=true>
        { "Select status" }
    </NativeSelectOption>
    <NativeSelectOption value="todo">{ "Todo" }</NativeSelectOption>
    <NativeSelectOption value="in-progress">{ "In Progress" }</NativeSelectOption>
    <NativeSelectOption value="done">{ "Done" }</NativeSelectOption>
</NativeSelect>"#,
        },
        Example {
            title: "With Groups",
            description: "Organize options using NativeSelectOptGroup for better categorization.",
            demo: html! {
                <div style="width: 250px;">
                    <NativeSelect>
                        <NativeSelectOption value="" disabled=true selected=true>
                            { "Select department" }
                        </NativeSelectOption>
                        <NativeSelectOptGroup label="Engineering">
                            <NativeSelectOption value="frontend">{ "Frontend" }</NativeSelectOption>
                            <NativeSelectOption value="backend">{ "Backend" }</NativeSelectOption>
                            <NativeSelectOption value="devops">{ "DevOps" }</NativeSelectOption>
                        </NativeSelectOptGroup>
                        <NativeSelectOptGroup label="Sales">
                            <NativeSelectOption value="sales-rep">{ "Sales Rep" }</NativeSelectOption>
                            <NativeSelectOption value="account-manager">{ "Account Manager" }</NativeSelectOption>
                            <NativeSelectOption value="sales-director">{ "Sales Director" }</NativeSelectOption>
                        </NativeSelectOptGroup>
                        <NativeSelectOptGroup label="Other">
                            <NativeSelectOption value="support">{ "Customer Support" }</NativeSelectOption>
                            <NativeSelectOption value="product">{ "Product Manager" }</NativeSelectOption>
                            <NativeSelectOption value="operations">{ "Operations Manager" }</NativeSelectOption>
                        </NativeSelectOptGroup>
                    </NativeSelect>
                </div>
            },
            code: r#"<NativeSelect>
    <NativeSelectOptGroup label="Engineering">
        <NativeSelectOption value="frontend">{ "Frontend" }</NativeSelectOption>
        <NativeSelectOption value="backend">{ "Backend" }</NativeSelectOption>
        <NativeSelectOption value="devops">{ "DevOps" }</NativeSelectOption>
    </NativeSelectOptGroup>
    <NativeSelectOptGroup label="Sales">
        <NativeSelectOption value="sales-rep">{ "Sales Rep" }</NativeSelectOption>
        ...
    </NativeSelectOptGroup>
</NativeSelect>"#,
        },
        Example {
            title: "Disabled State",
            description: "Disable individual options or the entire select component.",
            demo: html! {
                <div class="flex gap-4" style="display: flex; gap: 1rem; align-items: start;">
                    <div style="width: 180px;">
                        <Label>{ "Disabled Select" }</Label>
                        <NativeSelect disabled=true>
                            <NativeSelectOption value="option">{ "Can't select" }</NativeSelectOption>
                        </NativeSelect>
                    </div>
                    <div style="width: 180px;">
                        <Label>{ "Disabled Options" }</Label>
                        <NativeSelect>
                            <NativeSelectOption value="" disabled=true selected=true>
                                { "Select priority" }
                            </NativeSelectOption>
                            <NativeSelectOption value="low">{ "Low" }</NativeSelectOption>
                            <NativeSelectOption value="medium">{ "Medium" }</NativeSelectOption>
                            <NativeSelectOption value="high" disabled=true>{ "High (unavailable)" }</NativeSelectOption>
                            <NativeSelectOption value="critical" disabled=true>{ "Critical (unavailable)" }</NativeSelectOption>
                        </NativeSelect>
                    </div>
                </div>
            },
            code: r#"// Disabled select
<NativeSelect disabled=true>
    <NativeSelectOption value="option">{ "Can't select" }</NativeSelectOption>
</NativeSelect>

// Disabled options
<NativeSelect>
    <NativeSelectOption value="high" disabled=true>
        { "High (unavailable)" }
    </NativeSelectOption>
</NativeSelect>"#,
        },
        Example {
            title: "Invalid State",
            description: "Show validation errors with the aria-invalid attribute and error styling.",
            demo: html! {
                <div style="width: 200px;">
                    <Label>{ "Role (required)" }</Label>
                    <NativeSelect aria_invalid=true>
                        <NativeSelectOption value="" disabled=true selected=true>
                            { "Select role" }
                        </NativeSelectOption>
                        <NativeSelectOption value="admin">{ "Admin" }</NativeSelectOption>
                        <NativeSelectOption value="editor">{ "Editor" }</NativeSelectOption>
                        <NativeSelectOption value="viewer">{ "Viewer" }</NativeSelectOption>
                    </NativeSelect>
                    <p style="color: hsl(var(--color-destructive)); font-size: 0.875rem; margin-top: 0.25rem;">
                        { "Please select a role" }
                    </p>
                </div>
            },
            code: r#"<NativeSelect aria_invalid=true>
    <NativeSelectOption value="" disabled=true selected=true>
        { "Select role" }
    </NativeSelectOption>
    <NativeSelectOption value="admin">{ "Admin" }</NativeSelectOption>
    <NativeSelectOption value="editor">{ "Editor" }</NativeSelectOption>
</NativeSelect>"#,
        },
        Example {
            title: "Sizes",
            description: "Native select with different sizes.",
            demo: html! {
                <div style="display: flex; flex-direction: column; gap: 1rem;">
                    <div style="width: 200px;">
                        <Label>{ "Extra Small" }</Label>
                        <NativeSelect size={shadcn_rs::Size::Xs}>
                            <NativeSelectOption value="xs">{ "Extra Small" }</NativeSelectOption>
                        </NativeSelect>
                    </div>
                    <div style="width: 200px;">
                        <Label>{ "Small" }</Label>
                        <NativeSelect size={shadcn_rs::Size::Sm}>
                            <NativeSelectOption value="sm">{ "Small" }</NativeSelectOption>
                        </NativeSelect>
                    </div>
                    <div style="width: 200px;">
                        <Label>{ "Medium (default)" }</Label>
                        <NativeSelect size={shadcn_rs::Size::Md}>
                            <NativeSelectOption value="md">{ "Medium" }</NativeSelectOption>
                        </NativeSelect>
                    </div>
                    <div style="width: 200px;">
                        <Label>{ "Large" }</Label>
                        <NativeSelect size={shadcn_rs::Size::Lg}>
                            <NativeSelectOption value="lg">{ "Large" }</NativeSelectOption>
                        </NativeSelect>
                    </div>
                </div>
            },
            code: r#"<NativeSelect size={Size::Sm}>
    <NativeSelectOption value="sm">{ "Small" }</NativeSelectOption>
</NativeSelect>

<NativeSelect size={Size::Md}>
    <NativeSelectOption value="md">{ "Medium" }</NativeSelectOption>
</NativeSelect>

<NativeSelect size={Size::Lg}>
    <NativeSelectOption value="lg">{ "Large" }</NativeSelectOption>
</NativeSelect>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "value",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Controlled selected value",
        },
        PropDoc {
            name: "default_value",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Default value (uncontrolled)",
        },
        PropDoc {
            name: "size",
            prop_type: "Size",
            default: "Md",
            description: "Size variant (Xs, Sm, Md, Lg, Xl)",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the select is disabled",
        },
        PropDoc {
            name: "required",
            prop_type: "bool",
            default: "false",
            description: "Whether the field is required",
        },
        PropDoc {
            name: "aria_invalid",
            prop_type: "Option<bool>",
            default: "-",
            description: "Set to true to show invalid/error styling",
        },
        PropDoc {
            name: "onchange",
            prop_type: "Callback<Event>",
            default: "-",
            description: "Change event handler",
        },
    ];

    let notes = html! {
        <div class="notes">
            <h3>{ "Native Select vs Select" }</h3>
            <ul>
                <li>{ "Use " }<strong>{ "NativeSelect" }</strong>{ " when you need native browser behavior, better performance, or mobile-optimized dropdowns." }</li>
                <li>{ "Use " }<strong>{ "Select" }</strong>{ " when you need custom styling, animations, or complex interactions." }</li>
            </ul>
            <p>{ "The NativeSelect component provides native HTML select functionality with consistent styling that matches your design system." }</p>
            <h3>{ "Accessibility" }</h3>
            <ul>
                <li>{ "Maintains all native HTML select accessibility features" }</li>
                <li>{ "Screen readers can navigate through options using arrow keys" }</li>
                <li>{ "The chevron icon is marked as " }<code>{ "aria-hidden=\"true\"" }</code>{ " to avoid duplication" }</li>
                <li>{ "Use " }<code>{ "aria-label" }</code>{ " or " }<code>{ "aria-labelledby" }</code>{ " for additional context when needed" }</li>
            </ul>
        </div>
    };

    html! {
        <ComponentPage
            name="Native Select"
            description="A styled native HTML select element with consistent design system integration."
            {examples}
            {props}
            notes={notes}
        />
    }
}
