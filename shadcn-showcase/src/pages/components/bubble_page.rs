use shadcn_rs::*;
use yew::prelude::*;

use crate::components::{ComponentPage, Example};

#[function_component(BubblePage)]
pub fn bubble_page() -> Html {
    let liked = use_state(|| false);
    let react = {
        let liked = liked.clone();
        Callback::from(move |_: MouseEvent| liked.set(!*liked))
    };
    let examples = vec![Example {
        title: "Conversation and reactions",
        description: "Incoming and outgoing groups, every surface variant, and a reaction you can toggle.",
        demo: html! {
            <div class="demo-stack">
                <BubbleGroup>
                    <Bubble>
                        <BubbleContent>{ "Your workspace is ready. What would you like to build?" }</BubbleContent>
                        <BubbleReactions aria_label="Message reactions">
                            <button type="button" class="btn variant-outline" aria-pressed={liked.to_string()} onclick={react}>
                                { if *liked { "👍 1" } else { "👍 React" } }
                            </button>
                        </BubbleReactions>
                    </Bubble>
                </BubbleGroup>
                <BubbleGroup align={BubbleAlign::End}>
                    <Bubble align={BubbleAlign::End} variant={BubbleVariant::Tinted}>
                        <BubbleContent>{ "An app that feels great to use." }</BubbleContent>
                    </Bubble>
                </BubbleGroup>
                { for [BubbleVariant::Secondary, BubbleVariant::Muted, BubbleVariant::Outline, BubbleVariant::Ghost, BubbleVariant::Destructive].into_iter().map(|variant| html! {
                    <Bubble {variant}><BubbleContent><>{ format!("{variant:?} bubble") }</></BubbleContent></Bubble>
                }) }
            </div>
        },
        code: r#"<Bubble align={BubbleAlign::End} variant={BubbleVariant::Tinted}>
    <BubbleContent>{ "An app that feels great to use." }</BubbleContent>
</Bubble>"#,
    }];
    html! { <ComponentPage name="Bubble" description="Conversational surfaces with alignment, grouping, and reactions." {examples} /> }
}
