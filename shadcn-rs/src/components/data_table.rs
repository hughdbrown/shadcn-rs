//! Data Table component
//!
//! Column-driven data table with sorting, filtering, pagination, and selection.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{DataTable, DataTableColumn, SelectionMode, SortDirection};
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
//!         User { id: 1, name: "Jane".into(), email: "jane@example.com".into() },
//!         User { id: 2, name: "John".into(), email: "john@example.com".into() },
//!     ];
//!
//!     let columns = vec![
//!         DataTableColumn::text(
//!             "name",
//!             "Name",
//!             Callback::from(|user: User| AttrValue::from(user.name)),
//!         ),
//!         DataTableColumn::text(
//!             "email",
//!             "Email",
//!             Callback::from(|user: User| AttrValue::from(user.email)),
//!         ),
//!     ];
//!
//!     html! {
//!         <DataTable<User>
//!             columns={columns}
//!             data={data}
//!             sortable={true}
//!             selectable={true}
//!             selection_mode={SelectionMode::Multiple}
//!             default_sort_column={Some(AttrValue::from("name"))}
//!             default_sort_direction={SortDirection::Ascending}
//!         />
//!     }
//! }
//! ```

use yew::prelude::*;

/// Sort direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    /// Ascending order.
    Ascending,
    /// Descending order.
    Descending,
}

impl SortDirection {
    fn toggled(self) -> Self {
        match self {
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }

    fn aria_value(self) -> &'static str {
        match self {
            SortDirection::Ascending => "ascending",
            SortDirection::Descending => "descending",
        }
    }
}

/// Selection mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionMode {
    /// No row selection.
    None,
    /// Exactly one row may be selected.
    Single,
    /// Any number of rows may be selected.
    Multiple,
}

/// Column metadata for [`DataTable`].
///
/// Use [`DataTableColumn::text`] for the common case of a sortable, searchable
/// text column. Construct the struct directly to opt out of those behaviours
/// or to provide a custom `cell` renderer that diverges from the accessor
/// text.
pub struct DataTableColumn<T: Clone + PartialEq + 'static> {
    /// Stable identifier used by `default_sort_column` and aria attributes.
    pub id: AttrValue,
    /// Header label rendered in the `<th>` cell.
    pub header: AttrValue,
    /// Extracts the canonical text for sorting and filtering.
    pub accessor: Callback<T, AttrValue>,
    /// Optional custom cell renderer; falls back to the accessor text.
    pub cell: Option<Callback<T, Html>>,
    /// Whether the user can change the active sort by clicking this column.
    pub sortable: bool,
    /// Whether the table-level filter input searches this column.
    pub searchable: bool,
    /// Extra classes applied to both the header and body cells.
    pub class: Classes,
}

impl<T: Clone + PartialEq + 'static> DataTableColumn<T> {
    /// Build a sortable, searchable text column with no custom cell renderer.
    pub fn text(
        id: impl Into<AttrValue>,
        header: impl Into<AttrValue>,
        accessor: Callback<T, AttrValue>,
    ) -> Self {
        Self {
            id: id.into(),
            header: header.into(),
            accessor,
            cell: None,
            sortable: true,
            searchable: true,
            class: Classes::new(),
        }
    }
}

impl<T: Clone + PartialEq + 'static> Clone for DataTableColumn<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id.clone(),
            header: self.header.clone(),
            accessor: self.accessor.clone(),
            cell: self.cell.clone(),
            sortable: self.sortable,
            searchable: self.searchable,
            class: self.class.clone(),
        }
    }
}

impl<T: Clone + PartialEq + 'static> PartialEq for DataTableColumn<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.header == other.header
            && self.accessor == other.accessor
            && self.cell == other.cell
            && self.sortable == other.sortable
            && self.searchable == other.searchable
            && self.class == other.class
    }
}

/// Data table component properties.
#[derive(Properties, PartialEq)]
pub struct DataTableProps<T: Clone + PartialEq + 'static> {
    /// Column definitions controlling headers, accessors, and rendering.
    pub columns: Vec<DataTableColumn<T>>,

    /// Row data.
    pub data: Vec<T>,

    /// Enable sorting affordances on sortable columns.
    #[prop_or(false)]
    pub sortable: bool,

    /// Enable the filter input that searches across `searchable` columns.
    #[prop_or(false)]
    pub filterable: bool,

    /// Enable row selection.
    #[prop_or(false)]
    pub selectable: bool,

    /// Selection mode. Leave at `SelectionMode::None` and checkboxes render
    /// read-only; set `Single` or `Multiple` alongside `selectable` to let
    /// users toggle rows.
    #[prop_or(SelectionMode::None)]
    pub selection_mode: SelectionMode,

    /// Controlled selection state. When `Some`, the parent owns selection.
    #[prop_or_default]
    pub selected: Option<Vec<usize>>,

    /// Selection change handler. Receives the new selection.
    #[prop_or_default]
    pub on_selection_change: Option<Callback<Vec<usize>>>,

    /// Enable client-side pagination.
    #[prop_or(false)]
    pub paginated: bool,

    /// Rows per page when pagination is enabled.
    #[prop_or(10)]
    pub rows_per_page: usize,

    /// Initial sort column id.
    #[prop_or_default]
    pub default_sort_column: Option<AttrValue>,

    /// Initial sort direction.
    #[prop_or(SortDirection::Ascending)]
    pub default_sort_direction: SortDirection,

    /// Additional CSS classes on the root element.
    #[prop_or_default]
    pub class: Classes,
}

/// Orders cell values numerically when both parse as numbers, else as text,
/// so "9" sorts before "10".
fn compare_cell_values(a: &str, b: &str) -> std::cmp::Ordering {
    match (a.trim().parse::<f64>(), b.trim().parse::<f64>()) {
        (Ok(x), Ok(y)) => x.total_cmp(&y),
        _ => a.cmp(b),
    }
}

/// Data table component.
///
/// Feature-rich table for displaying and manipulating tabular data.
///
/// # Accessibility
/// - ARIA grid role
/// - Column headers expose `aria-sort` for the active column
/// - Row selection uses `aria-selected`
#[function_component(DataTable)]
pub fn data_table<T: Clone + PartialEq + 'static>(props: &DataTableProps<T>) -> Html {
    let sort_column = use_state(|| props.default_sort_column.clone());
    let sort_direction = use_state(|| props.default_sort_direction);
    let filter = use_state(|| AttrValue::from(""));
    let page = use_state(|| 0usize);
    let internal_selected = use_state(Vec::<usize>::new);

    let current_selection: Vec<usize> = props
        .selected
        .clone()
        .unwrap_or_else(|| (*internal_selected).clone());

    // Filter
    let filter_text = filter.to_lowercase();
    let mut visible: Vec<usize> = props
        .data
        .iter()
        .enumerate()
        .filter(|(_, item)| {
            if !props.filterable || filter_text.is_empty() {
                return true;
            }
            props.columns.iter().any(|col| {
                col.searchable
                    && col
                        .accessor
                        .emit((*item).clone())
                        .to_lowercase()
                        .contains(&filter_text)
            })
        })
        .map(|(i, _)| i)
        .collect();

    // Sort
    if props.sortable
        && let Some(active_id) = sort_column.as_ref()
        && let Some(col) = props.columns.iter().find(|c| &c.id == active_id)
    {
        // Compute each key once rather than twice per comparison.
        let mut keyed: Vec<(AttrValue, usize)> = visible
            .into_iter()
            .map(|index| (col.accessor.emit(props.data[index].clone()), index))
            .collect();
        keyed.sort_by(|(a, _), (b, _)| {
            let ord = compare_cell_values(a, b);
            match *sort_direction {
                SortDirection::Ascending => ord,
                SortDirection::Descending => ord.reverse(),
            }
        });
        visible = keyed.into_iter().map(|(_, index)| index).collect();
    }

    // Pagination
    let total_pages = if props.paginated && props.rows_per_page > 0 {
        visible.len().div_ceil(props.rows_per_page).max(1)
    } else {
        1
    };
    let current_page = (*page).min(total_pages.saturating_sub(1));
    let visible_on_page: Vec<usize> = if props.paginated && props.rows_per_page > 0 {
        let start = current_page * props.rows_per_page;
        let end = (start + props.rows_per_page).min(visible.len());
        visible[start..end].to_vec()
    } else {
        visible.clone()
    };

    let on_filter_input = {
        let filter = filter.clone();
        let page = page.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            filter.set(AttrValue::from(input.value()));
            page.set(0);
        })
    };

    let toggle_selection = {
        let internal_selected = internal_selected.clone();
        let on_selection_change = props.on_selection_change.clone();
        let selected_external = props.selected.clone();
        let selection_mode = props.selection_mode;
        Callback::from(move |idx: usize| {
            let current: Vec<usize> = selected_external
                .clone()
                .unwrap_or_else(|| (*internal_selected).clone());
            let next = match selection_mode {
                SelectionMode::None => current,
                SelectionMode::Single => {
                    if current.contains(&idx) {
                        Vec::new()
                    } else {
                        vec![idx]
                    }
                }
                SelectionMode::Multiple => {
                    if let Some(pos) = current.iter().position(|i| *i == idx) {
                        let mut next = current.clone();
                        next.remove(pos);
                        next
                    } else {
                        let mut next = current.clone();
                        next.push(idx);
                        next
                    }
                }
            };
            if selected_external.is_none() {
                internal_selected.set(next.clone());
            }
            if let Some(cb) = &on_selection_change {
                cb.emit(next);
            }
        })
    };

    let select_all_toggle = {
        let internal_selected = internal_selected.clone();
        let on_selection_change = props.on_selection_change.clone();
        let selected_external = props.selected.clone();
        let visible_indices = visible.clone();
        let selection_mode = props.selection_mode;
        Callback::from(move |_| {
            if selection_mode != SelectionMode::Multiple {
                return;
            }
            let current: Vec<usize> = selected_external
                .clone()
                .unwrap_or_else(|| (*internal_selected).clone());
            let all_selected =
                !visible_indices.is_empty() && visible_indices.iter().all(|i| current.contains(i));
            let next: Vec<usize> = if all_selected {
                current
                    .into_iter()
                    .filter(|i| !visible_indices.contains(i))
                    .collect()
            } else {
                let mut next = current.clone();
                for i in &visible_indices {
                    if !next.contains(i) {
                        next.push(*i);
                    }
                }
                next
            };
            if selected_external.is_none() {
                internal_selected.set(next.clone());
            }
            if let Some(cb) = &on_selection_change {
                cb.emit(next);
            }
        })
    };

    // Step from the clamped page: the stored page can exceed `total_pages`
    // after the data shrinks.
    let prev_page = {
        let page = page.clone();
        Callback::from(move |_| page.set(current_page.saturating_sub(1)))
    };

    let next_page_cb = {
        let page = page.clone();
        Callback::from(move |_| {
            if current_page + 1 < total_pages {
                page.set(current_page + 1);
            }
        })
    };

    let classes: Classes = vec![
        Classes::from("data-table"),
        if props.sortable {
            Classes::from("data-table-sortable")
        } else {
            Classes::new()
        },
        if props.selectable {
            Classes::from("data-table-selectable")
        } else {
            Classes::new()
        },
        props.class.clone(),
    ]
    .into_iter()
    .collect();

    let active_sort = sort_column.as_ref().cloned();
    let active_dir = *sort_direction;

    let header_cells = props
        .columns
        .iter()
        .map(|col| {
            let is_active = active_sort.as_ref() == Some(&col.id);
            let aria_sort: Option<&'static str> = if is_active {
                Some(active_dir.aria_value())
            } else {
                None
            };
            let header_class = col.class.clone();
            let show_button = props.sortable && col.sortable;
            let header_label = col.header.clone();

            let on_sort = {
                let sort_column = sort_column.clone();
                let sort_direction = sort_direction.clone();
                let column_id = col.id.clone();
                Callback::from(move |_: MouseEvent| {
                    if sort_column.as_ref() == Some(&column_id) {
                        sort_direction.set(sort_direction.toggled());
                    } else {
                        sort_column.set(Some(column_id.clone()));
                        sort_direction.set(SortDirection::Ascending);
                    }
                })
            };

            html! {
                <th class={header_class} aria-sort={aria_sort}>
                    if show_button {
                        <button
                            type="button"
                            class="data-table-sort-button"
                            onclick={on_sort}
                        >
                            { header_label }
                            if is_active {
                                <span class="data-table-sort-indicator">
                                    { match active_dir {
                                        SortDirection::Ascending => " \u{25B2}",
                                        SortDirection::Descending => " \u{25BC}",
                                    } }
                                </span>
                            }
                        </button>
                    } else {
                        { header_label }
                    }
                </th>
            }
        })
        .collect::<Html>();

    let body = if visible_on_page.is_empty() {
        let colspan = props.columns.len() + if props.selectable { 1 } else { 0 };
        html! {
            <tr>
                <td class="data-table-empty" colspan={colspan.to_string()}>
                    { "No results." }
                </td>
            </tr>
        }
    } else {
        visible_on_page
            .iter()
            .map(|&original_idx| {
                let item = props.data[original_idx].clone();
                let is_selected = current_selection.contains(&original_idx);
                let mut row_class = Classes::from("data-table-row");
                if is_selected {
                    row_class.push("data-table-row-selected");
                }
                let on_select = {
                    let toggle_selection = toggle_selection.clone();
                    let idx = original_idx;
                    Callback::from(move |_: MouseEvent| toggle_selection.emit(idx))
                };
                let cells = props.columns.iter().map(|col| {
                    let cell_content = if let Some(renderer) = &col.cell {
                        renderer.emit(item.clone())
                    } else {
                        let text = col.accessor.emit(item.clone());
                        html! { { text } }
                    };
                    html! { <td class={col.class.clone()}>{ cell_content }</td> }
                }).collect::<Html>();

                html! {
                    <tr key={original_idx} role="row" class={row_class} aria-selected={is_selected.to_string()}>
                        if props.selectable {
                            <td class="data-table-select-cell">
                                <input
                                    type="checkbox"
                                    aria-label={format!("Select row {}", original_idx + 1)}
                                    checked={is_selected}
                                    onclick={on_select}
                                />
                            </td>
                        }
                        { cells }
                    </tr>
                }
            })
            .collect::<Html>()
    };

    let select_all_checked =
        !visible.is_empty() && visible.iter().all(|i| current_selection.contains(i));

    html! {
        <div class={classes} role="grid">
            if props.filterable {
                <div class="data-table-toolbar">
                    <input
                        type="text"
                        class="data-table-filter"
                        placeholder="Filter..."
                        value={(*filter).clone()}
                        oninput={on_filter_input}
                    />
                </div>
            }
            <table class="data-table-table">
                <thead class="data-table-header">
                    <tr>
                        if props.selectable {
                            <th class="data-table-select-column">
                                if props.selection_mode == SelectionMode::Multiple {
                                    <input
                                        type="checkbox"
                                        aria-label="Select all"
                                        onclick={select_all_toggle}
                                        checked={select_all_checked}
                                    />
                                }
                            </th>
                        }
                        { header_cells }
                    </tr>
                </thead>
                <tbody class="data-table-body">
                    { body }
                </tbody>
            </table>
            if props.paginated && total_pages > 1 {
                <div class="data-table-pagination">
                    <span>{ format!("Page {} of {}", current_page + 1, total_pages) }</span>
                    <div class="data-table-pagination-actions">
                        <button
                            type="button"
                            class="data-table-page-button"
                            onclick={prev_page}
                            disabled={current_page == 0}
                        >
                            { "Previous" }
                        </button>
                        <button
                            type="button"
                            class="data-table-page-button"
                            onclick={next_page_cb}
                            disabled={current_page + 1 >= total_pages}
                        >
                            { "Next" }
                        </button>
                    </div>
                </div>
            }
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

    fn name_accessor() -> Callback<TestData, AttrValue> {
        Callback::from(|d: TestData| AttrValue::from(d.name))
    }

    #[test]
    fn text_helper_defaults_to_sortable_and_searchable() {
        let col: DataTableColumn<TestData> = DataTableColumn::text("name", "Name", name_accessor());
        assert!(col.sortable);
        assert!(col.searchable);
        assert!(col.cell.is_none());
        assert_eq!(col.id, "name");
        assert_eq!(col.header, "Name");
    }

    #[test]
    fn compare_cell_values_orders_numbers_numerically() {
        use std::cmp::Ordering;
        assert_eq!(compare_cell_values("9", "10"), Ordering::Less);
        assert_eq!(compare_cell_values("-1.5", "2"), Ordering::Less);
        assert_eq!(compare_cell_values("apple", "banana"), Ordering::Less);
        assert_eq!(compare_cell_values("10", "abc"), Ordering::Less);
    }

    #[test]
    fn sort_direction_toggles() {
        assert_eq!(
            SortDirection::Ascending.toggled(),
            SortDirection::Descending
        );
        assert_eq!(
            SortDirection::Descending.toggled(),
            SortDirection::Ascending
        );
    }

    #[test]
    fn props_accept_column_driven_config() {
        let columns = vec![DataTableColumn::text("name", "Name", name_accessor())];
        let data = vec![TestData {
            id: 1,
            name: "Test".into(),
        }];
        let props = DataTableProps {
            columns,
            data,
            sortable: true,
            filterable: true,
            selectable: true,
            selection_mode: SelectionMode::Multiple,
            selected: None,
            on_selection_change: None,
            paginated: true,
            rows_per_page: 5,
            default_sort_column: Some(AttrValue::from("name")),
            default_sort_direction: SortDirection::Ascending,
            class: Classes::new(),
        };
        assert!(props.sortable);
        assert_eq!(props.selection_mode, SelectionMode::Multiple);
        assert_eq!(props.rows_per_page, 5);
    }
}
