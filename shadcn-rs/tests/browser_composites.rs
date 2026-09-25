#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;

use shadcn_rs::{
    Combobox, ComboboxContent, ComboboxEmpty, ComboboxInput, ComboboxItem, ComboboxTrigger,
    Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, Menubar,
    MenubarContent, MenubarItem, MenubarMenu, MenubarRadioGroup, MenubarRadioItem, MenubarTrigger,
    NavigationMenu, NavigationMenuContent, NavigationMenuItem, NavigationMenuLink,
    NavigationMenuList, NavigationMenuTrigger, Tabs, TabsContent, TabsList, TabsTrigger,
};

mod utils;

use utils::{
    active_id, attr, click, click_nth, count, exists, focus, input, keydown, mount_root,
    mousedown_body, query, settle, text,
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

#[function_component(NavigationMenuHarness)]
fn navigation_menu_harness() -> Html {
    let open = use_state(|| None::<AttrValue>);
    let on_value_change = {
        let open = open.clone();
        Callback::from(move |value: Option<AttrValue>| open.set(value))
    };

    html! {
        <div>
            <div id="nav-open">{ open.as_deref().unwrap_or("none").to_string() }</div>
            <NavigationMenu value={(*open).clone()} on_value_change={Some(on_value_change)}>
                <NavigationMenuList>
                    <NavigationMenuItem value="docs">
                        <NavigationMenuTrigger>{ "Docs" }</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <NavigationMenuLink href="#intro">{ "Intro" }</NavigationMenuLink>
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                    <NavigationMenuItem value="blog">
                        <NavigationMenuTrigger>{ "Blog" }</NavigationMenuTrigger>
                        <NavigationMenuContent>
                            <span id="blog-content">{ "Posts" }</span>
                        </NavigationMenuContent>
                    </NavigationMenuItem>
                </NavigationMenuList>
            </NavigationMenu>
        </div>
    }
}

#[test]
async fn navigation_menu_shows_only_active_content() {
    let root = mount_root("nav-test");
    let _app = yew::Renderer::<NavigationMenuHarness>::with_root(root).render();
    settle().await;

    let docs = "#nav-test .navigation-menu-item[data-state] > .navigation-menu-trigger";
    assert_eq!(count("#nav-test .navigation-menu-content"), 0);
    assert_eq!(attr(docs, "aria-expanded"), "false");

    click_nth("#nav-test .navigation-menu-trigger", 0);
    settle().await;
    assert_eq!(text("#nav-test #nav-open"), "docs");
    assert_eq!(count("#nav-test .navigation-menu-content"), 1);
    assert_eq!(attr(docs, "aria-expanded"), "true");
    assert_eq!(
        attr(docs, "aria-controls"),
        attr("#nav-test .navigation-menu-content", "id")
    );

    // Switching items shows only the new item's content.
    click_nth("#nav-test .navigation-menu-trigger", 1);
    settle().await;
    assert_eq!(text("#nav-test #nav-open"), "blog");
    assert_eq!(count("#nav-test .navigation-menu-content"), 1);
    assert!(exists("#nav-test #blog-content"));
    assert_eq!(attr(docs, "aria-expanded"), "false");

    // Escape closes.
    keydown("#nav-test .navigation-menu-list", "Escape", false);
    settle().await;
    assert_eq!(text("#nav-test #nav-open"), "none");
    assert_eq!(count("#nav-test .navigation-menu-content"), 0);

    // Clicking a trigger twice toggles; clicking outside closes.
    click_nth("#nav-test .navigation-menu-trigger", 0);
    settle().await;
    click_nth("#nav-test .navigation-menu-trigger", 0);
    settle().await;
    assert_eq!(count("#nav-test .navigation-menu-content"), 0);
    click_nth("#nav-test .navigation-menu-trigger", 0);
    settle().await;
    mousedown_body();
    settle().await;
    assert_eq!(text("#nav-test #nav-open"), "none");
    assert_eq!(count("#nav-test .navigation-menu-content"), 0);
}

#[function_component(CommandHarness)]
fn command_harness() -> Html {
    html! {
        <Command>
            <CommandInput placeholder="Search" />
            <CommandList>
                <CommandEmpty>{ "No results found." }</CommandEmpty>
                <CommandGroup heading="Suggestions">
                    <CommandItem>{ "Calendar" }</CommandItem>
                    <CommandItem value="emoji">{ "Search Emoji" }</CommandItem>
                </CommandGroup>
            </CommandList>
        </Command>
    }
}

#[test]
async fn command_empty_only_without_matches() {
    let root = mount_root("command-test");
    let _app = yew::Renderer::<CommandHarness>::with_root(root).render();
    settle().await;
    settle().await;

    assert_eq!(count("#command-test .command-empty"), 0);

    // Items without a value match on their text.
    input("#command-test .command-input", "cal");
    settle().await;
    assert_eq!(count("#command-test .command-empty"), 0);
    assert_eq!(count("#command-test .command-item:not([hidden])"), 1);

    input("#command-test .command-input", "zzz");
    settle().await;
    assert_eq!(count("#command-test .command-item:not([hidden])"), 0);
    assert_eq!(count("#command-test .command-empty"), 1);

    input("#command-test .command-input", "");
    settle().await;
    assert_eq!(count("#command-test .command-empty"), 0);
    assert_eq!(count("#command-test .command-item:not([hidden])"), 2);
}

#[function_component(ComboboxHarness)]
fn combobox_harness() -> Html {
    let value = use_state(|| None::<AttrValue>);
    let forced_open = use_state(|| None::<bool>);
    let on_value_change = {
        let value = value.clone();
        Callback::from(move |next: AttrValue| value.set(Some(next)))
    };
    let open_externally = {
        let forced_open = forced_open.clone();
        Callback::from(move |_: MouseEvent| forced_open.set(Some(true)))
    };

    html! {
        <div>
            <div id="combobox-value">{ value.as_deref().unwrap_or("none").to_string() }</div>
            <button id="open-externally" type="button" onclick={open_externally}>{ "Open" }</button>
            <Combobox open={*forced_open} on_value_change={Some(on_value_change)}>
                <ComboboxTrigger>{ "Select framework..." }</ComboboxTrigger>
                <ComboboxContent>
                    <ComboboxInput placeholder="Search..." />
                    <ComboboxEmpty>{ "No framework found." }</ComboboxEmpty>
                    <ComboboxItem value="next">{ "Next.js" }</ComboboxItem>
                    <ComboboxItem value="astro">{ "Astro" }</ComboboxItem>
                </ComboboxContent>
            </Combobox>
        </div>
    }
}

#[test]
async fn combobox_filters_selects_and_closes() {
    let root = mount_root("combobox-test");
    let _app = yew::Renderer::<ComboboxHarness>::with_root(root).render();
    settle().await;

    // A later change to `open` is respected.
    assert_eq!(count("#combobox-test .combobox-content"), 0);
    click("#combobox-test #open-externally");
    settle().await;
    settle().await;
    assert_eq!(count("#combobox-test .combobox-content"), 1);
    assert_eq!(count("#combobox-test .combobox-empty"), 0);

    input("#combobox-test .combobox-input", "zzz");
    settle().await;
    assert_eq!(count("#combobox-test .combobox-empty"), 1);

    input("#combobox-test .combobox-input", "ast");
    settle().await;
    assert_eq!(count("#combobox-test .combobox-empty"), 0);
    assert_eq!(count("#combobox-test .combobox-item:not([hidden])"), 1);

    // Selecting commits the value, fills the trigger, and closes.
    click("#combobox-test .combobox-item[data-value='astro']");
    settle().await;
    assert_eq!(text("#combobox-test #combobox-value"), "astro");
    assert_eq!(count("#combobox-test .combobox-content"), 0);
    assert_eq!(text("#combobox-test .combobox-trigger"), "Astro");
    assert_eq!(
        attr("#combobox-test .combobox-trigger", "aria-expanded"),
        "false"
    );

    // Reopening shows every item (the label is not used as a filter) and
    // marks the selected one.
    click("#combobox-test .combobox-trigger");
    settle().await;
    settle().await;
    assert_eq!(count("#combobox-test .combobox-item:not([hidden])"), 2);
    assert_eq!(
        attr(
            "#combobox-test .combobox-item[data-value='astro']",
            "aria-selected"
        ),
        "true"
    );
}
