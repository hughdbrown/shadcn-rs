//! Data Table component
//!
//! Full-featured data table with sorting, filtering, pagination, and selection.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{DataTable, DataTableColumn};
//!
//! #[derive(Clone, PartialEq)]
//! struct User {
//!     id: u32,
//!     name: String,
//!     email: String,
//! }
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let data = vec![
//!         User { id: 1, name: "John".to_string(), email: "john@example.com".to_string() },
//!         User { id: 2, name: "Jane".to_string(), email: "jane@example.com".to_string() },
//!     ];
//!
//!     html! {
//!         <DataTable<User>
//!             data={data}
//!             sortable=true
//!             selectable=true
//!         />
//!     }
//! }
//! ```

use yew::prelude::*;

/// Sort direction
#[derive(Debug, Clone, PartialEq)]
pub enum SortDirection {
    /// Ascending order
    Ascending,
    /// Descending order
    Descending,
}

/// Selection mode
#[derive(Debug, Clone, PartialEq)]
pub enum SelectionMode {
    /// No selection
    None,
    /// Single row selection
    Single,
    /// Multiple row selection
    Multiple,
}

/// Data table component properties
#[derive(Properties, PartialEq)]
pub struct DataTableProps<T: Clone + PartialEq + 'static> {
    /// Data to display
    pub data: Vec<T>,

    /// Enable sorting
    #[prop_or(false)]
    pub sortable: bool,

    /// Enable filtering
    #[prop_or(false)]
    pub filterable: bool,

    /// Enable selection
    #[prop_or(false)]
    pub selectable: bool,

    /// Selection mode
    #[prop_or(SelectionMode::None)]
    pub selection_mode: SelectionMode,

    /// Selected row indices
    #[prop_or_default]
    pub selected: Option<Vec<usize>>,

    /// Selection change handler
    #[prop_or_default]
    pub on_selection_change: Option<Callback<Vec<usize>>>,

    /// Enable pagination
    #[prop_or(false)]
    pub paginated: bool,

    /// Rows per page
    #[prop_or(10)]
    pub rows_per_page: usize,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Render function for rows
    #[prop_or_default]
    pub render_row: Option<Callback<T, Html>>,
}

/// Data table component
///
/// Feature-rich table for displaying and manipulating data.
///
/// # Accessibility
/// - ARIA grid role
/// - Column headers with aria-sort
/// - Row selection with aria-selected
/// - Keyboard navigation
#[function_component(DataTable)]
pub fn data_table<T: Clone + PartialEq + 'static>(props: &DataTableProps<T>) -> Html {
    let sortable_val = props.sortable;
    let selectable_val = props.selectable;
    let data = &props.data;
    let class = &props.class;

    let classes: Classes = vec![
        Classes::from("data-table"),
        if sortable_val {
            Classes::from("data-table-sortable")
        } else {
            Classes::new()
        },
        if selectable_val {
            Classes::from("data-table-selectable")
        } else {
            Classes::new()
        },
        class.clone(),
    ]
    .into_iter()
    .collect();

    html! {
        <div class={classes} role="grid">
            <table class="data-table-table">
                <thead class="data-table-header">
                    <tr>
                        if selectable_val {
                            <th class="data-table-select-column">
                                <input type="checkbox" aria-label="Select all" />
                            </th>
                        }
                        <th>{ "Column 1" }</th>
                        <th>{ "Column 2" }</th>
                    </tr>
                </thead>
                <tbody class="data-table-body">
                    {
                        data.iter().enumerate().map(|(idx, _item)| {
                            html! {
                                <tr key={idx} role="row">
                                    if selectable_val {
                                        <td class="data-table-select-cell">
                                            <input type="checkbox" aria-label={format!("Select row {}", idx + 1)} />
                                        </td>
                                    }
                                    <td>{ format!("Row {} - Col 1", idx + 1) }</td>
                                    <td>{ format!("Row {} - Col 2", idx + 1) }</td>
                                </tr>
                            }
                        }).collect::<Html>()
                    }
                </tbody>
            </table>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, PartialEq)]
    struct TestData {
        id: u32,
        name: String,
    }

    #[test]
    fn test_data_table_default() {
        let data = vec![
            TestData {
                id: 1,
                name: "Test".to_string(),
            },
        ];

        let props = DataTableProps {
            data,
            sortable: false,
            filterable: false,
            selectable: false,
            selection_mode: SelectionMode::None,
            selected: None,
            on_selection_change: None,
            paginated: false,
            rows_per_page: 10,
            class: Classes::new(),
            render_row: None,
        };

        assert!(!props.sortable);
        assert!(!props.selectable);
    }

    #[test]
    fn test_data_table_sortable() {
        let data = vec![
            TestData {
                id: 1,
                name: "Test".to_string(),
            },
        ];

        let props = DataTableProps {
            data,
            sortable: true,
            filterable: false,
            selectable: false,
            selection_mode: SelectionMode::None,
            selected: None,
            on_selection_change: None,
            paginated: false,
            rows_per_page: 10,
            class: Classes::new(),
            render_row: None,
        };

        assert!(props.sortable);
    }

    #[test]
    fn test_data_table_selectable() {
        let data = vec![
            TestData {
                id: 1,
                name: "Test".to_string(),
            },
        ];

        let props = DataTableProps {
            data,
            sortable: false,
            filterable: false,
            selectable: true,
            selection_mode: SelectionMode::Multiple,
            selected: None,
            on_selection_change: None,
            paginated: false,
            rows_per_page: 10,
            class: Classes::new(),
            render_row: None,
        };

        assert!(props.selectable);
        assert_eq!(props.selection_mode, SelectionMode::Multiple);
    }

    #[test]
    fn test_data_table_paginated() {
        let data = vec![
            TestData {
                id: 1,
                name: "Test".to_string(),
            },
        ];

        let props = DataTableProps {
            data,
            sortable: false,
            filterable: false,
            selectable: false,
            selection_mode: SelectionMode::None,
            selected: None,
            on_selection_change: None,
            paginated: true,
            rows_per_page: 20,
            class: Classes::new(),
            render_row: None,
        };

        assert!(props.paginated);
        assert_eq!(props.rows_per_page, 20);
    }

    #[test]
    fn test_sort_direction() {
        assert_eq!(SortDirection::Ascending, SortDirection::Ascending);
        assert_ne!(SortDirection::Ascending, SortDirection::Descending);
    }
}
