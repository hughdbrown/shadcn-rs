//! Radio component showcase page

use shadcn_rs::{Label, Radio, RadioGroup};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Radio showcase page
#[function_component(RadioPage)]
pub fn radio_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic radio group.",
            demo: html! {
                <RadioGroup name="options">
                    <div class="flex items-center space-x-2">
                        <Radio value="default" id="r1" name="options" />
                        <Label html_for="r1">{ "Default" }</Label>
                    </div>
                    <div class="flex items-center space-x-2">
                        <Radio value="comfortable" id="r2" name="options" checked={true} />
                        <Label html_for="r2">{ "Comfortable" }</Label>
                    </div>
                    <div class="flex items-center space-x-2">
                        <Radio value="compact" id="r3" name="options" />
                        <Label html_for="r3">{ "Compact" }</Label>
                    </div>
                </RadioGroup>
            },
            code: r##"<RadioGroup name="options">
    <div class="flex items-center space-x-2">
        <Radio value="default" id="r1" name="options" />
        <Label html_for="r1">{ "Default" }</Label>
    </div>
    <div class="flex items-center space-x-2">
        <Radio value="comfortable" id="r2" name="options" checked={true} />
        <Label html_for="r2">{ "Comfortable" }</Label>
    </div>
    <div class="flex items-center space-x-2">
        <Radio value="compact" id="r3" name="options" />
        <Label html_for="r3">{ "Compact" }</Label>
    </div>
</RadioGroup>"##,
        },
        Example {
            title: "Disabled",
            description: "Radio group with disabled option.",
            demo: html! {
                <RadioGroup name="disabled-example">
                    <div class="flex items-center space-x-2">
                        <Radio value="option1" id="d1" name="disabled-example" checked={true} />
                        <Label html_for="d1">{ "Option 1" }</Label>
                    </div>
                    <div class="flex items-center space-x-2">
                        <Radio value="option2" id="d2" name="disabled-example" disabled={true} />
                        <Label html_for="d2" class="text-muted-foreground">{ "Option 2 (disabled)" }</Label>
                    </div>
                    <div class="flex items-center space-x-2">
                        <Radio value="option3" id="d3" name="disabled-example" />
                        <Label html_for="d3">{ "Option 3" }</Label>
                    </div>
                </RadioGroup>
            },
            code: r##"<RadioGroup name="disabled-example">
    <Radio value="option1" id="d1" checked={true} />
    <Radio value="option2" id="d2" disabled={true} />
    <Radio value="option3" id="d3" />
</RadioGroup>"##,
        },
        Example {
            title: "With Description",
            description: "Radio options with descriptions.",
            demo: html! {
                <RadioGroup name="payment">
                    <div class="flex items-start space-x-2 p-4 border rounded-lg">
                        <Radio value="card" id="card" name="payment" checked={true} />
                        <div class="grid gap-1.5 leading-none">
                            <Label html_for="card">{ "Card" }</Label>
                            <p class="text-sm text-muted-foreground">
                                { "Pay with your credit card." }
                            </p>
                        </div>
                    </div>
                    <div class="flex items-start space-x-2 p-4 border rounded-lg">
                        <Radio value="paypal" id="paypal" name="payment" />
                        <div class="grid gap-1.5 leading-none">
                            <Label html_for="paypal">{ "PayPal" }</Label>
                            <p class="text-sm text-muted-foreground">
                                { "Pay with your PayPal account." }
                            </p>
                        </div>
                    </div>
                </RadioGroup>
            },
            code: r##"<RadioGroup name="payment">
    <div class="flex items-start space-x-2 p-4 border rounded-lg">
        <Radio value="card" id="card" name="payment" />
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
            name: "checked",
            prop_type: "bool",
            default: "false",
            description: "Whether the radio is checked",
        },
        PropDoc {
            name: "name",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Name attribute to group radios",
        },
        PropDoc {
            name: "value",
            prop_type: "Option<AttrValue>",
            default: "-",
            description: "Value of the radio",
        },
        PropDoc {
            name: "onchange",
            prop_type: "Option<Callback<Event>>",
            default: "-",
            description: "Change event handler",
        },
        PropDoc {
            name: "disabled",
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
