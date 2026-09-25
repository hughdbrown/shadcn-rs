use shadcn_rs::*;
use yew::prelude::*;

use crate::components::{ComponentPage, Example};

#[function_component(DirectionSample)]
fn direction_sample() -> Html {
    let dir = use_direction();
    html! {
        <div class="demo-stack">
            <p>{ format!("Current direction: {}", dir.as_str()) }</p>
            <p lang="ar">{ "مرحباً بكم في مكتبة المكونات" }</p>
            <div class="demo-row"><Badge>{ "First" }</Badge><Badge>{ "Second" }</Badge><Badge>{ "Third" }</Badge></div>
            <Input aria_label="Direction sample input" placeholder="Type here…" />
        </div>
    }
}

#[function_component(DirectionPage)]
pub fn direction_page() -> Html {
    let rtl = use_state(|| false);
    let toggle = {
        let rtl = rtl.clone();
        Callback::from(move |_: MouseEvent| rtl.set(!*rtl))
    };
    let examples = vec![Example {
        title: "Switch reading direction",
        description: "The provider updates HTML direction and the use_direction hook for its descendants.",
        demo: html! {
            <div class="demo-stack">
                <Button onclick={toggle}><>{ if *rtl { "Use left-to-right" } else { "Use right-to-left" } }</></Button>
                <DirectionProvider dir={if *rtl { Direction::Rtl } else { Direction::Ltr }}><DirectionSample /></DirectionProvider>
            </div>
        },
        code: r#"<DirectionProvider dir={Direction::Rtl}>
    <Input aria_label="Name" placeholder="الاسم" />
</DirectionProvider>"#,
    }];
    html! { <ComponentPage name="Direction" import_code="use shadcn_rs::{Direction, DirectionProvider, use_direction};" description="Set text direction for a subtree and read it through context." {examples} /> }
}
