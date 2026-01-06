//! Switch component showcase page

use yew::prelude::*;
use shadcn_rs::Switch;

use crate::components::{ComponentPage, Example, PropDoc};

/// Switch showcase page
#[function_component(SwitchPage)]
pub fn switch_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic toggle switch.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Switch id="airplane-mode" />
                    <label html_for="airplane-mode" class="text-sm font-medium">
                        { "Airplane Mode" }
                    </label>
                </div>
            },
            code: r#"<div class="flex items-center space-x-2">
    <Switch id="airplane-mode" />
    <label html_for="airplane-mode">
        { "Airplane Mode" }
    </label>
</div>"#,
        },
        Example {
            title: "Checked",
            description: "A pre-checked switch.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Switch id="checked-switch" checked={true} />
                    <label html_for="checked-switch" class="text-sm font-medium">
                        { "Enabled" }
                    </label>
                </div>
            },
            code: r#"<Switch id="checked-switch" checked={true} />"#,
        },
        Example {
            title: "Disabled",
            description: "A disabled switch.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Switch id="disabled-switch" disabled={true} />
                    <label html_for="disabled-switch" class="text-sm font-medium text-muted-foreground">
                        { "Disabled" }
                    </label>
                </div>
            },
            code: r#"<Switch id="disabled-switch" disabled={true} />"#,
        },
        Example {
            title: "Form Integration",
            description: "Switch used in a settings form.",
            demo: html! {
                <div class="space-y-4">
                    <div class="flex items-center justify-between rounded-lg border p-4">
                        <div class="space-y-0.5">
                            <div class="text-sm font-medium">{ "Marketing emails" }</div>
                            <div class="text-sm text-muted-foreground">
                                { "Receive emails about new products and features." }
                            </div>
                        </div>
                        <Switch />
                    </div>
                    <div class="flex items-center justify-between rounded-lg border p-4">
                        <div class="space-y-0.5">
                            <div class="text-sm font-medium">{ "Security emails" }</div>
                            <div class="text-sm text-muted-foreground">
                                { "Receive emails about your account security." }
                            </div>
                        </div>
                        <Switch checked={true} />
                    </div>
                </div>
            },
            code: r#"<div class="flex items-center justify-between">
    <div>
        <div>{ "Marketing emails" }</div>
        <div>{ "Receive emails about new products." }</div>
    </div>
    <Switch />
</div>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "checked",
            prop_type: "bool",
            default: "false",
            description: "Whether the switch is on",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the switch is disabled",
        },
        PropDoc {
            name: "onchange",
            prop_type: "Callback<bool>",
            default: "-",
            description: "Change event handler",
        },
        PropDoc {
            name: "id",
            prop_type: "Option<String>",
            default: "-",
            description: "HTML id attribute",
        },
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "Additional CSS classes",
        },
    ];

    html! {
        <ComponentPage
            name="Switch"
            description="A control that allows the user to toggle between on and off states."
            {examples}
            {props}
        />
    }
}
