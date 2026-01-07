//! Combobox component showcase page

use shadcn_rs::{
    Combobox, ComboboxContent, ComboboxEmpty, ComboboxGroup, ComboboxInput, ComboboxItem,
    ComboboxTrigger,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ComboboxPage)]
pub fn combobox_page() -> Html {
    let examples = vec![Example {
        title: "Default",
        description: "A searchable select component.",
        demo: html! {
            <Combobox>
                <ComboboxTrigger class="w-[200px]">
                    { "Select framework..." }
                </ComboboxTrigger>
                <ComboboxContent>
                    <ComboboxInput placeholder="Search framework..." />
                    <ComboboxEmpty>{ "No framework found." }</ComboboxEmpty>
                    <ComboboxGroup>
                        <ComboboxItem value="next">{ "Next.js" }</ComboboxItem>
                        <ComboboxItem value="sveltekit">{ "SvelteKit" }</ComboboxItem>
                        <ComboboxItem value="nuxt">{ "Nuxt" }</ComboboxItem>
                        <ComboboxItem value="remix">{ "Remix" }</ComboboxItem>
                        <ComboboxItem value="astro">{ "Astro" }</ComboboxItem>
                    </ComboboxGroup>
                </ComboboxContent>
            </Combobox>
        },
        code: r#"<Combobox>
    <ComboboxTrigger>{ "Select..." }</ComboboxTrigger>
    <ComboboxContent>
        <ComboboxInput placeholder="Search..." />
        <ComboboxEmpty>{ "No results." }</ComboboxEmpty>
        <ComboboxGroup>
            <ComboboxItem value="item1">{ "Item 1" }</ComboboxItem>
            <ComboboxItem value="item2">{ "Item 2" }</ComboboxItem>
        </ComboboxGroup>
    </ComboboxContent>
</Combobox>"#,
    }];

    let props = vec![
        PropDoc {
            name: "value",
            prop_type: "Option<String>",
            default: "-",
            description: "Controlled selected value",
        },
        PropDoc {
            name: "on_value_change",
            prop_type: "Callback<String>",
            default: "-",
            description: "Value change handler",
        },
        PropDoc {
            name: "placeholder",
            prop_type: "Option<String>",
            default: "-",
            description: "Placeholder text",
        },
    ];

    html! { <ComponentPage name="Combobox" description="Autocomplete input with command palette." {examples} {props} /> }
}
