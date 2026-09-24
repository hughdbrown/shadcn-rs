use std::time::Duration;

use wasm_bindgen::JsCast;
use web_sys::{Element, Event, EventInit, HtmlInputElement, KeyboardEvent, KeyboardEventInit};
use yew::platform::time::sleep;

pub async fn settle() {
    sleep(Duration::ZERO).await;
}

pub fn mount_root(id: &str) -> Element {
    if let Some(existing) = gloo::utils::document().get_element_by_id(id) {
        existing.remove();
    }

    let root = gloo::utils::document()
        .create_element("div")
        .expect("failed to create mount root");
    root.set_id(id);
    gloo::utils::body()
        .append_child(&root)
        .expect("failed to mount root to body");
    root
}

pub fn query(selector: &str) -> Element {
    gloo::utils::document()
        .query_selector(selector)
        .expect("query selector failed")
        .unwrap_or_else(|| panic!("missing selector: {selector}"))
}

pub fn click(selector: &str) {
    query(selector)
        .dyn_into::<web_sys::HtmlElement>()
        .expect("selector is not an HtmlElement")
        .click();
}

pub fn click_nth(selector: &str, index: u32) {
    gloo::utils::document()
        .query_selector_all(selector)
        .expect("query selector failed")
        .item(index)
        .unwrap_or_else(|| panic!("missing match {index} for selector: {selector}"))
        .dyn_into::<web_sys::HtmlElement>()
        .expect("selector is not an HtmlElement")
        .click();
}

pub fn focus(selector: &str) {
    query(selector)
        .dyn_into::<web_sys::HtmlElement>()
        .expect("selector is not an HtmlElement")
        .focus()
        .expect("failed to focus element");
}

pub fn input(selector: &str, value: &str) {
    let input = query(selector)
        .dyn_into::<HtmlInputElement>()
        .expect("selector is not an HtmlInputElement");
    input.set_value(value);

    let event_init = EventInit::new();
    event_init.set_bubbles(true);
    event_init.set_cancelable(true);
    let event = Event::new_with_event_init_dict("input", &event_init)
        .expect("failed to create input event");
    input
        .dispatch_event(&event)
        .expect("failed to dispatch input event");
}

pub fn keydown(selector: &str, key: &str, shift_key: bool) {
    let target = query(selector);
    let event_init = KeyboardEventInit::new();
    event_init.set_key(key);
    event_init.set_shift_key(shift_key);
    event_init.set_bubbles(true);
    event_init.set_cancelable(true);
    let event = KeyboardEvent::new_with_keyboard_event_init_dict("keydown", &event_init)
        .expect("failed to create keyboard event");
    target
        .dispatch_event(&event)
        .expect("failed to dispatch keyboard event");
}

pub fn text(selector: &str) -> String {
    query(selector)
        .text_content()
        .unwrap_or_default()
        .trim()
        .to_string()
}

pub fn active_id() -> String {
    gloo::utils::document()
        .active_element()
        .expect("no active element")
        .id()
}
