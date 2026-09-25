#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;

use shadcn_rs::{Checkbox, Radio, RadioGroup, Switch, Toggle};

#[allow(dead_code)]
mod utils;

use utils::{attr, click, is_checked, mount_root, settle, text};

wasm_bindgen_test_configure!(run_in_browser);

// ---------------------------------------------------------------------------
// Checkbox
// ---------------------------------------------------------------------------

#[function_component(CheckboxHarness)]
fn checkbox_harness() -> Html {
    let checked = use_state(|| false);
    let requested = use_state(|| String::from("none"));
    let on_checked_change = {
        let checked = checked.clone();
        let requested = requested.clone();
        Callback::from(move |value: bool| {
            checked.set(value);
            requested.set(value.to_string());
        })
    };
    let rejected = use_state(|| String::from("none"));
    let on_rejected_change = {
        let rejected = rejected.clone();
        Callback::from(move |value: bool| rejected.set(value.to_string()))
    };

    html! {
        <div>
            <Checkbox id="cb-uncontrolled" default_checked={true} />
            <Checkbox id="cb-controlled" checked={*checked} {on_checked_change} />
            <div id="cb-requested">{ (*requested).clone() }</div>
            // The parent ignores the request, so the box must stay unchecked.
            <Checkbox id="cb-locked" checked={false} on_checked_change={on_rejected_change} />
            <div id="cb-rejected">{ (*rejected).clone() }</div>
        </div>
    }
}

#[test]
async fn checkbox_default_checked_and_controlled() {
    let root = mount_root("checkbox-test");
    let _app = yew::Renderer::<CheckboxHarness>::with_root(root).render();
    settle().await;

    assert!(is_checked("#cb-uncontrolled"));
    click("#cb-uncontrolled");
    settle().await;
    assert!(!is_checked("#cb-uncontrolled"));

    assert!(!is_checked("#cb-controlled"));
    click("#cb-controlled");
    settle().await;
    assert!(is_checked("#cb-controlled"));
    assert_eq!(text("#cb-requested"), "true");
    assert_eq!(
        attr("#cb-controlled", "aria-checked").as_deref(),
        Some("true")
    );

    click("#cb-locked");
    settle().await;
    assert_eq!(text("#cb-rejected"), "true");
    assert!(!is_checked("#cb-locked"));
}

// ---------------------------------------------------------------------------
// Switch
// ---------------------------------------------------------------------------

#[function_component(SwitchHarness)]
fn switch_harness() -> Html {
    let on = use_state(|| true);
    let emitted = use_state(|| String::from("none"));
    let on_checked_change = {
        let on = on.clone();
        let emitted = emitted.clone();
        Callback::from(move |value: bool| {
            on.set(value);
            emitted.set(value.to_string());
        })
    };
    let force_off = {
        let on = on.clone();
        Callback::from(move |_: MouseEvent| on.set(false))
    };

    html! {
        <div>
            <Switch id="sw-uncontrolled" default_checked={true} />
            <Switch id="sw-controlled" checked={*on} {on_checked_change} />
            <button id="sw-force-off" type="button" onclick={force_off}>{ "off" }</button>
            <div id="sw-emitted">{ (*emitted).clone() }</div>
            <Switch id="sw-locked" checked={false} />
        </div>
    }
}

#[test]
async fn switch_controlled_and_uncontrolled() {
    let root = mount_root("switch-test");
    let _app = yew::Renderer::<SwitchHarness>::with_root(root).render();
    settle().await;

    assert_eq!(
        attr("#sw-uncontrolled", "aria-checked").as_deref(),
        Some("true")
    );
    click("#sw-uncontrolled");
    settle().await;
    assert_eq!(
        attr("#sw-uncontrolled", "aria-checked").as_deref(),
        Some("false")
    );

    // The parent can force a controlled switch off.
    assert_eq!(
        attr("#sw-controlled", "aria-checked").as_deref(),
        Some("true")
    );
    click("#sw-force-off");
    settle().await;
    assert_eq!(
        attr("#sw-controlled", "aria-checked").as_deref(),
        Some("false")
    );

    // Clicking reports the new value.
    click("#sw-controlled");
    settle().await;
    assert_eq!(text("#sw-emitted"), "true");
    assert_eq!(
        attr("#sw-controlled", "aria-checked").as_deref(),
        Some("true")
    );

    // Without a parent update, a controlled switch does not move.
    click("#sw-locked");
    settle().await;
    assert_eq!(attr("#sw-locked", "aria-checked").as_deref(), Some("false"));
}

// ---------------------------------------------------------------------------
// Radio Group
// ---------------------------------------------------------------------------

#[function_component(RadioHarness)]
fn radio_harness() -> Html {
    let emitted = use_state(|| String::from("none"));
    let on_uncontrolled = {
        let emitted = emitted.clone();
        Callback::from(move |value: AttrValue| emitted.set(value.to_string()))
    };
    let rejected = use_state(|| String::from("none"));
    let on_rejected = {
        let rejected = rejected.clone();
        Callback::from(move |value: AttrValue| rejected.set(value.to_string()))
    };
    let renders = use_state(|| 0u32);
    let rerender = {
        let renders = renders.clone();
        Callback::from(move |_: MouseEvent| renders.set(*renders + 1))
    };

    html! {
        <div>
            <RadioGroup name="rg-free" default_value="b" on_value_change={on_uncontrolled}>
                <Radio id="rg-free-a" value="a" />
                <Radio id="rg-free-b" value="b" />
                <Radio id="rg-free-c" value="c" />
            </RadioGroup>
            <div id="rg-emitted">{ (*emitted).clone() }</div>

            // Controlled, and the parent never accepts a change.
            <RadioGroup name="rg-locked" value="a" on_value_change={on_rejected}>
                <Radio id="rg-locked-a" value="a" />
                <Radio id="rg-locked-b" value="b" />
            </RadioGroup>
            <div id="rg-rejected">{ (*rejected).clone() }</div>

            <RadioGroup name="rg-disabled" disabled={true}>
                <Radio id="rg-disabled-a" value="a" />
            </RadioGroup>

            // Standalone radios still group by name.
            <Radio id="solo-a" name="solo" value="a" default_checked={true} />
            <Radio id="solo-b" name="solo" value="b" />
            <button id="solo-rerender" type="button" onclick={rerender}>
                { renders.to_string() }
            </button>
        </div>
    }
}

#[test]
async fn radio_group_drives_items() {
    let root = mount_root("radio-test");
    let _app = yew::Renderer::<RadioHarness>::with_root(root).render();
    settle().await;

    // Items take their name and checked state from the group.
    assert_eq!(attr("#rg-free-a", "name").as_deref(), Some("rg-free"));
    assert!(!is_checked("#rg-free-a"));
    assert!(is_checked("#rg-free-b"));

    click("#rg-free-c");
    settle().await;
    assert_eq!(text("#rg-emitted"), "c");
    assert!(is_checked("#rg-free-c"));
    assert!(!is_checked("#rg-free-b"));

    // A controlled group keeps the owner's value when the owner ignores a change.
    assert!(is_checked("#rg-locked-a"));
    click("#rg-locked-b");
    settle().await;
    assert_eq!(text("#rg-rejected"), "b");
    assert!(is_checked("#rg-locked-a"));
    assert!(!is_checked("#rg-locked-b"));

    // Group-level disabled reaches the items.
    assert!(attr("#rg-disabled-a", "disabled").is_some());

    // Standalone uncontrolled radios keep the user's choice across re-renders.
    assert!(is_checked("#solo-a"));
    click("#solo-b");
    settle().await;
    click("#solo-rerender");
    settle().await;
    assert!(is_checked("#solo-b"));
    assert!(!is_checked("#solo-a"));
}

// ---------------------------------------------------------------------------
// Toggle
// ---------------------------------------------------------------------------

#[function_component(ToggleHarness)]
fn toggle_harness() -> Html {
    let emitted = use_state(|| String::from("none"));
    let on_pressed_change = {
        let emitted = emitted.clone();
        Callback::from(move |value: bool| emitted.set(value.to_string()))
    };
    let on = use_state(|| false);
    let on_controlled_change = {
        let on = on.clone();
        Callback::from(move |value: bool| on.set(value))
    };

    html! {
        <div>
            <div id="tg-uncontrolled">
                <Toggle aria_label="Bold" {on_pressed_change}>{ "B" }</Toggle>
            </div>
            <div id="tg-emitted">{ (*emitted).clone() }</div>
            <div id="tg-controlled">
                <Toggle pressed={*on} on_pressed_change={on_controlled_change}>{ "I" }</Toggle>
            </div>
            // Controlled with no parent update: must stay unpressed.
            <div id="tg-locked">
                <Toggle pressed={false}>{ "U" }</Toggle>
            </div>
        </div>
    }
}

#[test]
async fn toggle_pressed_state_and_label() {
    let root = mount_root("toggle-test");
    let _app = yew::Renderer::<ToggleHarness>::with_root(root).render();
    settle().await;

    assert_eq!(
        attr("#tg-uncontrolled button", "aria-label").as_deref(),
        Some("Bold")
    );
    click("#tg-uncontrolled button");
    settle().await;
    assert_eq!(text("#tg-emitted"), "true");
    assert_eq!(
        attr("#tg-uncontrolled button", "aria-pressed").as_deref(),
        Some("true")
    );

    click("#tg-controlled button");
    settle().await;
    assert_eq!(
        attr("#tg-controlled button", "aria-pressed").as_deref(),
        Some("true")
    );
    click("#tg-controlled button");
    settle().await;
    assert_eq!(
        attr("#tg-controlled button", "aria-pressed").as_deref(),
        Some("false")
    );

    click("#tg-locked button");
    settle().await;
    assert_eq!(
        attr("#tg-locked button", "aria-pressed").as_deref(),
        Some("false")
    );
}
