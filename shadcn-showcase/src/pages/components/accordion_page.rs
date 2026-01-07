//! Accordion component showcase page

use shadcn_rs::{Accordion, AccordionContent, AccordionItem, AccordionTrigger, AccordionType};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(AccordionPage)]
pub fn accordion_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic accordion.",
            demo: html! {
                <Accordion accordion_type={AccordionType::Single} collapsible={true} class="w-full">
                    <AccordionItem value="item-1">
                        <AccordionTrigger>{ "Is it accessible?" }</AccordionTrigger>
                        <AccordionContent>
                            { "Yes. It adheres to the WAI-ARIA design pattern." }
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="item-2">
                        <AccordionTrigger>{ "Is it styled?" }</AccordionTrigger>
                        <AccordionContent>
                            { "Yes. It comes with default styles that match shadcn/ui." }
                        </AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="item-3">
                        <AccordionTrigger>{ "Is it animated?" }</AccordionTrigger>
                        <AccordionContent>
                            { "Yes. It's animated by default with CSS transitions." }
                        </AccordionContent>
                    </AccordionItem>
                </Accordion>
            },
            code: r#"<Accordion accordion_type="single" collapsible={true}>
    <AccordionItem value="item-1">
        <AccordionTrigger>{ "Question" }</AccordionTrigger>
        <AccordionContent>{ "Answer" }</AccordionContent>
    </AccordionItem>
</Accordion>"#,
        },
        Example {
            title: "Multiple",
            description: "Accordion allowing multiple items open.",
            demo: html! {
                <Accordion accordion_type={AccordionType::Multiple} class="w-full">
                    <AccordionItem value="item-1">
                        <AccordionTrigger>{ "Section 1" }</AccordionTrigger>
                        <AccordionContent>{ "Content for section 1." }</AccordionContent>
                    </AccordionItem>
                    <AccordionItem value="item-2">
                        <AccordionTrigger>{ "Section 2" }</AccordionTrigger>
                        <AccordionContent>{ "Content for section 2." }</AccordionContent>
                    </AccordionItem>
                </Accordion>
            },
            code: r#"<Accordion accordion_type="multiple">
    ...
</Accordion>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "accordion_type",
            prop_type: "&str",
            default: "\"single\"",
            description: "Selection type (single/multiple)",
        },
        PropDoc {
            name: "collapsible",
            prop_type: "bool",
            default: "false",
            description: "Allow collapsing all items",
        },
        PropDoc {
            name: "value",
            prop_type: "Option<String>",
            default: "-",
            description: "Controlled open item(s)",
        },
        PropDoc {
            name: "default_value",
            prop_type: "Option<String>",
            default: "-",
            description: "Default open item(s)",
        },
    ];

    html! { <ComponentPage name="Accordion" description="A vertically stacked set of interactive headings." {examples} {props} /> }
}
