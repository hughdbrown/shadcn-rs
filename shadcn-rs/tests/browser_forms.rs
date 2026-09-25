#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;

use shadcn_rs::{Checkbox, Switch};

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
