#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;

use shadcn_rs::{
    Checkbox, InputOTP, NativeSelect, NativeSelectOption, Radio, RadioGroup, SelectAdvanced,
    SelectContent, SelectItem, SelectTrigger, SelectValue, Switch, Toggle, ToggleGroup,
    ToggleGroupItem, ToggleGroupType,
};

#[allow(dead_code)]
mod utils;

use utils::{
    attr, change_select, click, click_nth, input, input_value, is_checked, mount_root,
    select_value, settle, text,
};

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

// ---------------------------------------------------------------------------
// Toggle Group
// ---------------------------------------------------------------------------

#[function_component(ToggleGroupHarness)]
fn toggle_group_harness() -> Html {
    let marks = use_state(Vec::<AttrValue>::new);
    let on_value_change = {
        let marks = marks.clone();
        Callback::from(move |value: Vec<AttrValue>| marks.set(value))
    };
    let single = use_state(|| String::from("unset"));
    let on_single_change = {
        let single = single.clone();
        Callback::from(move |value: Vec<AttrValue>| {
            single.set(
                value
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(","),
            )
        })
    };
    let joined = marks
        .iter()
        .map(|v| v.as_str())
        .collect::<Vec<_>>()
        .join(",");

    html! {
        <div>
            <div id="tgm">
                <ToggleGroup
                    r#type={ToggleGroupType::Multiple}
                    value={(*marks).clone()}
                    {on_value_change}
                >
                    <ToggleGroupItem value="bold" aria_label="Bold">{ "B" }</ToggleGroupItem>
                    <ToggleGroupItem value="italic" aria_label="Italic">{ "I" }</ToggleGroupItem>
                </ToggleGroup>
            </div>
            <div id="tgm-value">{ joined }</div>
            <div id="tgs">
                <ToggleGroup
                    r#type={ToggleGroupType::Single}
                    default_value={vec![AttrValue::from("left")]}
                    on_value_change={on_single_change}
                >
                    <ToggleGroupItem value="left">{ "L" }</ToggleGroupItem>
                    <ToggleGroupItem value="right">{ "R" }</ToggleGroupItem>
                </ToggleGroup>
            </div>
            <div id="tgs-value">{ (*single).clone() }</div>
        </div>
    }
}

#[test]
async fn toggle_group_emits_selection() {
    let root = mount_root("toggle-group-test");
    let _app = yew::Renderer::<ToggleGroupHarness>::with_root(root).render();
    settle().await;

    assert_eq!(
        attr("#tgm [data-value='bold']", "aria-label").as_deref(),
        Some("Bold")
    );

    click("#tgm [data-value='bold']");
    settle().await;
    assert_eq!(text("#tgm-value"), "bold");
    click("#tgm [data-value='italic']");
    settle().await;
    assert_eq!(text("#tgm-value"), "bold,italic");
    click("#tgm [data-value='bold']");
    settle().await;
    assert_eq!(text("#tgm-value"), "italic");
    assert_eq!(
        attr("#tgm [data-value='bold']", "aria-pressed").as_deref(),
        Some("false")
    );

    // Single mode: default_value applies, and a deselect reports an empty selection.
    assert_eq!(
        attr("#tgs [data-value='left']", "aria-checked").as_deref(),
        Some("true")
    );
    click("#tgs [data-value='right']");
    settle().await;
    assert_eq!(text("#tgs-value"), "right");
    click("#tgs [data-value='right']");
    settle().await;
    assert_eq!(text("#tgs-value"), "");
}

// ---------------------------------------------------------------------------
// Input OTP
// ---------------------------------------------------------------------------

#[function_component(OtpHarness)]
fn otp_harness() -> Html {
    let code = use_state(String::new);
    let on_change = {
        let code = code.clone();
        Callback::from(move |value: String| code.set(value))
    };
    let clear = {
        let code = code.clone();
        Callback::from(move |_: MouseEvent| code.set(String::new()))
    };
    let preset = {
        let code = code.clone();
        Callback::from(move |_: MouseEvent| code.set(String::from("987")))
    };

    html! {
        <div>
            <div id="otp-controlled">
                <InputOTP length={3} value={(*code).clone()} {on_change} />
            </div>
            <div id="otp-value">{ (*code).clone() }</div>
            <button id="otp-clear" type="button" onclick={clear}>{ "clear" }</button>
            <button id="otp-preset" type="button" onclick={preset}>{ "preset" }</button>
            <div id="otp-uncontrolled">
                <InputOTP length={3} default_value="42" />
            </div>
        </div>
    }
}

#[test]
async fn input_otp_follows_controlled_value() {
    let root = mount_root("otp-test");
    let _app = yew::Renderer::<OtpHarness>::with_root(root).render();
    settle().await;

    assert_eq!(input_value("#otp-uncontrolled input:nth-child(1)"), "4");
    assert_eq!(input_value("#otp-uncontrolled input:nth-child(2)"), "2");

    input("#otp-controlled input:nth-child(1)", "1");
    settle().await;
    input("#otp-controlled input:nth-child(2)", "2");
    settle().await;
    assert_eq!(text("#otp-value"), "12");
    assert_eq!(input_value("#otp-controlled input:nth-child(2)"), "2");

    // The parent clears the code after mount; the fields must follow.
    click("#otp-clear");
    settle().await;
    assert_eq!(input_value("#otp-controlled input:nth-child(1)"), "");
    assert_eq!(input_value("#otp-controlled input:nth-child(2)"), "");

    click("#otp-preset");
    settle().await;
    assert_eq!(input_value("#otp-controlled input:nth-child(1)"), "9");
    assert_eq!(input_value("#otp-controlled input:nth-child(3)"), "7");
}

// ---------------------------------------------------------------------------
// Native Select
// ---------------------------------------------------------------------------

#[function_component(NativeSelectHarness)]
fn native_select_harness() -> Html {
    let value = use_state(|| AttrValue::from("b"));
    let onchange = {
        let value = value.clone();
        Callback::from(move |e: Event| {
            let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
            value.set(AttrValue::from(select.value()));
        })
    };
    let set_c = {
        let value = value.clone();
        Callback::from(move |_: MouseEvent| value.set(AttrValue::from("c")))
    };
    let options = || {
        html! {
            <>
                <NativeSelectOption value="a">{ "A" }</NativeSelectOption>
                <NativeSelectOption value="b">{ "B" }</NativeSelectOption>
                <NativeSelectOption value="c">{ "C" }</NativeSelectOption>
            </>
        }
    };

    html! {
        <div>
            <NativeSelect id="ns-default" default_value="c">{ options() }</NativeSelect>
            <NativeSelect id="ns-controlled" value={(*value).clone()} {onchange}>
                { options() }
            </NativeSelect>
            <div id="ns-value">{ (*value).clone() }</div>
            <button id="ns-set-c" type="button" onclick={set_c}>{ "c" }</button>
            // Controlled with no parent update: a pick must snap back.
            <NativeSelect id="ns-locked" value="a">{ options() }</NativeSelect>
        </div>
    }
}

#[test]
async fn native_select_value_and_default_value() {
    let root = mount_root("native-select-test");
    let _app = yew::Renderer::<NativeSelectHarness>::with_root(root).render();
    settle().await;

    assert_eq!(select_value("#ns-default"), "c");
    assert_eq!(select_value("#ns-controlled"), "b");

    click("#ns-set-c");
    settle().await;
    assert_eq!(select_value("#ns-controlled"), "c");

    change_select("#ns-controlled", "a");
    settle().await;
    assert_eq!(text("#ns-value"), "a");
    assert_eq!(select_value("#ns-controlled"), "a");

    change_select("#ns-locked", "c");
    settle().await;
    assert_eq!(select_value("#ns-locked"), "a");

    // The uncontrolled select keeps the user's pick across re-renders.
    change_select("#ns-default", "a");
    click("#ns-set-c");
    settle().await;
    assert_eq!(select_value("#ns-default"), "a");
}

// ---------------------------------------------------------------------------
// Select (custom)
// ---------------------------------------------------------------------------

#[function_component(SelectHarness)]
fn select_harness() -> Html {
    let fruit = use_state(|| AttrValue::from("banana"));
    let on_value_change = {
        let fruit = fruit.clone();
        Callback::from(move |value: AttrValue| fruit.set(value))
    };
    let set_orange = {
        let fruit = fruit.clone();
        Callback::from(move |_: MouseEvent| fruit.set(AttrValue::from("orange")))
    };
    let items = || {
        html! {
            <SelectContent>
                <SelectItem value="apple">{ "Apple" }</SelectItem>
                <SelectItem value="banana">{ "Banana" }</SelectItem>
                <SelectItem value="orange">{ "Orange" }</SelectItem>
            </SelectContent>
        }
    };

    html! {
        <div>
            <div id="sel-default">
                <SelectAdvanced default_value="apple">
                    <SelectTrigger><SelectValue placeholder="Pick" /></SelectTrigger>
                    { items() }
                </SelectAdvanced>
            </div>
            <div id="sel-controlled">
                <SelectAdvanced value={(*fruit).clone()} {on_value_change}>
                    <SelectTrigger><SelectValue placeholder="Pick" /></SelectTrigger>
                    { items() }
                </SelectAdvanced>
            </div>
            <button id="sel-set-orange" type="button" onclick={set_orange}>{ "orange" }</button>
            <div id="sel-empty">
                <SelectAdvanced>
                    <SelectTrigger><SelectValue placeholder="Pick" /></SelectTrigger>
                    { items() }
                </SelectAdvanced>
            </div>
        </div>
    }
}

#[test]
async fn select_shows_label_for_preset_value() {
    let root = mount_root("select-test");
    let _app = yew::Renderer::<SelectHarness>::with_root(root).render();
    settle().await;

    // Labels show on first render, before the list was ever opened.
    assert_eq!(text("#sel-default .select-value"), "Apple");
    assert_eq!(text("#sel-controlled .select-value"), "Banana");
    assert_eq!(text("#sel-empty .select-value"), "Pick");
    assert!(attr("#sel-default .select-content", "hidden").is_some());

    click("#sel-set-orange");
    settle().await;
    assert_eq!(text("#sel-controlled .select-value"), "Orange");

    click("#sel-default .select-trigger");
    settle().await;
    assert!(attr("#sel-default .select-content", "hidden").is_none());
    click_nth("#sel-default .select-item", 2);
    settle().await;
    assert_eq!(text("#sel-default .select-value"), "Orange");
    assert!(attr("#sel-default .select-content", "hidden").is_some());
}
