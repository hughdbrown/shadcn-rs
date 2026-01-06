//! Breadcrumb component showcase page

use yew::prelude::*;
use shadcn_rs::{Breadcrumb, BreadcrumbList, BreadcrumbItem, BreadcrumbLink, BreadcrumbPage as BreadcrumbPageComponent, BreadcrumbSeparator};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(BreadcrumbShowcasePage)]
pub fn breadcrumb_showcase_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic breadcrumb navigation.",
            demo: html! {
                <Breadcrumb>
                    <BreadcrumbList>
                        <BreadcrumbItem>
                            <BreadcrumbLink href="/">{ "Home" }</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbLink href="/components">{ "Components" }</BreadcrumbLink>
                        </BreadcrumbItem>
                        <BreadcrumbSeparator />
                        <BreadcrumbItem>
                            <BreadcrumbPageComponent>{ "Breadcrumb" }</BreadcrumbPageComponent>
                        </BreadcrumbItem>
                    </BreadcrumbList>
                </Breadcrumb>
            },
            code: r##"<Breadcrumb>
    <BreadcrumbList>
        <BreadcrumbItem>
            <BreadcrumbLink href="/">{ "Home" }</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
            <BreadcrumbPage>{ "Current" }</BreadcrumbPage>
        </BreadcrumbItem>
    </BreadcrumbList>
</Breadcrumb>"##,
        },
    ];

    let props = vec![
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Breadcrumb content" },
    ];

    html! { <ComponentPage name="Breadcrumb" description="Displays the path to the current resource." {examples} {props} /> }
}
