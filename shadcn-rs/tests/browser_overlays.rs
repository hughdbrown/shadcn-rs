#![cfg(target_arch = "wasm32")]

use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use web_sys::{MouseEvent, MouseEventInit};
use yew::prelude::*;

use shadcn_rs::{
    AlertDialog, AlertDialogContent, AlertDialogTrigger, Collapsible, CollapsibleContent,
    CollapsibleTrigger, Dialog, DialogContent, DialogTrigger, Drawer, DrawerContent, DrawerTrigger,
    Popover, PopoverContent, PopoverTrigger, Sheet, SheetContent, SheetTrigger, Tooltip,
    TooltipContent, TooltipProvider, TooltipTrigger,
};

mod utils;

use utils::{active_id, click, focus, keydown, mount_root, query, settle, text};

wasm_bindgen_test_configure!(run_in_browser);

fn exists(selector: &str) -> bool {
    gloo::utils::document()
        .query_selector(selector)
        .expect("query selector failed")
        .is_some()
}

fn attr(selector: &str, name: &str) -> String {
    query(selector).get_attribute(name).unwrap_or_default()
}

/// Dispatches a mouse event (`mousedown`, `mouseenter`, `mouseleave`, ...).
fn mouse(selector: &str, kind: &str) {
    let init = MouseEventInit::new();
    // mouseenter/mouseleave don't bubble in browsers; mirror that.
    init.set_bubbles(!matches!(kind, "mouseenter" | "mouseleave"));
    init.set_cancelable(true);
    let event =
        MouseEvent::new_with_mouse_event_init_dict(kind, &init).expect("failed to create event");
    query(selector)
        .dispatch_event(&event)
        .expect("failed to dispatch mouse event");
}

fn blur_active() {
    if let Some(active) = gloo::utils::document().active_element()
        && let Ok(element) = active.dyn_into::<web_sys::HtmlElement>()
    {
        element.blur().expect("blur failed");
    }
}

async fn wait_ms(ms: u64) {
    yew::platform::time::sleep(std::time::Duration::from_millis(ms)).await;
}

// ---------------------------------------------------------------------------
// Dialog family: controlled `open: Option<bool>`, `default_open`, observable
// `on_open_change` in uncontrolled mode.
// ---------------------------------------------------------------------------

/// Which dialog-like component a harness renders.
#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Dialog,
    AlertDialog,
    Drawer,
    Sheet,
}

#[derive(Properties, PartialEq)]
struct ModalHarnessProps {
    kind: Kind,
    #[prop_or_default]
    open: Option<bool>,
    #[prop_or_default]
    default_open: bool,
    #[prop_or_default]
    observe: bool,
}

/// Renders the chosen component; when `observe` is set it records every
/// `on_open_change` value into `#events` without controlling the component.
#[function_component(ModalHarness)]
fn modal_harness(props: &ModalHarnessProps) -> Html {
    let events = use_state(Vec::<bool>::new);
    let on_open_change = props.observe.then(|| {
        let events = events.clone();
        Callback::from(move |value: bool| {
            let mut next = (*events).clone();
            next.push(value);
            events.set(next);
        })
    });
    let log = events
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",");
    let trigger = html! { <button id="modal-trigger" type="button">{ "Open" }</button> };
    let body = html! { <button id="modal-body" type="button">{ "Body" }</button> };
    let open = props.open;
    let default_open = props.default_open;

    let component = match props.kind {
        Kind::Dialog => html! {
            <Dialog {open} {default_open} {on_open_change}>
                <DialogTrigger>{ trigger }</DialogTrigger>
                <DialogContent>{ body }</DialogContent>
            </Dialog>
        },
        Kind::AlertDialog => html! {
            <AlertDialog {open} {default_open} {on_open_change}>
                <AlertDialogTrigger>{ trigger }</AlertDialogTrigger>
                <AlertDialogContent>{ body }</AlertDialogContent>
            </AlertDialog>
        },
        Kind::Drawer => html! {
            <Drawer {open} {default_open} {on_open_change}>
                <DrawerTrigger>{ trigger }</DrawerTrigger>
                <DrawerContent>{ body }</DrawerContent>
            </Drawer>
        },
        Kind::Sheet => html! {
            <Sheet {open} {default_open} {on_open_change}>
                <SheetTrigger>{ trigger }</SheetTrigger>
                <SheetContent>{ body }</SheetContent>
            </Sheet>
        },
    };

    html! {
        <>
            <span id="events">{ log }</span>
            { component }
        </>
    }
}

const KINDS: [Kind; 4] = [Kind::Dialog, Kind::AlertDialog, Kind::Drawer, Kind::Sheet];

#[test]
async fn modal_default_open_renders_open() {
    for kind in KINDS {
        let root = mount_root("modal-default-open");
        let app = yew::Renderer::<ModalHarness>::with_root_and_props(
            root,
            ModalHarnessProps {
                kind,
                open: None,
                default_open: true,
                observe: false,
            },
        )
        .render();
        settle().await;
        assert!(exists("#modal-body"), "default_open must render content");
        app.destroy();
        settle().await;
    }
}

#[test]
async fn modal_uncontrolled_open_change_is_observable() {
    for kind in KINDS {
        let root = mount_root("modal-observe");
        let app = yew::Renderer::<ModalHarness>::with_root_and_props(
            root,
            ModalHarnessProps {
                kind,
                open: None,
                default_open: false,
                observe: true,
            },
        )
        .render();
        settle().await;
        assert!(!exists("#modal-body"));

        // Passing only `on_open_change` must not lock the component closed.
        click("#modal-trigger");
        settle().await;
        assert!(
            exists("#modal-body"),
            "trigger must open uncontrolled modal"
        );

        keydown("#modal-body", "Escape", false);
        settle().await;
        assert!(!exists("#modal-body"), "Escape must close");
        assert_eq!(text("#events"), "true,false");
        app.destroy();
        settle().await;
    }
}

#[test]
async fn modal_controlled_open_is_honored_without_callback() {
    for kind in KINDS {
        let root = mount_root("modal-controlled");
        let app = yew::Renderer::<ModalHarness>::with_root_and_props(
            root,
            ModalHarnessProps {
                kind,
                open: Some(true),
                default_open: false,
                observe: true,
            },
        )
        .render();
        settle().await;
        assert!(exists("#modal-body"), "open=Some(true) must render");

        // Controlled: Escape reports the request but the parent keeps it open.
        keydown("#modal-body", "Escape", false);
        settle().await;
        assert!(exists("#modal-body"), "controlled modal must stay open");
        assert_eq!(text("#events"), "false");
        app.destroy();
        settle().await;
    }
}

// ---------------------------------------------------------------------------
// Collapsible: the toggle must flip the effective (controlled) value.
// ---------------------------------------------------------------------------

#[function_component(ControlledCollapsible)]
fn controlled_collapsible() -> Html {
    let open = use_state(|| true);
    let on_open_change = {
        let open = open.clone();
        Callback::from(move |value: bool| open.set(value))
    };
    html! {
        <Collapsible open={*open} {on_open_change}>
            <CollapsibleTrigger>{ "Toggle" }</CollapsibleTrigger>
            <CollapsibleContent>
                <span id="collapsible-body">{ "Body" }</span>
            </CollapsibleContent>
        </Collapsible>
    }
}

#[test]
async fn collapsible_controlled_trigger_closes_and_reopens() {
    let root = mount_root("collapsible-controlled");
    let app = yew::Renderer::<ControlledCollapsible>::with_root(root).render();
    settle().await;
    assert!(exists("#collapsible-body"));
    assert_eq!(attr(".collapsible-trigger", "aria-expanded"), "true");

    // Before the fix this emitted `true` again (internal state started false).
    click(".collapsible-trigger");
    settle().await;
    assert!(
        !exists("#collapsible-body"),
        "controlled trigger must close"
    );
    assert_eq!(attr(".collapsible-trigger", "aria-expanded"), "false");

    click(".collapsible-trigger");
    settle().await;
    assert!(
        exists("#collapsible-body"),
        "controlled trigger must reopen"
    );
    app.destroy();
    settle().await;
}

// ---------------------------------------------------------------------------
// Popover: root state, trigger toggle, outside click, Escape, focus restore.
// ---------------------------------------------------------------------------

#[derive(Properties, PartialEq)]
struct PopoverHarnessProps {
    #[prop_or_default]
    controlled: bool,
    #[prop_or_default]
    default_open: bool,
}

#[function_component(PopoverHarness)]
fn popover_harness(props: &PopoverHarnessProps) -> Html {
    let state = use_state(|| props.default_open);
    let on_open_change = {
        let state = state.clone();
        Callback::from(move |value: bool| state.set(value))
    };
    let open = props.controlled.then_some(*state);
    html! {
        <>
            <button id="outside" type="button">{ "Outside" }</button>
            <Popover {open} default_open={props.default_open} {on_open_change}>
                <PopoverTrigger>
                    <button id="popover-trigger" type="button">{ "Open" }</button>
                </PopoverTrigger>
                <PopoverContent>
                    <button id="popover-inner" type="button">{ "Inner" }</button>
                </PopoverContent>
            </Popover>
        </>
    }
}

#[test]
async fn popover_trigger_toggles_uncontrolled_and_controlled() {
    for controlled in [false, true] {
        let root = mount_root("popover-toggle");
        let app = yew::Renderer::<PopoverHarness>::with_root_and_props(
            root,
            PopoverHarnessProps {
                controlled,
                default_open: false,
            },
        )
        .render();
        settle().await;
        assert!(!exists("#popover-inner"));
        assert_eq!(attr(".popover-trigger", "aria-expanded"), "false");

        click("#popover-trigger");
        settle().await;
        assert!(exists("#popover-inner"), "trigger must open");
        assert_eq!(attr(".popover-trigger", "aria-expanded"), "true");
        // Content is anchored inside the root, next to the trigger.
        assert!(
            query(".popover-root")
                .query_selector(".popover-content")
                .expect("query failed")
                .is_some()
        );

        click("#popover-trigger");
        settle().await;
        assert!(!exists("#popover-inner"), "trigger must close");
        app.destroy();
        settle().await;
    }
}

#[test]
async fn popover_default_open_and_outside_click() {
    let root = mount_root("popover-outside");
    let app = yew::Renderer::<PopoverHarness>::with_root_and_props(
        root,
        PopoverHarnessProps {
            controlled: false,
            default_open: true,
        },
    )
    .render();
    settle().await;
    assert!(exists("#popover-inner"), "default_open must render content");

    // A press inside the content must not close it.
    mouse("#popover-inner", "mousedown");
    settle().await;
    assert!(exists("#popover-inner"));

    mouse("#outside", "mousedown");
    settle().await;
    assert!(!exists("#popover-inner"), "outside click must close");
    app.destroy();
    settle().await;
}

#[test]
async fn popover_escape_closes_and_restores_focus() {
    let root = mount_root("popover-escape");
    let app = yew::Renderer::<PopoverHarness>::with_root_and_props(
        root,
        PopoverHarnessProps {
            controlled: true,
            default_open: false,
        },
    )
    .render();
    settle().await;
    focus("#popover-trigger");
    click("#popover-trigger");
    settle().await;
    assert_eq!(active_id(), "popover-inner", "focus moves into content");

    keydown("#popover-inner", "Escape", false);
    settle().await;
    assert!(!exists("#popover-inner"), "Escape must close");
    assert_eq!(active_id(), "popover-trigger", "focus returns to trigger");
    app.destroy();
    settle().await;
}

// ---------------------------------------------------------------------------
// Tooltip: hidden by default; hover (after delay) and focus show it; leave,
// blur and Escape hide it; open/default_open; provider delay.
// ---------------------------------------------------------------------------

#[derive(Properties, PartialEq, Default)]
struct TooltipHarnessProps {
    #[prop_or_default]
    delay: Option<u32>,
    #[prop_or_default]
    open: Option<bool>,
    #[prop_or_default]
    default_open: bool,
    #[prop_or_default]
    provider_delay: Option<u32>,
}

#[function_component(TooltipHarness)]
fn tooltip_harness(props: &TooltipHarnessProps) -> Html {
    let tooltip = html! {
        <Tooltip delay_duration={props.delay} open={props.open} default_open={props.default_open}>
            <TooltipTrigger>
                <button id="tip-trigger" type="button">{ "Hover" }</button>
            </TooltipTrigger>
            <TooltipContent>{ "Tip" }</TooltipContent>
        </Tooltip>
    };
    match props.provider_delay {
        Some(delay) => html! {
            <TooltipProvider delay_duration={delay}>{ tooltip }</TooltipProvider>
        },
        None => tooltip,
    }
}

fn render_tooltip(id: &str, props: TooltipHarnessProps) -> yew::AppHandle<TooltipHarness> {
    yew::Renderer::<TooltipHarness>::with_root_and_props(mount_root(id), props).render()
}

#[test]
async fn tooltip_hidden_until_hover_delay_then_hides_on_leave() {
    let app = render_tooltip(
        "tooltip-hover",
        TooltipHarnessProps {
            delay: Some(50),
            ..TooltipHarnessProps::default()
        },
    );
    settle().await;
    assert!(!exists("[role='tooltip']"), "tooltip must start hidden");

    mouse(".tooltip-trigger", "mouseenter");
    settle().await;
    assert!(!exists("[role='tooltip']"), "must wait for the delay");
    wait_ms(120).await;
    assert!(exists("[role='tooltip']"), "must show after the delay");
    let id = attr("[role='tooltip']", "id");
    assert!(!id.is_empty());
    assert_eq!(attr(".tooltip-trigger", "aria-describedby"), id);

    mouse(".tooltip-trigger", "mouseleave");
    settle().await;
    assert!(!exists("[role='tooltip']"), "must hide on pointer leave");

    // Leaving before the delay elapses cancels the pending open.
    mouse(".tooltip-trigger", "mouseenter");
    settle().await;
    mouse(".tooltip-trigger", "mouseleave");
    wait_ms(120).await;
    assert!(!exists("[role='tooltip']"), "cancelled open must not fire");
    app.destroy();
    settle().await;
}

#[test]
async fn tooltip_focus_shows_and_blur_or_escape_hides() {
    let app = render_tooltip(
        "tooltip-focus",
        TooltipHarnessProps {
            delay: Some(10_000),
            ..TooltipHarnessProps::default()
        },
    );
    settle().await;

    // Keyboard focus opens immediately, regardless of the hover delay.
    focus("#tip-trigger");
    settle().await;
    assert!(exists("[role='tooltip']"), "focus must show");

    keydown("#tip-trigger", "Escape", false);
    settle().await;
    assert!(!exists("[role='tooltip']"), "Escape must hide");

    blur_active();
    settle().await;
    focus("#tip-trigger");
    settle().await;
    assert!(exists("[role='tooltip']"));
    blur_active();
    settle().await;
    assert!(!exists("[role='tooltip']"), "blur must hide");
    app.destroy();
    settle().await;
}

#[test]
async fn tooltip_open_and_default_open() {
    let app = render_tooltip(
        "tooltip-open",
        TooltipHarnessProps {
            open: Some(true),
            ..TooltipHarnessProps::default()
        },
    );
    settle().await;
    assert!(exists("[role='tooltip']"), "open=Some(true) must show");
    // Controlled: leaving only reports the request.
    mouse(".tooltip-trigger", "mouseleave");
    settle().await;
    assert!(exists("[role='tooltip']"), "controlled tooltip stays open");
    app.destroy();
    settle().await;

    let app = render_tooltip(
        "tooltip-default-open",
        TooltipHarnessProps {
            default_open: true,
            ..TooltipHarnessProps::default()
        },
    );
    settle().await;
    assert!(exists("[role='tooltip']"), "default_open must show");
    mouse(".tooltip-trigger", "mouseleave");
    settle().await;
    assert!(!exists("[role='tooltip']"));
    app.destroy();
    settle().await;
}

#[test]
async fn tooltip_uses_provider_delay() {
    let app = render_tooltip(
        "tooltip-provider",
        TooltipHarnessProps {
            provider_delay: Some(0),
            ..TooltipHarnessProps::default()
        },
    );
    settle().await;
    mouse(".tooltip-trigger", "mouseenter");
    settle().await;
    assert!(exists("[role='tooltip']"), "provider delay 0 opens at once");
    app.destroy();
    settle().await;
}
