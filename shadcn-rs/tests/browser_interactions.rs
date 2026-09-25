#![cfg(target_arch = "wasm32")]

use wasm_bindgen::JsCast;
use wasm_bindgen_test::{wasm_bindgen_test as test, wasm_bindgen_test_configure};
use yew::prelude::*;

use shadcn_rs::{
    Calendar, CalendarMode, Carousel, CarouselContent, CarouselItem, CarouselNext,
    CarouselPrevious, DataTable, DataTableColumn, Dialog, DialogContent, DialogTrigger,
    SelectionMode, SortDirection,
};
use shadcn_rs::{Resizable, ResizableHandle, ResizableOrientation, ResizablePanel};

mod utils;

use utils::{
    active_id, click, click_nth, focus, inject_component_styles, input, keydown, mount_root, query,
    settle, text,
};

wasm_bindgen_test_configure!(run_in_browser);

#[function_component(DialogHarness)]
fn dialog_harness() -> Html {
    let open = use_state(|| false);
    let on_open_change = {
        let open = open.clone();
        Callback::from(move |value: bool| open.set(value))
    };

    html! {
        <Dialog open={*open} on_open_change={Some(on_open_change)}>
            <DialogTrigger>
                <button id="open-dialog" type="button">{ "Open dialog" }</button>
            </DialogTrigger>
            <DialogContent>
                <button id="first-action" type="button">{ "First" }</button>
                <button id="second-action" type="button">{ "Second" }</button>
            </DialogContent>
        </Dialog>
    }
}

#[test]
async fn dialog_focus_traps_and_restores() {
    let root = mount_root("dialog-test");
    let _app = yew::Renderer::<DialogHarness>::with_root(root).render();

    settle().await;
    // Programmatic `.click()` does not move focus; a keyboard user activating
    // the trigger would already have it focused.
    focus("#dialog-test #open-dialog");
    click("#dialog-test #open-dialog");
    settle().await;

    // An aria-hidden ancestor would hide the whole dialog from screen readers.
    assert!(
        query("[role='dialog']")
            .closest("[aria-hidden='true']")
            .expect("closest selector failed")
            .is_none(),
        "dialog must not be inside an aria-hidden subtree"
    );
    assert_eq!(active_id(), "first-action");

    keydown(".dialog-content", "Tab", false);
    settle().await;
    assert_eq!(active_id(), "second-action");

    keydown(".dialog-content", "Tab", false);
    settle().await;
    assert_eq!(active_id(), "first-action");

    keydown(".dialog-content", "Escape", false);
    settle().await;
    assert_eq!(active_id(), "open-dialog");
}

#[function_component(CarouselHarness)]
fn carousel_harness() -> Html {
    let current = use_state(|| 0usize);
    let on_slide_change = {
        let current = current.clone();
        Callback::from(move |index: usize| current.set(index))
    };

    html! {
        <div>
            <div id="carousel-index">{ current.to_string() }</div>
            <Carousel
                current={Some(*current)}
                on_slide_change={Some(on_slide_change)}
                loop_slides={false}
                class="w-full"
            >
                <CarouselContent>
                    <CarouselItem>{ "Alpha" }</CarouselItem>
                    <CarouselItem>{ "Beta" }</CarouselItem>
                    <CarouselItem>{ "Gamma" }</CarouselItem>
                </CarouselContent>
                <CarouselPrevious />
                <CarouselNext />
            </Carousel>
        </div>
    }
}

#[test]
async fn carousel_navigation_updates_state() {
    let root = mount_root("carousel-test");
    let _app = yew::Renderer::<CarouselHarness>::with_root(root).render();

    settle().await;
    assert_eq!(text("#carousel-test #carousel-index"), "0");

    click("#carousel-test .carousel-next");
    settle().await;
    assert_eq!(text("#carousel-test #carousel-index"), "1");

    click("#carousel-test .carousel-indicator:nth-of-type(3)");
    settle().await;
    assert_eq!(text("#carousel-test #carousel-index"), "2");

    click("#carousel-test .carousel-previous");
    settle().await;
    assert_eq!(text("#carousel-test #carousel-index"), "1");
}

#[function_component(CalendarRangeHarness)]
fn calendar_range_harness() -> Html {
    let selection = use_state(|| Some(String::from("2026-04-17")));
    let onselect = {
        let selection = selection.clone();
        Callback::from(move |value: String| selection.set(Some(value)))
    };

    html! {
        <div>
            <div id="calendar-value">{ selection.as_deref().unwrap_or("none").to_string() }</div>
            <Calendar
                mode={CalendarMode::Range}
                selected={(*selection).clone()}
                onselect={Some(onselect)}
            />
        </div>
    }
}

#[test]
async fn calendar_range_selection_updates_value() {
    let root = mount_root("calendar-test");
    let _app = yew::Renderer::<CalendarRangeHarness>::with_root(root).render();

    settle().await;
    click("#calendar-test [aria-label='April 20, 2026']");
    settle().await;

    assert_eq!(
        text("#calendar-test #calendar-value"),
        "2026-04-17..2026-04-20"
    );
}

#[derive(Clone, PartialEq)]
struct Invoice {
    status: &'static str,
    email: &'static str,
    amount: u32,
}

#[function_component(DataTableHarness)]
fn data_table_harness() -> Html {
    let selected_rows = use_state(Vec::<usize>::new);
    let data = vec![
        Invoice {
            status: "Processing",
            email: "zeta@example.com",
            amount: 200,
        },
        Invoice {
            status: "Paid",
            email: "alpha@example.com",
            amount: 500,
        },
        Invoice {
            status: "Failed",
            email: "mango@example.com",
            amount: 100,
        },
    ];

    html! {
        <div>
            <div id="selection-count">{ selected_rows.len().to_string() }</div>
            <DataTable<Invoice>
                columns={vec![
                    DataTableColumn::text(
                        "status",
                        "Status",
                        Callback::from(|invoice: Invoice| AttrValue::from(invoice.status)),
                    ),
                    DataTableColumn::text(
                        "email",
                        "Email",
                        Callback::from(|invoice: Invoice| AttrValue::from(invoice.email)),
                    ),
                    DataTableColumn {
                        id: "amount".into(),
                        header: "Amount".into(),
                        accessor: Callback::from(|invoice: Invoice| AttrValue::from(invoice.amount.to_string())),
                        cell: Some(Callback::from(|invoice: Invoice| html! { <span>{ invoice.amount }</span> })),
                        sortable: true,
                        searchable: false,
                        class: Classes::new(),
                    },
                ]}
                data={data}
                sortable={true}
                filterable={true}
                selectable={true}
                selection_mode={SelectionMode::Single}
                paginated={true}
                rows_per_page={1}
                default_sort_column={Some(AttrValue::from("email"))}
                default_sort_direction={SortDirection::Ascending}
                selected={Some((*selected_rows).clone())}
                on_selection_change={Some({
                    let selected_rows = selected_rows.clone();
                    Callback::from(move |indices: Vec<usize>| selected_rows.set(indices))
                })}
            />
        </div>
    }
}

#[test]
async fn data_table_filter_sort_paginate_and_select() {
    let root = mount_root("table-test");
    let _app = yew::Renderer::<DataTableHarness>::with_root(root).render();

    settle().await;
    assert!(text("#table-test .data-table-body").contains("alpha@example.com"));

    // Email is already the ascending sort column; clicking its header flips it.
    click_nth("#table-test .data-table-sort-button", 1);
    settle().await;
    assert!(text("#table-test .data-table-body").contains("zeta@example.com"));

    input("#table-test .data-table-filter", "mango");
    settle().await;
    assert!(text("#table-test .data-table-body").contains("mango@example.com"));

    let row_checkbox = query("#table-test .data-table-select-cell input");
    row_checkbox
        .dyn_into::<web_sys::HtmlInputElement>()
        .expect("row checkbox should be an input")
        .click();
    settle().await;
    assert_eq!(text("#table-test #selection-count"), "1");
}

#[function_component(VerticalResizableHarness)]
fn vertical_resizable_harness() -> Html {
    html! {
        <Resizable orientation={ResizableOrientation::Vertical}>
            <ResizablePanel index={0}>{ "Top" }</ResizablePanel>
            <ResizableHandle />
            <ResizablePanel index={1}>{ "Bottom" }</ResizablePanel>
        </Resizable>
    }
}

fn mouse(target: &web_sys::EventTarget, kind: &str, x: f64, y: f64) {
    let init = web_sys::MouseEventInit::new();
    init.set_bubbles(true);
    init.set_cancelable(true);
    init.set_client_x(x as i32);
    init.set_client_y(y as i32);
    let event = web_sys::MouseEvent::new_with_mouse_event_init_dict(kind, &init)
        .expect("failed to create mouse event");
    target
        .dispatch_event(&event)
        .expect("failed to dispatch mouse event");
}

#[test]
async fn vertical_resizable_responds_to_dragging() {
    inject_component_styles();
    let root = mount_root("resizable-test");
    let _app = yew::Renderer::<VerticalResizableHarness>::with_root(root).render();
    settle().await;

    let group = query("#resizable-test .resizable");
    let rect = group.get_bounding_client_rect();
    let x = rect.left() + rect.width() / 2.0;
    let y = rect.top() + rect.height() * 0.3;
    let window: web_sys::EventTarget = gloo::utils::window().into();

    mouse(
        &query("#resizable-test .resizable-handle"),
        "mousedown",
        x,
        y,
    );
    mouse(&window, "mousemove", x, y);
    mouse(&window, "mouseup", x, y);
    settle().await;

    let heights: Vec<f64> = ["[data-panel-index='0']", "[data-panel-index='1']"]
        .iter()
        .map(|panel| {
            query(&format!("#resizable-test {panel}"))
                .get_bounding_client_rect()
                .height()
        })
        .collect();
    let top_share = heights[0] / (heights[0] + heights[1]);
    assert!(
        (0.25..0.35).contains(&top_share),
        "top panel should take ~30% after dragging, got {top_share:.2} ({heights:?})"
    );
}
