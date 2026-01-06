//! Select component showcase page

use yew::prelude::*;
use shadcn_rs::{Select, SelectTrigger, SelectValue, SelectContent, SelectItem, SelectGroup, SelectLabel};

use crate::components::{ComponentPage, Example, PropDoc};

/// Select showcase page
#[function_component(SelectPage)]
pub fn select_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic select dropdown.",
            demo: html! {
                <Select>
                    <SelectTrigger class="w-[180px]">
                        <SelectValue placeholder="Select a fruit" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="apple">{ "Apple" }</SelectItem>
                        <SelectItem value="banana">{ "Banana" }</SelectItem>
                        <SelectItem value="orange">{ "Orange" }</SelectItem>
                        <SelectItem value="grape">{ "Grape" }</SelectItem>
                    </SelectContent>
                </Select>
            },
            code: r#"<Select>
    <SelectTrigger class="w-[180px]">
        <SelectValue placeholder="Select a fruit" />
    </SelectTrigger>
    <SelectContent>
        <SelectItem value="apple">{ "Apple" }</SelectItem>
        <SelectItem value="banana">{ "Banana" }</SelectItem>
        <SelectItem value="orange">{ "Orange" }</SelectItem>
    </SelectContent>
</Select>"#,
        },
        Example {
            title: "Disabled",
            description: "A disabled select.",
            demo: html! {
                <Select disabled={true}>
                    <SelectTrigger class="w-[180px]">
                        <SelectValue placeholder="Select..." />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectItem value="1">{ "Option 1" }</SelectItem>
                        <SelectItem value="2">{ "Option 2" }</SelectItem>
                    </SelectContent>
                </Select>
            },
            code: r#"<Select disabled={true}>
    <SelectTrigger>
        <SelectValue placeholder="Select..." />
    </SelectTrigger>
    ...
</Select>"#,
        },
        Example {
            title: "Grouped",
            description: "Select with grouped options.",
            demo: html! {
                <Select>
                    <SelectTrigger class="w-[280px]">
                        <SelectValue placeholder="Select a timezone" />
                    </SelectTrigger>
                    <SelectContent>
                        <SelectGroup>
                            <SelectLabel>{ "North America" }</SelectLabel>
                            <SelectItem value="est">{ "Eastern Time (ET)" }</SelectItem>
                            <SelectItem value="cst">{ "Central Time (CT)" }</SelectItem>
                            <SelectItem value="mst">{ "Mountain Time (MT)" }</SelectItem>
                            <SelectItem value="pst">{ "Pacific Time (PT)" }</SelectItem>
                        </SelectGroup>
                        <SelectGroup>
                            <SelectLabel>{ "Europe" }</SelectLabel>
                            <SelectItem value="gmt">{ "Greenwich Mean Time (GMT)" }</SelectItem>
                            <SelectItem value="cet">{ "Central European Time (CET)" }</SelectItem>
                        </SelectGroup>
                    </SelectContent>
                </Select>
            },
            code: r#"<Select>
    <SelectTrigger>
        <SelectValue placeholder="Select a timezone" />
    </SelectTrigger>
    <SelectContent>
        <SelectGroup>
            <SelectLabel>{ "North America" }</SelectLabel>
            <SelectItem value="est">{ "Eastern Time" }</SelectItem>
            ...
        </SelectGroup>
        <SelectGroup>
            <SelectLabel>{ "Europe" }</SelectLabel>
            ...
        </SelectGroup>
    </SelectContent>
</Select>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "value",
            prop_type: "Option<String>",
            default: "-",
            description: "Controlled selected value",
        },
        PropDoc {
            name: "default_value",
            prop_type: "Option<String>",
            default: "-",
            description: "Default value (uncontrolled)",
        },
        PropDoc {
            name: "disabled",
            prop_type: "bool",
            default: "false",
            description: "Whether the select is disabled",
        },
        PropDoc {
            name: "onchange",
            prop_type: "Callback<String>",
            default: "-",
            description: "Change event handler",
        },
        PropDoc {
            name: "placeholder",
            prop_type: "Option<String>",
            default: "-",
            description: "Placeholder text when no value is selected",
        },
    ];

    html! {
        <ComponentPage
            name="Select"
            description="Displays a list of options for the user to pick from."
            {examples}
            {props}
        />
    }
}
