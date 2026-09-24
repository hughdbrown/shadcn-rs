//! Focus management helpers for overlays and composite widgets.

use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, KeyboardEvent};

fn is_focusable(element: &Element) -> bool {
    if element.has_attribute("disabled")
        || element.has_attribute("hidden")
        || element.get_attribute("aria-hidden").as_deref() == Some("true")
    {
        return false;
    }

    // An explicit tabindex wins: `-1` means focusable by script only, which
    // must not be part of the Tab cycle.
    if let Some(tabindex) = element
        .get_attribute("tabindex")
        .and_then(|value| value.parse::<i32>().ok())
    {
        return tabindex >= 0;
    }

    match element.tag_name().to_ascii_lowercase().as_str() {
        "input" => element.get_attribute("type").as_deref() != Some("hidden"),
        "button" | "select" | "textarea" => true,
        "a" => element.has_attribute("href"),
        _ => false,
    }
}

/// Returns the currently focused document element, if any.
pub fn active_element() -> Option<Element> {
    gloo::utils::document().active_element()
}

/// Moves focus to the provided element when it supports focus.
pub fn focus_element(element: &Element) {
    if let Some(html_element) = element.dyn_ref::<HtmlElement>() {
        let _ = html_element.focus();
    }
}

/// Collects focusable descendants within a root element.
pub fn collect_focusable(root: &Element) -> Vec<Element> {
    let Ok(node_list) =
        root.query_selector_all("a[href], button, input, select, textarea, [tabindex]")
    else {
        return Vec::new();
    };

    let mut elements = Vec::new();
    for index in 0..node_list.length() {
        if let Some(node) = node_list.item(index)
            && let Some(element) = node.dyn_ref::<Element>()
            && is_focusable(element)
        {
            elements.push(element.clone());
        }
    }
    elements
}

/// Focuses the first focusable descendant, falling back to the root element.
pub fn focus_first_within(root: &Element) {
    if let Some(first) = collect_focusable(root).first() {
        focus_element(first);
    } else {
        focus_element(root);
    }
}

/// Keeps `Tab` navigation contained within the provided root element.
pub fn trap_tab_navigation(root: &Element, event: &KeyboardEvent) {
    if event.key() != "Tab" {
        return;
    }

    let focusable = collect_focusable(root);
    if focusable.is_empty() {
        event.prevent_default();
        focus_element(root);
        return;
    }

    let current = active_element();
    let current_index = current
        .as_ref()
        .and_then(|active| focusable.iter().position(|item| item == active));

    let next_index = if event.shift_key() {
        match current_index {
            Some(0) | None => focusable.len() - 1,
            Some(index) => index.saturating_sub(1),
        }
    } else {
        match current_index {
            Some(index) if index + 1 < focusable.len() => index + 1,
            _ => 0,
        }
    };

    event.prevent_default();
    focus_element(&focusable[next_index]);
}
