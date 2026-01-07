//! Table component
//!
//! Semantic HTML table with styling support.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Table>
//!             <TableHeader>
//!                 <TableRow>
//!                     <TableHead>{ "Name" }</TableHead>
//!                     <TableHead>{ "Email" }</TableHead>
//!                 </TableRow>
//!             </TableHeader>
//!             <TableBody>
//!                 <TableRow>
//!                     <TableCell>{ "John Doe" }</TableCell>
//!                     <TableCell>{ "john@example.com" }</TableCell>
//!                 </TableRow>
//!             </TableBody>
//!         </Table>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Table container properties
#[derive(Properties, PartialEq, Clone)]
pub struct TableProps {
    /// Table caption
    #[prop_or_default]
    pub caption: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Table container component
///
/// Semantic table with consistent styling.
///
/// # Accessibility
/// - Uses semantic `<table>` element
/// - Supports caption for screen readers
/// - Proper heading/cell relationships
#[function_component(Table)]
pub fn table(props: &TableProps) -> Html {
    let TableProps {
        caption,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("table"), class].into_iter().collect();

    html! {
        <div class="table-wrapper">
            <table class={classes}>
                if let Some(caption_text) = caption {
                    <caption class="table-caption">{ caption_text }</caption>
                }
                { children }
            </table>
        </div>
    }
}

/// Table header properties
#[derive(Properties, PartialEq, Clone)]
pub struct TableHeaderProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Table header component
#[function_component(TableHeader)]
pub fn table_header(props: &TableHeaderProps) -> Html {
    let TableHeaderProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("table-header"), class]
        .into_iter()
        .collect();

    html! {
        <thead class={classes}>
            { children }
        </thead>
    }
}

/// Table body properties
#[derive(Properties, PartialEq, Clone)]
pub struct TableBodyProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Table body component
#[function_component(TableBody)]
pub fn table_body(props: &TableBodyProps) -> Html {
    let TableBodyProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("table-body"), class]
        .into_iter()
        .collect();

    html! {
        <tbody class={classes}>
            { children }
        </tbody>
    }
}

/// Table footer properties
#[derive(Properties, PartialEq, Clone)]
pub struct TableFooterProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Table footer component
#[function_component(TableFooter)]
pub fn table_footer(props: &TableFooterProps) -> Html {
    let TableFooterProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("table-footer"), class]
        .into_iter()
        .collect();

    html! {
        <tfoot class={classes}>
            { children }
        </tfoot>
    }
}

/// Table row properties
#[derive(Properties, PartialEq, Clone)]
pub struct TableRowProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Table row component
#[function_component(TableRow)]
pub fn table_row(props: &TableRowProps) -> Html {
    let TableRowProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("table-row"), class]
        .into_iter()
        .collect();

    html! {
        <tr class={classes}>
            { children }
        </tr>
    }
}

/// Table head cell properties
#[derive(Properties, PartialEq, Clone)]
pub struct TableHeadProps {
    /// Column span
    #[prop_or_default]
    pub colspan: Option<u32>,

    /// Row span
    #[prop_or_default]
    pub rowspan: Option<u32>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Table head cell component
#[function_component(TableHead)]
pub fn table_head(props: &TableHeadProps) -> Html {
    let TableHeadProps {
        colspan,
        rowspan,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("table-head"), class]
        .into_iter()
        .collect();

    html! {
        <th
            class={classes}
            colspan={colspan.map(|c| c.to_string())}
            rowspan={rowspan.map(|r| r.to_string())}
        >
            { children }
        </th>
    }
}

/// Table cell properties
#[derive(Properties, PartialEq, Clone)]
pub struct TableCellProps {
    /// Column span
    #[prop_or_default]
    pub colspan: Option<u32>,

    /// Row span
    #[prop_or_default]
    pub rowspan: Option<u32>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Table cell component
#[function_component(TableCell)]
pub fn table_cell(props: &TableCellProps) -> Html {
    let TableCellProps {
        colspan,
        rowspan,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("table-cell"), class]
        .into_iter()
        .collect();

    html! {
        <td
            class={classes}
            colspan={colspan.map(|c| c.to_string())}
            rowspan={rowspan.map(|r| r.to_string())}
        >
            { children }
        </td>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_with_caption() {
        let props = TableProps {
            caption: Some(AttrValue::from("User data")),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.caption, Some(AttrValue::from("User data")));
    }

    #[test]
    fn test_table_without_caption() {
        let props = TableProps {
            caption: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.caption, None);
    }

    #[test]
    fn test_table_head_colspan() {
        let props = TableHeadProps {
            colspan: Some(2),
            rowspan: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.colspan, Some(2));
    }

    #[test]
    fn test_table_cell_rowspan() {
        let props = TableCellProps {
            colspan: None,
            rowspan: Some(3),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.rowspan, Some(3));
    }
}
