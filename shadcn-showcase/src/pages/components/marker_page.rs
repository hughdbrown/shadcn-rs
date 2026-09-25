use shadcn_rs::*;
use yew::prelude::*;

use crate::components::{ComponentPage, Example};

#[function_component(MarkerPage)]
pub fn marker_page() -> Html {
    let busy = use_state(|| false);
    let toggle = {
        let busy = busy.clone();
        Callback::from(move |_: MouseEvent| busy.set(!*busy))
    };
    let examples = vec![Example {
        title: "Activity and milestones",
        description: "Toggle the processing state and compare all four marker variants.",
        demo: html! {
            <div class="demo-stack">
                <Button onclick={toggle}><>{ if *busy { "Finish processing" } else { "Start processing" } }</></Button>
                <Marker shimmer={*busy} role="status">
                    <MarkerIcon><span>{ if *busy { "◌" } else { "✓" } }</span></MarkerIcon>
                    <MarkerContent><>{ if *busy { "Processing your files…" } else { "All files ready" } }</></MarkerContent>
                </Marker>
                <Marker variant={MarkerVariant::Subtle}><MarkerContent>{ "Saved just now" }</MarkerContent></Marker>
                <Marker variant={MarkerVariant::Border}><MarkerContent>{ "A teammate joined the conversation" }</MarkerContent></Marker>
                <Marker variant={MarkerVariant::Separator}><MarkerContent>{ "Today" }</MarkerContent></Marker>
            </div>
        },
        code: r#"<Marker shimmer={busy} role="status">
    <MarkerIcon><span>{ "✓" }</span></MarkerIcon>
    <MarkerContent>{ "All files ready" }</MarkerContent>
</Marker>"#,
    }];
    html! { <ComponentPage name="Marker" description="Status updates, system notes, and labeled separators." {examples} /> }
}
