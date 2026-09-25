//! Switch component showcase page

use shadcn_rs::{Button, Size, Switch, Variant};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Switch showcase page
#[function_component(SwitchPage)]
pub fn switch_page() -> Html {
    let wifi = use_state(|| true);
    let on_checked_change = {
        let wifi = wifi.clone();
        Callback::from(move |value: bool| wifi.set(value))
    };
    let force_off = {
        let wifi = wifi.clone();
        Callback::from(move |_: MouseEvent| wifi.set(false))
    };

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
            title: "Default Checked",
            description: "An uncontrolled switch that starts on.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Switch id="checked-switch" default_checked={true} />
                    <label html_for="checked-switch" class="text-sm font-medium">
                        { "Enabled" }
                    </label>
                </div>
            },
            code: r#"<Switch id="checked-switch" default_checked={true} />"#,
        },
        Example {
            title: "Controlled",
            description: "The parent owns the state and can force the switch off.",
            demo: html! {
                <div class="flex items-center space-x-2">
                    <Switch
                        id="wifi-switch"
                        checked={*wifi}
                        on_checked_change={on_checked_change.clone()}
                    />
                    <label html_for="wifi-switch" class="text-sm font-medium">
                        { if *wifi { "Wi-Fi on" } else { "Wi-Fi off" } }
                    </label>
                    <Button variant={Variant::Outline} size={Size::Sm} onclick={force_off.clone()}>
                        { "Turn off" }
                    </Button>
                </div>
            },
            code: r#"let wifi = use_state(|| true);
let on_checked_change = {
    let wifi = wifi.clone();
    Callback::from(move |value: bool| wifi.set(value))
};

<Switch checked={*wifi} {on_checked_change} />
<Button onclick={move |_| wifi.set(false)}>{ "Turn off" }</Button>"#,
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
                        <Switch default_checked={true} />
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
            prop_type: "Option<bool>",
            default: "None",
            description: "Controlled on/off state (leave unset for uncontrolled)",
        },
        PropDoc {
            name: "default_checked",
            prop_type: "bool",
            default: "false",
            description: "Initial state when uncontrolled",
        },
        PropDoc {
            name: "on_checked_change",
            prop_type: "Option<Callback<bool>>",
            default: "-",
            description: "Called with the new checked value",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the switch is disabled",
        },
        PropDoc {
            name: "onchange",
            prop_type: "Option<Callback<Event>>",
            default: "-",
            description: "Raw click/key event that toggled the switch",
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
