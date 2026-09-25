//! Radio component showcase page

use shadcn_rs::{Label, Radio, RadioGroup};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Radio showcase page
#[function_component(RadioPage)]
pub fn radio_page() -> Html {
    let plan = use_state(|| AttrValue::from("pro"));
    let on_plan_change = {
        let plan = plan.clone();
        Callback::from(move |value: AttrValue| plan.set(value))
    };

    let examples = vec![
        Example {
            title: "Default",
            description: "An uncontrolled radio group with a default value.",
            demo: html! {
                <RadioGroup name="options" default_value="comfortable">
                    <div class="flex items-center space-x-2">
                        <Radio value="default" id="r1" />
                        <Label html_for="r1">{ "Default" }</Label>
                    </div>
                    <div class="flex items-center space-x-2">
                        <Radio value="comfortable" id="r2" />
                        <Label html_for="r2">{ "Comfortable" }</Label>
                    </div>
                    <div class="flex items-center space-x-2">
                        <Radio value="compact" id="r3" />
                        <Label html_for="r3">{ "Compact" }</Label>
                    </div>
                </RadioGroup>
            },
            code: r##"<RadioGroup name="options" default_value="comfortable">
    <div class="flex items-center space-x-2">
        <Radio value="default" id="r1" />
        <Label html_for="r1">{ "Default" }</Label>
    </div>
    <div class="flex items-center space-x-2">
        <Radio value="comfortable" id="r2" />
        <Label html_for="r2">{ "Comfortable" }</Label>
    </div>
    <div class="flex items-center space-x-2">
        <Radio value="compact" id="r3" />
        <Label html_for="r3">{ "Compact" }</Label>
    </div>
</RadioGroup>"##,
        },
        Example {
            title: "Controlled",
            description: "The parent owns the value; on_value_change reports the selection.",
            demo: html! {
                <div class="space-y-2">
                    <RadioGroup
                        name="plan"
                        value={(*plan).clone()}
                        on_value_change={on_plan_change.clone()}
                    >
                        <div class="flex items-center space-x-2">
                            <Radio value="free" id="plan-free" />
                            <Label html_for="plan-free">{ "Free" }</Label>
                        </div>
                        <div class="flex items-center space-x-2">
                            <Radio value="pro" id="plan-pro" />
                            <Label html_for="plan-pro">{ "Pro" }</Label>
                        </div>
                        <div class="flex items-center space-x-2">
                            <Radio value="team" id="plan-team" />
                            <Label html_for="plan-team">{ "Team" }</Label>
                        </div>
                    </RadioGroup>
                    <p class="text-sm text-muted-foreground">
                        { format!("Selected: {}", *plan) }
                    </p>
                </div>
            },
            code: r##"let plan = use_state(|| AttrValue::from("pro"));
let on_value_change = {
    let plan = plan.clone();
    Callback::from(move |value: AttrValue| plan.set(value))
};

<RadioGroup name="plan" value={(*plan).clone()} {on_value_change}>
    <Radio value="free" id="plan-free" />
    <Radio value="pro" id="plan-pro" />
    <Radio value="team" id="plan-team" />
</RadioGroup>"##,
        },
        Example {
            title: "Disabled",
            description: "A disabled item, and a group disabled as a whole.",
            demo: html! {
                <div class="space-y-4">
                    <RadioGroup name="disabled-example" default_value="option1">
                        <div class="flex items-center space-x-2">
                            <Radio value="option1" id="d1" />
                            <Label html_for="d1">{ "Option 1" }</Label>
                        </div>
                        <div class="flex items-center space-x-2">
                            <Radio value="option2" id="d2" disabled={true} />
                            <Label html_for="d2" class="text-muted-foreground">{ "Option 2 (disabled)" }</Label>
                        </div>
                        <div class="flex items-center space-x-2">
                            <Radio value="option3" id="d3" />
                            <Label html_for="d3">{ "Option 3" }</Label>
                        </div>
                    </RadioGroup>
                    <RadioGroup name="disabled-group" default_value="on" disabled={true}>
                        <div class="flex items-center space-x-2">
                            <Radio value="on" id="dg1" />
                            <Label html_for="dg1" class="text-muted-foreground">{ "On" }</Label>
                        </div>
                        <div class="flex items-center space-x-2">
                            <Radio value="off" id="dg2" />
                            <Label html_for="dg2" class="text-muted-foreground">{ "Off" }</Label>
                        </div>
                    </RadioGroup>
                </div>
            },
            code: r##"<RadioGroup name="disabled-example" default_value="option1">
    <Radio value="option1" id="d1" />
    <Radio value="option2" id="d2" disabled={true} />
    <Radio value="option3" id="d3" />
</RadioGroup>

<RadioGroup name="disabled-group" default_value="on" disabled={true}>
    <Radio value="on" id="dg1" />
    <Radio value="off" id="dg2" />
</RadioGroup>"##,
        },
        Example {
            title: "With Description",
            description: "Radio options with descriptions.",
            demo: html! {
                <RadioGroup name="payment" default_value="card">
                    <div class="flex items-start space-x-2 p-4 border rounded-lg">
                        <Radio value="card" id="card" />
                        <div class="grid gap-1.5 leading-none">
                            <Label html_for="card">{ "Card" }</Label>
                            <p class="text-sm text-muted-foreground">
                                { "Pay with your credit card." }
                            </p>
                        </div>
                    </div>
                    <div class="flex items-start space-x-2 p-4 border rounded-lg">
                        <Radio value="paypal" id="paypal" />
                        <div class="grid gap-1.5 leading-none">
                            <Label html_for="paypal">{ "PayPal" }</Label>
                            <p class="text-sm text-muted-foreground">
                                { "Pay with your PayPal account." }
                            </p>
                        </div>
                    </div>
                </RadioGroup>
            },
            code: r##"<RadioGroup name="payment" default_value="card">
    <div class="flex items-start space-x-2 p-4 border rounded-lg">
        <Radio value="card" id="card" />
        <div class="grid gap-1.5">
            <Label html_for="card">{ "Card" }</Label>
            <p>{ "Pay with your credit card." }</p>
        </div>
    </div>
    // ...
</RadioGroup>"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "RadioGroup value",
            prop_type: "Option<AttrValue>",
            default: "None",
            description: "Controlled selected value",
        },
        PropDoc {
            name: "RadioGroup default_value",
            prop_type: "Option<AttrValue>",
            default: "None",
            description: "Initial selected value when uncontrolled",
        },
        PropDoc {
            name: "RadioGroup on_value_change",
            prop_type: "Option<Callback<AttrValue>>",
            default: "-",
            description: "Called with the newly selected value",
        },
        PropDoc {
            name: "RadioGroup name",
            prop_type: "AttrValue",
            default: "-",
            description: "Name given to every radio in the group",
        },
        PropDoc {
            name: "RadioGroup disabled",
            prop_type: "bool",
            default: "false",
            description: "Disables every radio in the group",
        },
        PropDoc {
            name: "Radio value",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Value of the radio",
        },
        PropDoc {
            name: "Radio checked",
            prop_type: "Option<bool>",
            default: "None",
            description: "Controlled state for a standalone radio (ignored inside a group)",
        },
        PropDoc {
            name: "Radio default_checked",
            prop_type: "bool",
            default: "false",
            description: "Initial state for an uncontrolled standalone radio",
        },
        PropDoc {
            name: "Radio disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the radio is disabled",
        },
    ];

    html! {
        <ComponentPage
            name="Radio Group"
            description="A set of checkable buttons where only one can be checked at a time."
            {examples}
            {props}
        />
    }
}
