use shadcn_rs::*;
use yew::prelude::*;

use crate::components::{ComponentPage, Example};

#[function_component(MessagePage)]
pub fn message_page() -> Html {
    let draft = use_state(String::new);
    let messages = use_state(|| vec!["Let's try the new components.".to_string()]);
    let oninput = {
        let draft = draft.clone();
        Callback::from(move |event: InputEvent| {
            draft.set(
                event
                    .target_unchecked_into::<web_sys::HtmlInputElement>()
                    .value(),
            );
        })
    };
    let send = {
        let draft = draft.clone();
        let messages = messages.clone();
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();
            if !draft.trim().is_empty() {
                let mut next = (*messages).clone();
                next.push(draft.trim().to_string());
                messages.set(next);
                draft.set(String::new());
            }
        })
    };
    let examples = vec![Example {
        title: "A local conversation",
        description: "Send a message to exercise the composer, avatars, headers, bubbles, and delivery footer. Messages stay in this page.",
        demo: html! {
            <div class="demo-stack">
                <Message>
                    <MessageAvatar><Avatar><AvatarFallback>{ "AI" }</AvatarFallback></Avatar></MessageAvatar>
                    <MessageContent>
                        <MessageHeader>{ "Assistant" }</MessageHeader>
                        <Bubble><BubbleContent>{ "Welcome! Try writing a message below." }</BubbleContent></Bubble>
                        <MessageFooter>{ "Just now" }</MessageFooter>
                    </MessageContent>
                </Message>
                { for messages.iter().enumerate().map(|(index, text)| html! {
                    <Message key={index} align={MessageAlign::End}>
                        <MessageAvatar><Avatar><AvatarFallback>{ "YO" }</AvatarFallback></Avatar></MessageAvatar>
                        <MessageContent>
                            <MessageHeader>{ "You" }</MessageHeader>
                            <Bubble variant={BubbleVariant::Tinted}><BubbleContent><>{ text.clone() }</></BubbleContent></Bubble>
                            <MessageFooter>{ "Delivered locally" }</MessageFooter>
                        </MessageContent>
                    </Message>
                }) }
                <form class="demo-row" onsubmit={send}>
                    <Input aria_label="Message text" placeholder="Write a message…" value={(*draft).clone()} {oninput} />
                    <Button r#type="submit" disabled={draft.trim().is_empty()}>{ "Send" }</Button>
                </form>
            </div>
        },
        code: r#"<Message align={MessageAlign::End}>
    <MessageAvatar><Avatar><AvatarFallback>{ "YO" }</AvatarFallback></Avatar></MessageAvatar>
    <MessageContent>
        <MessageHeader>{ "You" }</MessageHeader>
        <Bubble><BubbleContent>{ "Hello!" }</BubbleContent></Bubble>
        <MessageFooter>{ "Delivered locally" }</MessageFooter>
    </MessageContent>
</Message>"#,
    }];
    html! { <ComponentPage name="Message" description="Compose complete conversation turns with sender and delivery details." {examples} /> }
}
