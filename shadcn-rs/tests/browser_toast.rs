#![cfg(target_arch = "wasm32")]

use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;

use shadcn_rs::{PromiseMessages, ToastId, ToastOptions, ToastPosition, Toaster, use_toast};

#[allow(dead_code)]
mod utils;

use utils::{click, count, mount_root, query, settle, text, wait_ms};

wasm_bindgen_test_configure!(run_in_browser);

#[function_component(ToastButtons)]
fn toast_buttons() -> Html {
    let toast = use_toast();
    let last = use_mut_ref(|| Option::<ToastId>::None);

    let add = {
        let toast = toast.clone();
        let last = last.clone();
        Callback::from(move |_: MouseEvent| {
            let id = toast.add(ToastOptions::new("Hello toast").description("Saved just now"));
            *last.borrow_mut() = Some(id);
        })
    };
    let update = {
        let toast = toast.clone();
        let last = last.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(id) = *last.borrow() {
                toast.update(id, ToastOptions::success("Updated text"));
            }
        })
    };
    let error = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            toast.error("Broken");
        })
    };
    let quick = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            toast.add(ToastOptions::new("Quick").duration(50));
        })
    };
    let promise = {
        let toast = toast.clone();
        Callback::from(move |_: MouseEvent| {
            toast.promise(
                async {
                    yew::platform::time::sleep(std::time::Duration::from_millis(100)).await;
                    Ok::<u32, String>(7)
                },
                PromiseMessages::new("Loading data", "Loaded", "Failed")
                    .success_with(|n: &u32| ToastOptions::success(format!("Loaded {n} rows"))),
            );
        })
    };
    let action_ran = use_state(|| false);
    let with_action = {
        let toast = toast.clone();
        let action_ran = action_ran.clone();
        Callback::from(move |_: MouseEvent| {
            let action_ran = action_ran.clone();
            toast.add(
                ToastOptions::new("Deleted")
                    .sticky()
                    .action("Undo", Callback::from(move |()| action_ran.set(true))),
            );
        })
    };

    html! {
        <>
            <button id="add" type="button" onclick={add}>{ "Add" }</button>
            <button id="update" type="button" onclick={update}>{ "Update" }</button>
            <button id="error" type="button" onclick={error}>{ "Error" }</button>
            <button id="quick" type="button" onclick={quick}>{ "Quick" }</button>
            <button id="with-action" type="button" onclick={with_action}>{ "Action" }</button>
            <button id="promise" type="button" onclick={promise}>{ "Promise" }</button>
            <span id="action-ran">{ action_ran.to_string() }</span>
        </>
    }
}

#[function_component(ToastHarness)]
fn toast_harness() -> Html {
    html! {
        <Toaster position={ToastPosition::TopRight} limit={3}>
            <ToastButtons />
        </Toaster>
    }
}

fn mount(id: &str) -> yew::AppHandle<ToastHarness> {
    yew::Renderer::<ToastHarness>::with_root(mount_root(id)).render()
}

#[test]
async fn click_shows_toast_with_text() {
    let _app = mount("toast-show");
    settle().await;
    assert_eq!(count("#toast-show .toast"), 0);

    click("#toast-show #add");
    settle().await;

    assert_eq!(count("#toast-show .toast"), 1);
    assert_eq!(text("#toast-show .toast .toast-title"), "Hello toast");
    assert_eq!(
        text("#toast-show .toast .toast-description"),
        "Saved just now"
    );
    let toast = query("#toast-show .toast");
    assert_eq!(toast.get_attribute("role").as_deref(), Some("status"));
    assert_eq!(toast.get_attribute("aria-live").as_deref(), Some("polite"));
    assert_eq!(
        query("#toast-show .toaster-region")
            .get_attribute("aria-label")
            .as_deref(),
        Some("Notifications")
    );
}

#[test]
async fn close_button_removes_toast() {
    let _app = mount("toast-close");
    settle().await;
    click("#toast-close #add");
    settle().await;
    assert_eq!(count("#toast-close .toast"), 1);

    let close = query("#toast-close .toast-close");
    assert_eq!(
        close.get_attribute("aria-label").as_deref(),
        Some("Close notification")
    );
    click("#toast-close .toast-close");
    settle().await;
    assert_eq!(
        query("#toast-close .toast")
            .get_attribute("data-state")
            .as_deref(),
        Some("closed")
    );

    wait_ms(400).await;
    assert_eq!(count("#toast-close .toast"), 0);
}

#[test]
async fn update_changes_text_and_type() {
    let _app = mount("toast-update");
    settle().await;
    click("#toast-update #add");
    settle().await;
    assert_eq!(text("#toast-update .toast-title"), "Hello toast");

    click("#toast-update #update");
    settle().await;
    assert_eq!(count("#toast-update .toast"), 1);
    assert_eq!(text("#toast-update .toast-title"), "Updated text");
    assert_eq!(
        query("#toast-update .toast")
            .get_attribute("data-type")
            .as_deref(),
        Some("success")
    );
    assert_eq!(count("#toast-update .toast-description"), 0);
}

#[test]
async fn error_toast_is_an_alert() {
    let _app = mount("toast-error");
    settle().await;
    click("#toast-error #error");
    settle().await;
    let toast = query("#toast-error .toast");
    assert_eq!(toast.get_attribute("role").as_deref(), Some("alert"));
    assert_eq!(
        toast.get_attribute("aria-live").as_deref(),
        Some("assertive")
    );
}

#[test]
async fn limit_hides_older_toasts_until_a_slot_frees() {
    let _app = mount("toast-limit");
    settle().await;
    for _ in 0..4 {
        click("#toast-limit #add");
        settle().await;
    }
    assert_eq!(count("#toast-limit .toast"), 3);

    click("#toast-limit .toast-close");
    wait_ms(400).await;
    // The closed toast leaves and the queued oldest one takes its slot.
    assert_eq!(count("#toast-limit .toast"), 3);
}

#[test]
async fn toast_auto_dismisses_after_duration() {
    let _app = mount("toast-auto");
    settle().await;
    click("#toast-auto #quick");
    settle().await;
    assert_eq!(count("#toast-auto .toast"), 1);

    wait_ms(500).await;
    assert_eq!(count("#toast-auto .toast"), 0);
}

#[test]
async fn action_runs_callback_and_closes() {
    let _app = mount("toast-action");
    settle().await;
    click("#toast-action #with-action");
    settle().await;
    assert_eq!(text("#toast-action .toast-action"), "Undo");

    click("#toast-action .toast-action");
    wait_ms(400).await;
    assert_eq!(count("#toast-action .toast"), 0);
    assert_eq!(text("#toast-action #action-ran"), "true");
}

#[test]
async fn promise_turns_loading_into_success() {
    let _app = mount("toast-promise");
    settle().await;
    click("#toast-promise #promise");
    settle().await;
    assert_eq!(text("#toast-promise .toast-title"), "Loading data");
    assert_eq!(
        query("#toast-promise .toast")
            .get_attribute("data-type")
            .as_deref(),
        Some("loading")
    );

    wait_ms(300).await;
    assert_eq!(count("#toast-promise .toast"), 1);
    assert_eq!(text("#toast-promise .toast-title"), "Loaded 7 rows");
    assert_eq!(
        query("#toast-promise .toast")
            .get_attribute("data-type")
            .as_deref(),
        Some("success")
    );
}

#[allow(deprecated)]
#[function_component(LegacySonnerHarness)]
fn legacy_sonner_harness() -> Html {
    use shadcn_rs::{Sonner, SonnerPosition, SonnerToast, SonnerType};
    html! {
        <Sonner position={SonnerPosition::TopCenter}>
            <SonnerToast r#type={SonnerType::Success} title="Legacy" duration={0} />
        </Sonner>
    }
}

#[test]
async fn deprecated_sonner_still_renders() {
    let _app = yew::Renderer::<LegacySonnerHarness>::with_root(mount_root("toast-legacy")).render();
    settle().await;
    assert_eq!(text("#toast-legacy .sonner-toast .toast-title"), "Legacy");
    assert!(
        query("#toast-legacy .sonner")
            .class_list()
            .contains("toaster-top-center")
    );
}
