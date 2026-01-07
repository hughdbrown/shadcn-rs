//! Slider component showcase page

use shadcn_rs::Slider;
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

/// Slider showcase page
#[function_component(SliderPage)]
pub fn slider_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic slider.",
            demo: html! {
                <Slider default_value={vec![50.0]} max={100.0} step={1.0} class="w-[300px]" />
            },
            code: r##"<Slider default_value={vec![50.0]} max={100.0} step={1.0} />"##,
        },
        Example {
            title: "Range",
            description: "Slider with min and max range.",
            demo: html! {
                <div class="space-y-4 w-[300px]">
                    <div class="flex justify-between text-sm">
                        <span>{ "0" }</span>
                        <span>{ "100" }</span>
                    </div>
                    <Slider default_value={vec![25.0]} min={0.0} max={100.0} step={1.0} />
                </div>
            },
            code: r##"<Slider default_value={vec![25.0]} min={0.0} max={100.0} step={1.0} />"##,
        },
        Example {
            title: "Step",
            description: "Slider with step intervals.",
            demo: html! {
                <div class="space-y-4 w-[300px]">
                    <p class="text-sm text-muted-foreground">{ "Step: 10" }</p>
                    <Slider default_value={vec![50.0]} max={100.0} step={10.0} />
                </div>
            },
            code: r##"<Slider default_value={vec![50.0]} max={100.0} step={10.0} />"##,
        },
        Example {
            title: "Disabled",
            description: "A disabled slider.",
            demo: html! {
                <Slider default_value={vec![50.0]} max={100.0} step={1.0} disabled={true} class="w-[300px]" />
            },
            code: r##"<Slider default_value={vec![50.0]} disabled={true} />"##,
        },
        Example {
            title: "With Labels",
            description: "Slider with value display.",
            demo: html! {
                <div class="space-y-4 w-[300px]">
                    <div class="flex justify-between">
                        <span class="text-sm font-medium">{ "Volume" }</span>
                        <span class="text-sm text-muted-foreground">{ "50%" }</span>
                    </div>
                    <Slider default_value={vec![50.0]} max={100.0} step={1.0} />
                </div>
            },
            code: r##"<div class="space-y-4">
    <div class="flex justify-between">
        <span>{ "Volume" }</span>
        <span>{ format!("{}%", value) }</span>
    </div>
    <Slider value={vec![value]} onchange={on_change} />
</div>"##,
        },
        Example {
            title: "Range Slider",
            description: "Slider with two handles for range selection.",
            demo: html! {
                <div class="space-y-4 w-[300px]">
                    <p class="text-sm text-muted-foreground">{ "Select a range" }</p>
                    <Slider default_value={vec![25.0, 75.0]} max={100.0} step={1.0} />
                </div>
            },
            code: r##"<Slider default_value={vec![25.0, 75.0]} max={100.0} step={1.0} />"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "value",
            prop_type: "Option<Vec<f64>>",
            default: "-",
            description: "Controlled value(s)",
        },
        PropDoc {
            name: "default_value",
            prop_type: "Option<Vec<f64>>",
            default: "-",
            description: "Default value(s) (uncontrolled)",
        },
        PropDoc {
            name: "min",
            prop_type: "f64",
            default: "0.0",
            description: "Minimum value",
        },
        PropDoc {
            name: "max",
            prop_type: "f64",
            default: "100.0",
            description: "Maximum value",
        },
        PropDoc {
            name: "step",
            prop_type: "f64",
            default: "1.0",
            description: "Step increment",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the slider is disabled",
        },
        PropDoc {
            name: "on_value_change",
            prop_type: "Option<Callback<Vec<f64>>>",
            default: "-",
            description: "Change event handler",
        },
    ];

    html! {
        <ComponentPage
            name="Slider"
            description="An input where the user selects a value from within a given range."
            {examples}
            {props}
        />
    }
}
