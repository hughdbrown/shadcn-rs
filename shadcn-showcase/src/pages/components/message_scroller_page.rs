use shadcn_rs::*;
use yew::prelude::*;

use crate::components::{ComponentPage, Example};

#[function_component(MessageScrollerPage)]
pub fn message_scroller_page() -> Html {
    let count = use_state(|| 12usize);
    let append = {
        let count = count.clone();
        Callback::from(move |_: MouseEvent| count.set(*count + 1))
    };
    let examples = vec![Example {
        title: "Follow a conversation",
        description: "Add messages at the bottom, scroll back to read earlier messages, then use the arrow to jump to the latest.",
        demo: html! {
            <div class="demo-stack">
                <Button onclick={append}>{ "Add message" }</Button>
                <MessageScrollerProvider>
                    <MessageScroller>
                        <MessageScrollerViewport id="conversation-viewport" class="demo-message-viewport">
                            <MessageScrollerContent>
                                { for (1..=*count).map(|index| html! {
                                    <MessageScrollerItem key={index} message_id={format!("message-{index}")}>
                                        <Bubble><BubbleContent><>{ format!("Message {index} — a little more of the conversation.") }</></BubbleContent></Bubble>
                                    </MessageScrollerItem>
                                }) }
                            </MessageScrollerContent>
                        </MessageScrollerViewport>
                        <MessageScrollerButton />
                    </MessageScroller>
                </MessageScrollerProvider>
            </div>
        },
        code: r#"<MessageScrollerProvider>
    <MessageScroller>
        <MessageScrollerViewport>
            <MessageScrollerContent>
                <MessageScrollerItem><Bubble><BubbleContent>{ "Hello!" }</BubbleContent></Bubble></MessageScrollerItem>
            </MessageScrollerContent>
        </MessageScrollerViewport>
        <MessageScrollerButton />
    </MessageScroller>
</MessageScrollerProvider>"#,
    }];
    html! { <ComponentPage name="Message Scroller" description="A scrollable conversation with a jump-to-latest control." {examples} /> }
}
