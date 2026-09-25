#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;

use shadcn_rs::{Tabs, TabsContent, TabsList, TabsTrigger};

mod utils;

use utils::{active_id, attr, click, focus, keydown, mount_root, query, settle, text};

wasm_bindgen_test_configure!(run_in_browser);

#[function_component(TabsHarness)]
fn tabs_harness() -> Html {
    let changes = use_state(Vec::<String>::new);
    let on_value_change = {
        let changes = changes.clone();
        Callback::from(move |value: String| {
            let mut next = (*changes).clone();
            next.push(value);
            changes.set(next);
        })
    };

    html! {
        <div>
            <div id="tabs-changes">{ changes.join(",") }</div>
            <Tabs default_value="one" on_value_change={Some(on_value_change)}>
                <TabsList>
                    <TabsTrigger value="one"><span id="tab-one-label">{ "One" }</span></TabsTrigger>
                    <TabsTrigger value="two">{ "Two" }</TabsTrigger>
                    <TabsTrigger value="three">{ "Three" }</TabsTrigger>
                </TabsList>
                <TabsContent value="one">{ "Panel one" }</TabsContent>
                <TabsContent value="two">{ "Panel two" }</TabsContent>
                <TabsContent value="three">{ "Panel three" }</TabsContent>
            </Tabs>
        </div>
    }
}

fn tab(value: &str) -> String {
    format!("#tabs-test [role='tab'][data-value='{value}']")
}

fn panel(value: &str) -> String {
    format!("#tabs-test [role='tabpanel'][data-value='{value}']")
}

#[test]
async fn tabs_select_hide_and_keyboard() {
    let root = mount_root("tabs-test");
    let _app = yew::Renderer::<TabsHarness>::with_root(root).render();
    settle().await;

    // Initial state: default tab active, others hidden and out of Tab order.
    assert_eq!(attr(&tab("one"), "aria-selected"), "true");
    assert_eq!(attr(&tab("one"), "tabindex"), "0");
    assert_eq!(attr(&tab("two"), "aria-selected"), "false");
    assert_eq!(attr(&tab("two"), "tabindex"), "-1");
    assert!(!query(&panel("one")).has_attribute("hidden"));
    assert!(query(&panel("two")).has_attribute("hidden"));
    assert_eq!(attr(&panel("two"), "data-state"), "inactive");

    // Trigger and panel are linked both ways.
    let controls = attr(&tab("two"), "aria-controls");
    assert_eq!(attr(&panel("two"), "id"), controls);
    assert_eq!(
        attr(&panel("two"), "aria-labelledby"),
        attr(&tab("two"), "id")
    );

    click(&tab("two"));
    settle().await;
    assert_eq!(attr(&tab("two"), "aria-selected"), "true");
    assert!(query(&panel("one")).has_attribute("hidden"));
    assert!(!query(&panel("two")).has_attribute("hidden"));
    assert_eq!(text("#tabs-test #tabs-changes"), "two");

    // Keyboard: ArrowRight moves focus and activates, wrapping at the end.
    focus(&tab("two"));
    keydown(&tab("two"), "ArrowRight", false);
    settle().await;
    assert_eq!(attr(&tab("three"), "aria-selected"), "true");
    assert_eq!(active_id(), attr(&tab("three"), "id"));

    keydown(&tab("three"), "ArrowRight", false);
    settle().await;
    assert_eq!(attr(&tab("one"), "aria-selected"), "true");

    keydown(&tab("one"), "End", false);
    settle().await;
    assert_eq!(attr(&tab("three"), "aria-selected"), "true");

    keydown(&tab("three"), "Home", false);
    settle().await;
    assert_eq!(attr(&tab("one"), "aria-selected"), "true");
    assert!(!query(&panel("one")).has_attribute("hidden"));
    assert_eq!(text("#tabs-test #tabs-changes"), "two,three,one,three,one");
}
