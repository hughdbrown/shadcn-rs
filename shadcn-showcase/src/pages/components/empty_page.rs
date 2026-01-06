//! Empty state component showcase page

use yew::prelude::*;
use shadcn_rs::{Empty, Button};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(EmptyPage)]
pub fn empty_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "Empty state with title and description.",
            demo: html! {
                <Empty>
                    <div class="text-center py-10">
                        <h3 class="text-lg font-medium">{ "No results found" }</h3>
                        <p class="text-sm text-muted-foreground mt-2">
                            { "Try adjusting your search or filters." }
                        </p>
                    </div>
                </Empty>
            },
            code: r#"<Empty>
    <h3>{ "No results found" }</h3>
    <p>{ "Try adjusting your search." }</p>
</Empty>"#,
        },
        Example {
            title: "With Action",
            description: "Empty state with a call-to-action button.",
            demo: html! {
                <Empty>
                    <div class="text-center py-10">
                        <h3 class="text-lg font-medium">{ "No projects yet" }</h3>
                        <p class="text-sm text-muted-foreground mt-2 mb-4">
                            { "Get started by creating your first project." }
                        </p>
                        <Button>{ "Create Project" }</Button>
                    </div>
                </Empty>
            },
            code: r#"<Empty>
    <h3>{ "No projects yet" }</h3>
    <Button>{ "Create Project" }</Button>
</Empty>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Empty state content" },
    ];

    html! { <ComponentPage name="Empty" description="Displays an empty state placeholder." {examples} {props} /> }
}
