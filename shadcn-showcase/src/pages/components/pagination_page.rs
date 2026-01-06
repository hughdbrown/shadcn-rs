//! Pagination component showcase page

use yew::prelude::*;
use shadcn_rs::{Pagination, PaginationContent, PaginationItem, PaginationLink, PaginationPrevious, PaginationNext, PaginationEllipsis};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(PaginationPage)]
pub fn pagination_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic pagination component.",
            demo: html! {
                <Pagination>
                    <PaginationContent>
                        <PaginationItem>
                            <PaginationPrevious href="#" />
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationLink href="#">{ "1" }</PaginationLink>
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationLink href="#" is_active={true}>{ "2" }</PaginationLink>
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationLink href="#">{ "3" }</PaginationLink>
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationEllipsis />
                        </PaginationItem>
                        <PaginationItem>
                            <PaginationNext href="#" />
                        </PaginationItem>
                    </PaginationContent>
                </Pagination>
            },
            code: r##"<Pagination>
    <PaginationContent>
        <PaginationItem>
            <PaginationPrevious href="#" />
        </PaginationItem>
        <PaginationItem>
            <PaginationLink href="#" is_active={true}>{ "1" }</PaginationLink>
        </PaginationItem>
        <PaginationItem>
            <PaginationNext href="#" />
        </PaginationItem>
    </PaginationContent>
</Pagination>"##,
        },
    ];

    let props = vec![
        PropDoc { name: "href", prop_type: "AttrValue", default: "-", description: "Link URL" },
        PropDoc { name: "is_active", prop_type: "bool", default: "false", description: "Active state for PaginationLink" },
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Pagination content" },
    ];

    html! { <ComponentPage name="Pagination" description="Pagination with page navigation, next and previous links." {examples} {props} /> }
}
