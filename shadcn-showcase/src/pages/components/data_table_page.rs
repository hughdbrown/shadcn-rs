//! DataTable component showcase page

use shadcn_rs::{DataTable, DataTableColumn, SelectionMode, SortDirection};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[derive(Clone, PartialEq)]
struct Invoice {
    status: &'static str,
    email: &'static str,
    amount: u32,
}

#[function_component(DataTablePage)]
pub fn data_table_page() -> Html {
    let selected_rows = use_state(Vec::<usize>::new);
    let invoices = vec![
        Invoice {
            status: "Success",
            email: "ken99@example.com",
            amount: 316,
        },
        Invoice {
            status: "Processing",
            email: "abe45@example.com",
            amount: 242,
        },
        Invoice {
            status: "Failed",
            email: "monserrat44@example.com",
            amount: 837,
        },
        Invoice {
            status: "Pending",
            email: "curtis11@example.com",
            amount: 421,
        },
    ];

    let columns = vec![
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
            accessor: Callback::from(|invoice: Invoice| {
                AttrValue::from(invoice.amount.to_string())
            }),
            cell: Some(Callback::from(
                |invoice: Invoice| html! { <span>{ format!("${}.00", invoice.amount) }</span> },
            )),
            sortable: true,
            searchable: false,
            class: classes!("text-right"),
        },
    ];

    let examples = vec![Example {
        title: "Columns, Sorting, Filtering, And Pagination",
        description: "The table now uses explicit column definitions instead of raw row/header render callbacks.",
        demo: html! {
            <div class="space-y-3">
                <p class="text-sm text-muted-foreground">
                    { format!("Selected rows: {:?}", *selected_rows) }
                </p>
                <DataTable<Invoice>
                    columns={columns.clone()}
                    data={invoices}
                    sortable={true}
                    filterable={true}
                    selectable={true}
                    selection_mode={SelectionMode::Multiple}
                    paginated={true}
                    rows_per_page={2}
                    default_sort_column={Some(AttrValue::from("email"))}
                    default_sort_direction={SortDirection::Ascending}
                    selected={Some((*selected_rows).clone())}
                    on_selection_change={Some({
                        let selected_rows = selected_rows.clone();
                        Callback::from(move |indices: Vec<usize>| selected_rows.set(indices))
                    })}
                />
            </div>
        },
        code: r#"<DataTable<Invoice>
    columns={columns}
    data={invoices}
    sortable={true}
    filterable={true}
    selectable={true}
    selection_mode={SelectionMode::Multiple}
    paginated={true}
    rows_per_page={2}
    default_sort_column={Some("email".into())}
/>"#,
    }];

    let props = vec![
        PropDoc {
            name: "columns",
            prop_type: "Vec<DataTableColumn<T>>",
            default: "-",
            description: "Explicit column definitions that control headers, sorting/filtering participation, and per-cell rendering.",
        },
        PropDoc {
            name: "default_sort_column",
            prop_type: "Option<AttrValue>",
            default: "None",
            description: "Initial sortable column ID.",
        },
        PropDoc {
            name: "default_sort_direction",
            prop_type: "SortDirection",
            default: "SortDirection::Ascending",
            description: "Initial sort order for the active sort column.",
        },
        PropDoc {
            name: "filterable",
            prop_type: "bool",
            default: "false",
            description: "Shows a table-level filter input that searches across columns marked `searchable`.",
        },
        PropDoc {
            name: "selection_mode",
            prop_type: "SelectionMode",
            default: "SelectionMode::None",
            description: "Controls whether row selection is disabled, single-select, or multi-select.",
        },
    ];

    let notes = html! {
        <div class="space-y-3">
            <p>{ "Use `DataTableColumn::text` for straightforward text columns, then switch to a custom `cell` renderer only when the display markup diverges from the accessor text." }</p>
            <p>{ "The accessor value is reused for sorting and filtering, which keeps behavior predictable without duplicating column metadata." }</p>
        </div>
    };

    html! {
        <ComponentPage
            name="Data Table"
            description="A column-driven data table with explicit sorting, filtering, selection, and pagination behavior."
            {examples}
            {props}
            notes={Some(notes)}
        />
    }
}
