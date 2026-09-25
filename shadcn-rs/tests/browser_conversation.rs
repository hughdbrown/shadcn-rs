#![cfg(target_arch = "wasm32")]

use std::{cell::Cell, rc::Rc};

use shadcn_rs::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;

#[allow(dead_code)]
mod utils;

use utils::{click, mount_root, query, settle, text};

wasm_bindgen_test_configure!(run_in_browser);

#[function_component(ScrollControls)]
fn scroll_controls() -> Html {
    let context = use_message_scroller();
    html! {
        <>
            <button id="scroll-top" onclick={Callback::from(move |_: MouseEvent| context.scroll_to_top.emit(()))}>{ "Top" }</button>
            <span id="at-bottom">{ context.is_at_bottom.to_string() }</span>
        </>
    }
}

#[function_component(ScrollerHarness)]
fn scroller_harness() -> Html {
    let count = use_state(|| 10usize);
    let add = {
        let count = count.clone();
        Callback::from(move |_: MouseEvent| count.set(*count + 1))
    };
    html! {
        <MessageScrollerProvider>
            <style>{ "#scroll-test-viewport { height: 120px; overflow-y: auto; overflow-anchor: none; }" }</style>
            <ScrollControls />
            <button id="scroll-add" onclick={add}>{ "Add" }</button>
            <MessageScroller>
                <MessageScrollerViewport id="scroll-test-viewport">
                    <MessageScrollerContent>
                        { for (0..*count).map(|index| html! {
                            <MessageScrollerItem key={index}><div style="height: 40px">{ index }</div></MessageScrollerItem>
                        }) }
                    </MessageScrollerContent>
                </MessageScrollerViewport>
                <MessageScrollerButton />
            </MessageScroller>
        </MessageScrollerProvider>
    }
}

fn viewport() -> web_sys::HtmlElement {
    query("#scroll-test-viewport").unchecked_into()
}

fn assert_at_bottom() {
    let el = viewport();
    assert!(
        el.scroll_height() > el.client_height(),
        "fixture must overflow"
    );
    assert!((el.scroll_height() - el.client_height() - el.scroll_top()).abs() <= 4);
}

#[test]
async fn scroller_follows_new_messages_and_preserves_reading_position() {
    let app = yew::Renderer::<ScrollerHarness>::with_root(mount_root("scroller-test")).render();
    settle().await;
    assert_at_bottom();

    click("#scroll-add");
    settle().await;
    assert_at_bottom();

    // A user scrolls upwards: notify the component through the actual scroll listener.
    viewport().set_scroll_top(40);
    viewport()
        .dispatch_event(&web_sys::Event::new("scroll").unwrap())
        .unwrap();
    settle().await;
    assert_eq!(text("#at-bottom"), "false");
    click("#scroll-add");
    settle().await;
    assert_eq!(viewport().scroll_top(), 40);

    click("#scroller-test .message-scroller-button");
    settle().await;
    assert_at_bottom();
    assert_eq!(text("#at-bottom"), "true");

    click("#scroll-top");
    settle().await;
    assert_eq!(viewport().scroll_top(), 0);
    assert_eq!(text("#at-bottom"), "false");
    app.destroy();
    settle().await;
}

#[function_component(QuestionnaireHarness)]
fn questionnaire_harness() -> Html {
    let calls = use_state(|| 0usize);
    let counter = use_memo((), |_| Rc::new(Cell::new(0usize)));
    let on_submit = {
        let calls = calls.clone();
        Callback::from(move |()| {
            counter.set(counter.get() + 1);
            calls.set(counter.get());
        })
    };
    html! {
        <div id="questionnaire-test">
            <span id="submit-calls">{ *calls }</span>
            <Questionnaire total_steps={2} {on_submit}>
                <QuestionnaireProgress />
                <QuestionnaireItem step={0}><QuestionnaireTitle>{ "First" }</QuestionnaireTitle></QuestionnaireItem>
                <QuestionnaireItem step={1}><QuestionnaireTitle>{ "Last" }</QuestionnaireTitle></QuestionnaireItem>
                <QuestionnaireActions><QuestionnairePrevious /><QuestionnaireNext /><QuestionnaireSubmit /></QuestionnaireActions>
            </Questionnaire>
        </div>
    }
}

#[test]
async fn questionnaire_navigates_and_submits_once_per_click() {
    let app =
        yew::Renderer::<QuestionnaireHarness>::with_root(mount_root("questionnaire-root")).render();
    settle().await;
    assert_eq!(text("#questionnaire-test .questionnaire-title"), "First");
    click("#questionnaire-test .questionnaire-next");
    settle().await;
    assert_eq!(text("#questionnaire-test .questionnaire-title"), "Last");
    click("#questionnaire-test .questionnaire-previous");
    settle().await;
    assert_eq!(text("#questionnaire-test .questionnaire-title"), "First");
    click("#questionnaire-test .questionnaire-next");
    settle().await;
    click("#questionnaire-test .questionnaire-submit");
    settle().await;
    assert_eq!(text("#submit-calls"), "1");
    click("#questionnaire-test .questionnaire-submit");
    settle().await;
    assert_eq!(text("#submit-calls"), "2");
    app.destroy();
    settle().await;
}
