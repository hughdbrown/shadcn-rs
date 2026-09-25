#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;

use shadcn_rs::{
    AlertDialog, AlertDialogContent, AlertDialogTrigger, Dialog, DialogContent, DialogTrigger,
    Drawer, DrawerContent, DrawerTrigger, Sheet, SheetContent, SheetTrigger,
};

mod utils;

use utils::{click, keydown, mount_root, settle, text};

wasm_bindgen_test_configure!(run_in_browser);

fn exists(selector: &str) -> bool {
    gloo::utils::document()
        .query_selector(selector)
        .expect("query selector failed")
        .is_some()
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
