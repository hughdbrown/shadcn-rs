#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;

use shadcn_rs::{
    Menubar, MenubarContent, MenubarItem, MenubarMenu, MenubarRadioGroup, MenubarRadioItem,
    MenubarTrigger, Tabs, TabsContent, TabsList, TabsTrigger,
};

mod utils;

use utils::{
    active_id, attr, click, count, exists, focus, keydown, mount_root, mousedown_body, query,
    settle, text,
};

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

#[function_component(MenubarHarness)]
fn menubar_harness() -> Html {
    let clicked = use_state(String::new);
    let profile = use_state(|| AttrValue::from("a"));
    let on_new = {
        let clicked = clicked.clone();
        Callback::from(move |_: MouseEvent| clicked.set("new".into()))
    };
    let on_profile = {
        let profile = profile.clone();
        Callback::from(move |value: AttrValue| profile.set(value))
    };

    html! {
        <div>
            <div id="menubar-clicked">{ (*clicked).clone() }</div>
            <div id="menubar-profile">{ profile.to_string() }</div>
            <Menubar>
                <MenubarMenu value="file">
                    <MenubarTrigger>{ "File" }</MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem onclick={on_new}>{ "New" }</MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <MenubarMenu value="profiles">
                    <MenubarTrigger>{ "Profiles" }</MenubarTrigger>
                    <MenubarContent>
                        <MenubarRadioGroup value={(*profile).clone()} onchange={on_profile}>
                            <MenubarRadioItem value="a">{ "A" }</MenubarRadioItem>
                            <MenubarRadioItem value="b">{ "B" }</MenubarRadioItem>
                        </MenubarRadioGroup>
                    </MenubarContent>
                </MenubarMenu>
            </Menubar>
        </div>
    }
}

const FILE_TRIGGER: &str = "#menubar-test .menubar-trigger[data-menu='file']";
const PROFILES_TRIGGER: &str = "#menubar-test .menubar-trigger[data-menu='profiles']";

#[test]
async fn menubar_open_close_and_radio() {
    let root = mount_root("menubar-test");
    let _app = yew::Renderer::<MenubarHarness>::with_root(root).render();
    settle().await;

    // Closed by default: no content rendered.
    assert_eq!(count("#menubar-test .menubar-content"), 0);
    assert_eq!(attr(FILE_TRIGGER, "aria-expanded"), "false");

    // Click toggles open.
    click(FILE_TRIGGER);
    settle().await;
    assert_eq!(attr(FILE_TRIGGER, "aria-expanded"), "true");
    assert_eq!(count("#menubar-test .menubar-content"), 1);

    // Opening another menu closes the first (one open at a time).
    click(PROFILES_TRIGGER);
    settle().await;
    assert_eq!(attr(FILE_TRIGGER, "aria-expanded"), "false");
    assert_eq!(attr(PROFILES_TRIGGER, "aria-expanded"), "true");
    assert_eq!(count("#menubar-test .menubar-content"), 1);

    // Radio items read the group value and report changes.
    let radio = |value: &str| format!("#menubar-test .menubar-radio-item[data-value='{value}']");
    assert_eq!(attr(&radio("a"), "aria-checked"), "true");
    click(&radio("b"));
    settle().await;
    assert_eq!(text("#menubar-test #menubar-profile"), "b");
    assert_eq!(attr(&radio("b"), "aria-checked"), "true");
    assert_eq!(attr(&radio("a"), "aria-checked"), "false");

    // Click toggles closed.
    click(PROFILES_TRIGGER);
    settle().await;
    assert_eq!(count("#menubar-test .menubar-content"), 0);

    // Escape closes and returns focus to the trigger.
    click(FILE_TRIGGER);
    settle().await;
    assert!(exists("#menubar-test .menubar-content"));
    keydown("#menubar-test .menubar-item", "Escape", false);
    settle().await;
    assert_eq!(count("#menubar-test .menubar-content"), 0);
    assert_eq!(active_id(), attr(FILE_TRIGGER, "id"));

    // Click outside closes.
    click(FILE_TRIGGER);
    settle().await;
    assert!(exists("#menubar-test .menubar-content"));
    mousedown_body();
    settle().await;
    assert_eq!(count("#menubar-test .menubar-content"), 0);

    // Selecting an item runs its handler and closes the menu.
    click(FILE_TRIGGER);
    settle().await;
    click("#menubar-test .menubar-item");
    settle().await;
    assert_eq!(text("#menubar-test #menubar-clicked"), "new");
    assert_eq!(count("#menubar-test .menubar-content"), 0);
    assert_eq!(attr(FILE_TRIGGER, "aria-expanded"), "false");
}
