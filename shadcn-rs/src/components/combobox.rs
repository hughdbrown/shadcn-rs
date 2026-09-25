//! Combobox component
//!
//! An autocomplete input combined with a list of suggestions (searchable select).
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Combobox, ComboboxTrigger, ComboboxContent, ComboboxInput, ComboboxEmpty, ComboboxGroup, ComboboxItem};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let selected = use_state(|| None::<String>);
//!     let open = use_state(|| false);
//!
//!     html! {
//!         <Combobox>
//!             <ComboboxTrigger>
//!                 <ComboboxInput placeholder="Select framework..." />
//!             </ComboboxTrigger>
//!             <ComboboxContent>
//!                 <ComboboxEmpty>{ "No framework found." }</ComboboxEmpty>
//!                 <ComboboxGroup>
//!                     <ComboboxItem value="react">{ "React" }</ComboboxItem>
//!                     <ComboboxItem value="vue">{ "Vue" }</ComboboxItem>
//!                     <ComboboxItem value="svelte">{ "Svelte" }</ComboboxItem>
//!                 </ComboboxGroup>
//!             </ComboboxContent>
//!         </Combobox>
//!     }
//! }
//! ```

use crate::components::command::{
    matches_query, use_match_registry, use_report_match, use_text_content,
};
use crate::hooks::{use_click_outside_conditional, use_escape_key_conditional};
use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Context for sharing combobox state between parent and children
#[derive(Clone, PartialEq)]
pub struct ComboboxContext {
    /// Current filter/search query
    pub filter_query: String,
    /// Callback to update the filter query
    pub set_filter_query: Callback<String>,
    /// Whether the combobox dropdown is open
    pub is_open: bool,
    /// Callback to toggle open state
    pub toggle: Callback<()>,
    /// Callback to set open state
    pub set_open: Callback<bool>,
    /// Unique ID for the content element (used for aria-controls)
    pub content_id: String,
    /// Currently selected value
    pub value: Option<AttrValue>,
    /// Label (text) of the selected item, once one has been picked
    pub selected_label: Option<AttrValue>,
    /// Commits a selection: `(value, label)`. Closes the popup.
    pub select: Callback<(AttrValue, AttrValue)>,
    /// True while `filter_query` holds the selected label rather than typed
    /// text; items then ignore the filter
    pub query_is_selection: bool,
    /// Number of items that match the current filter
    pub match_count: usize,
    /// Items report `(id, Some(matches))` / `(id, None)` to the root
    pub report_match: Callback<(AttrValue, Option<bool>)>,
}

impl ComboboxContext {
    /// Query items should filter by (empty right after a selection)
    pub fn effective_query(&self) -> &str {
        if self.query_is_selection {
            ""
        } else {
            &self.filter_query
        }
    }
}

/// Combobox component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxProps {
    /// Whether the combobox is open. Later changes are applied too.
    #[prop_or_default]
    pub open: Option<bool>,

    /// Default open state (uncontrolled)
    #[prop_or(false)]
    pub default_open: bool,

    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Selected value (controlled)
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Initially selected value (uncontrolled)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Called with the value of the item the user selects
    #[prop_or_default]
    pub on_value_change: Option<Callback<AttrValue>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Combobox component
///
/// Container for the combobox trigger, input and content. Selecting an item
/// commits its value, shows its label in the trigger and input, and closes
/// the popup. Clicking outside the combobox or pressing Escape closes it.
#[function_component(Combobox)]
pub fn combobox(props: &ComboboxProps) -> Html {
    let ComboboxProps {
        open,
        default_open,
        on_open_change,
        value,
        default_value,
        on_value_change,
        class,
        children,
    } = props.clone();

    let open_state = use_state(|| open.unwrap_or(default_open));
    let is_open = *open_state;
    let filter_query = use_state(String::new);
    let query_is_selection = use_state(|| false);
    let internal_value = use_state(|| default_value);
    let selected_label = use_state(|| None::<AttrValue>);
    let content_id = use_memo((), |_| crate::generate_id("combobox"));
    let root_ref = use_node_ref();
    let (match_count, report_match) = use_match_registry();

    // Follow later changes to the `open` prop, not just the initial one.
    {
        let open_state = open_state.clone();
        use_effect_with(open, move |open| {
            if let Some(open) = *open {
                open_state.set(open);
            }
        });
    }

    let is_value_controlled = value.is_some();
    let current_value = if is_value_controlled {
        value
    } else {
        (*internal_value).clone()
    };

    let set_open = {
        let open_state = open_state.clone();
        let on_open_change = on_open_change.clone();
        Callback::from(move |value: bool| {
            open_state.set(value);
            if let Some(cb) = on_open_change.as_ref() {
                cb.emit(value);
            }
        })
    };

    let toggle = {
        let set_open = set_open.clone();
        Callback::from(move |_: ()| set_open.emit(!is_open))
    };

    let set_filter_query = {
        let filter_query = filter_query.clone();
        let query_is_selection = query_is_selection.clone();
        Callback::from(move |query: String| {
            filter_query.set(query);
            query_is_selection.set(false);
        })
    };

    let select = {
        let internal_value = internal_value.clone();
        let selected_label = selected_label.clone();
        let filter_query = filter_query.clone();
        let query_is_selection = query_is_selection.clone();
        let set_open = set_open.clone();
        Callback::from(move |(value, label): (AttrValue, AttrValue)| {
            if !is_value_controlled {
                internal_value.set(Some(value.clone()));
            }
            filter_query.set(label.to_string());
            query_is_selection.set(true);
            selected_label.set(Some(label));
            if let Some(cb) = on_value_change.as_ref() {
                cb.emit(value);
            }
            set_open.emit(false);
        })
    };

    {
        let set_open = set_open.clone();
        use_click_outside_conditional(root_ref.clone(), move || set_open.emit(false), is_open);
    }
    {
        let set_open = set_open.clone();
        use_escape_key_conditional(move || set_open.emit(false), is_open);
    }

    let context = ComboboxContext {
        filter_query: (*filter_query).clone(),
        set_filter_query,
        is_open,
        toggle,
        set_open,
        content_id: (*content_id).clone(),
        value: current_value,
        selected_label: (*selected_label).clone(),
        select,
        query_is_selection: *query_is_selection,
        match_count,
        report_match,
    };

    let classes: Classes = vec![
        Classes::from("combobox"),
        if is_open {
            Classes::from("combobox-open")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <ContextProvider<ComboboxContext> context={context}>
            <div ref={root_ref} class={classes}>
                { children }
            </div>
        </ContextProvider<ComboboxContext>>
    }
}

/// Combobox trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Children elements, shown until an item is selected
    pub children: Children,
}

/// Combobox trigger component
///
/// Toggles the popup. After a selection it shows the selected item's label.
#[function_component(ComboboxTrigger)]
pub fn combobox_trigger(props: &ComboboxTriggerProps) -> Html {
    let ComboboxTriggerProps {
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("combobox-trigger"), class]
        .into_iter()
        .collect();

    let context = use_context::<ComboboxContext>();
    let is_open = context.as_ref().map(|ctx| ctx.is_open).unwrap_or(false);
    let content_id = context
        .as_ref()
        .map(|ctx| ctx.content_id.clone())
        .unwrap_or_default();
    let selected_label = context.as_ref().and_then(|ctx| ctx.selected_label.clone());
    let trigger_ref = use_node_ref();

    let handle_click = {
        let context = context.clone();
        let onclick = onclick.clone();
        Callback::from(move |e: MouseEvent| {
            // Clicks inside an input nested in the trigger shouldn't toggle.
            let from_input = e
                .target()
                .and_then(|target| target.dyn_into::<web_sys::HtmlInputElement>().ok())
                .is_some();
            if let Some(ctx) = context.as_ref() {
                if from_input {
                    ctx.set_open.emit(true);
                } else {
                    ctx.toggle.emit(());
                }
            }
            if let Some(cb) = onclick.as_ref() {
                cb.emit(e);
            }
        })
    };

    // A trigger that wraps its own input keeps it; plain triggers show the
    // selected label in place of the placeholder children.
    // The DOM from the previous render tells us whether an input is nested.
    let has_input = trigger_ref
        .cast::<web_sys::Element>()
        .and_then(|el| el.query_selector("input").ok().flatten())
        .is_some();
    let content = match selected_label {
        Some(label) if !has_input => html! {
            <span class="combobox-value">{ label }</span>
        },
        _ => html! { { children } },
    };

    html! {
        <button
            ref={trigger_ref}
            type="button"
            class={classes}
            onclick={handle_click}
            role="combobox"
            aria-expanded={is_open.to_string()}
            aria-controls={content_id}
            aria-haspopup="listbox"
        >
            { content }
        </button>
    }
}

/// Combobox input properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxInputProps {
    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    /// Current value. When omitted, the input shows the typed filter, or the
    /// selected item's label after a selection.
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Input event handler
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Combobox input component
///
/// Filters the items as the user types and opens the popup.
#[function_component(ComboboxInput)]
pub fn combobox_input(props: &ComboboxInputProps) -> Html {
    let ComboboxInputProps {
        placeholder,
        value,
        oninput,
        class,
    } = props.clone();

    let context = use_context::<ComboboxContext>();

    let oninput_handler = {
        let context = context.clone();
        let oninput = oninput.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            if let Some(ctx) = context.as_ref() {
                ctx.set_filter_query.emit(input.value());
                if !ctx.is_open {
                    ctx.set_open.emit(true);
                }
            }
            if let Some(cb) = oninput.as_ref() {
                cb.emit(e);
            }
        })
    };

    let value = value.or_else(|| {
        context
            .as_ref()
            .map(|ctx| AttrValue::from(ctx.filter_query.clone()))
    });
    let (expanded, controls) = match context.as_ref() {
        Some(ctx) => (Some(ctx.is_open.to_string()), Some(ctx.content_id.clone())),
        None => (None, None),
    };

    let classes: Classes = vec![Classes::from("combobox-input"), class]
        .into_iter()
        .collect();

    html! {
        <input
            type="text"
            class={classes}
            placeholder={placeholder}
            value={value}
            oninput={oninput_handler}
            role="combobox"
            aria-autocomplete="list"
            aria-expanded={expanded}
            aria-controls={controls}
        />
    }
}

/// Combobox content properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Maximum number of visible items before scrolling
    #[prop_or_default]
    pub max_visible_items: Option<usize>,

    /// Whether to show a "Create" option when no items match
    #[prop_or(false)]
    pub allow_create: bool,

    /// Callback when user creates a new item
    #[prop_or_default]
    pub on_create: Option<Callback<String>>,

    /// Children elements
    pub children: Children,
}

/// Combobox content component
///
/// The popup holding the items. Rendered only while open.
#[function_component(ComboboxContent)]
pub fn combobox_content(props: &ComboboxContentProps) -> Html {
    let ComboboxContentProps {
        class,
        max_visible_items,
        allow_create,
        on_create,
        children,
    } = props.clone();

    let context = use_context::<ComboboxContext>();
    let is_open = context.as_ref().map(|ctx| ctx.is_open).unwrap_or(false);
    let content_id = context
        .as_ref()
        .map(|ctx| ctx.content_id.clone())
        .unwrap_or_default();

    // Don't render content when closed
    if !is_open {
        return html! {};
    }

    let style =
        max_visible_items.map(|n: usize| format!("max-height: {}px; overflow-y: auto;", n * 36));

    let classes: Classes = vec![Classes::from("combobox-content"), class]
        .into_iter()
        .collect();

    let create_option = if allow_create {
        let filter_query = context
            .as_ref()
            .map(|ctx| ctx.effective_query().to_string())
            .unwrap_or_default();
        if filter_query.is_empty() {
            html! {}
        } else {
            let query = filter_query.clone();
            let on_create = on_create.clone();
            let onclick = Callback::from(move |_: MouseEvent| {
                if let Some(cb) = on_create.as_ref() {
                    cb.emit(query.clone());
                }
            });
            html! {
                <div
                    class="combobox-item combobox-create-item"
                    role="option"
                    aria-selected="false"
                    onclick={onclick}
                    tabindex="0"
                >
                    { format!("Create \"{}\"", filter_query) }
                </div>
            }
        }
    } else {
        html! {}
    };

    html! {
        <div
            class={classes}
            id={content_id}
            role="listbox"
            style={style}
        >
            { children }
            { create_option }
        </div>
    }
}

/// Combobox empty properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxEmptyProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Combobox empty component
///
/// Displayed only when no items match the current filter.
#[function_component(ComboboxEmpty)]
pub fn combobox_empty(props: &ComboboxEmptyProps) -> Html {
    let ComboboxEmptyProps { class, children } = props.clone();

    let context = use_context::<ComboboxContext>();
    if context.is_some_and(|ctx| ctx.match_count > 0) {
        return html! {};
    }

    let classes: Classes = vec![Classes::from("combobox-empty"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="status">
            { children }
        </div>
    }
}

/// Combobox group properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxGroupProps {
    /// Group heading
    #[prop_or_default]
    pub heading: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Combobox group component
///
/// Groups related combobox items.
#[function_component(ComboboxGroup)]
pub fn combobox_group(props: &ComboboxGroupProps) -> Html {
    let ComboboxGroupProps {
        heading,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("combobox-group"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="group">
            {
                if let Some(heading_text) = heading {
                    html! {
                        <div class="combobox-group-heading">
                            { heading_text }
                        </div>
                    }
                } else {
                    html! {}
                }
            }
            { children }
        </div>
    }
}

/// Combobox item properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxItemProps {
    /// Value of this item
    pub value: AttrValue,

    /// Additional keywords for search matching
    #[prop_or_default]
    pub keywords: Option<AttrValue>,

    /// Selected state. Items also show as selected when their value equals
    /// the combobox value.
    #[prop_or(false)]
    pub selected: bool,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Combobox item component
///
/// A selectable item in the combobox. Clicking it (or Enter/Space) commits
/// its value to the combobox and closes the popup. Items that don't match
/// the filter stay mounted but hidden.
#[function_component(ComboboxItem)]
pub fn combobox_item(props: &ComboboxItemProps) -> Html {
    let ComboboxItemProps {
        value,
        keywords,
        selected,
        disabled,
        onclick,
        class,
        children,
    } = props.clone();

    let context = use_context::<ComboboxContext>();
    let node_ref = use_node_ref();
    let label = use_text_content(node_ref.clone());

    let is_visible = context.as_ref().is_none_or(|ctx| {
        matches_query(
            ctx.effective_query(),
            &[
                value.as_str(),
                keywords.as_deref().unwrap_or_default(),
                label.as_str(),
            ],
        )
    });
    use_report_match(
        context.as_ref().map(|ctx| ctx.report_match.clone()),
        is_visible,
    );

    let selected = selected
        || context
            .as_ref()
            .is_some_and(|ctx| ctx.value.as_ref() == Some(&value));

    let handle_click = {
        let value = value.clone();
        let label = label.clone();
        Callback::from(move |e: MouseEvent| {
            if disabled {
                return;
            }
            if let Some(cb) = onclick.as_ref() {
                cb.emit(e);
            }
            if let Some(ctx) = context.as_ref() {
                let label = if label.trim().is_empty() {
                    value.clone()
                } else {
                    AttrValue::from(label.trim().to_string())
                };
                ctx.select.emit((value.clone(), label));
            }
        })
    };

    let onkeydown = Callback::from(move |e: KeyboardEvent| {
        if disabled {
            return;
        }
        if matches!(e.key().as_str(), "Enter" | " ") {
            e.prevent_default();
            if let Some(item) = e
                .target()
                .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                .and_then(|target| target.closest(".combobox-item").ok().flatten())
                .and_then(|item| item.dyn_into::<web_sys::HtmlElement>().ok())
            {
                item.click();
            }
        }
    });

    let classes: Classes = vec![
        Classes::from("combobox-item"),
        if selected {
            Classes::from("combobox-item-selected")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("combobox-item-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let tabindex = if disabled { "-1" } else { "0" };

    html! {
        <div
            ref={node_ref}
            class={classes}
            role="option"
            data-value={value}
            aria-selected={selected.to_string()}
            aria-disabled={disabled.to_string()}
            hidden={!is_visible}
            onclick={handle_click}
            onkeydown={onkeydown}
            tabindex={tabindex}
        >
            { children }
        </div>
    }
}

/// Combobox separator properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxSeparatorProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Combobox separator component
///
/// Separates groups of combobox items.
#[function_component(ComboboxSeparator)]
pub fn combobox_separator(props: &ComboboxSeparatorProps) -> Html {
    let ComboboxSeparatorProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("combobox-separator"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="separator" />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combobox_default_closed() {
        let props = ComboboxProps {
            open: None,
            default_open: false,
            on_open_change: None,
            value: None,
            default_value: None,
            on_value_change: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.default_open);
    }

    #[test]
    fn test_combobox_item_selected() {
        let props = ComboboxItemProps {
            value: AttrValue::from("test"),
            keywords: None,
            selected: true,
            disabled: false,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.selected);
        assert!(!props.disabled);
    }

    #[test]
    fn test_combobox_item_disabled() {
        let props = ComboboxItemProps {
            value: AttrValue::from("test"),
            keywords: None,
            selected: false,
            disabled: true,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.selected);
        assert!(props.disabled);
    }

    #[test]
    fn test_combobox_group_with_heading() {
        let props = ComboboxGroupProps {
            heading: Some(AttrValue::from("Options")),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.heading, Some(AttrValue::from("Options")));
    }

    #[test]
    fn test_combobox_item_with_keywords() {
        let props = ComboboxItemProps {
            value: AttrValue::from("react"),
            keywords: Some(AttrValue::from("javascript frontend library")),
            selected: false,
            disabled: false,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(
            props.keywords,
            Some(AttrValue::from("javascript frontend library"))
        );
    }

    #[test]
    fn test_combobox_context_creation() {
        let ctx = ComboboxContext {
            filter_query: String::from("test"),
            set_filter_query: Callback::from(|_: String| {}),
            is_open: true,
            toggle: Callback::from(|_: ()| {}),
            set_open: Callback::from(|_: bool| {}),
            content_id: String::from("combobox-1"),
            value: None,
            selected_label: None,
            select: Callback::noop(),
            query_is_selection: false,
            match_count: 0,
            report_match: Callback::noop(),
        };

        assert_eq!(ctx.filter_query, "test");
        assert!(ctx.is_open);
    }

    #[test]
    fn test_combobox_effective_query_ignores_selection_label() {
        let mut ctx = ComboboxContext {
            filter_query: String::from("Next.js"),
            set_filter_query: Callback::noop(),
            is_open: true,
            toggle: Callback::noop(),
            set_open: Callback::noop(),
            content_id: String::from("combobox-3"),
            value: Some(AttrValue::from("next")),
            selected_label: Some(AttrValue::from("Next.js")),
            select: Callback::noop(),
            query_is_selection: true,
            match_count: 0,
            report_match: Callback::noop(),
        };
        assert_eq!(ctx.effective_query(), "");
        ctx.query_is_selection = false;
        assert_eq!(ctx.effective_query(), "Next.js");
    }

    #[test]
    fn test_combobox_context_empty_query() {
        let ctx = ComboboxContext {
            filter_query: String::new(),
            set_filter_query: Callback::from(|_: String| {}),
            is_open: false,
            toggle: Callback::from(|_: ()| {}),
            set_open: Callback::from(|_: bool| {}),
            content_id: String::from("combobox-2"),
            value: None,
            selected_label: None,
            select: Callback::noop(),
            query_is_selection: false,
            match_count: 0,
            report_match: Callback::noop(),
        };

        assert!(ctx.filter_query.is_empty());
        assert!(!ctx.is_open);
    }

    #[test]
    fn test_combobox_content_with_max_visible() {
        let props = ComboboxContentProps {
            class: Classes::new(),
            max_visible_items: Some(5),
            allow_create: false,
            on_create: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.max_visible_items, Some(5));
    }

    #[test]
    fn test_combobox_content_allow_create() {
        let props = ComboboxContentProps {
            class: Classes::new(),
            max_visible_items: None,
            allow_create: true,
            on_create: Some(Callback::from(|_: String| {})),
            children: Children::new(vec![]),
        };

        assert!(props.allow_create);
        assert!(props.on_create.is_some());
    }

    #[test]
    fn test_combobox_content_defaults() {
        let props = ComboboxContentProps {
            class: Classes::new(),
            max_visible_items: None,
            allow_create: false,
            on_create: None,
            children: Children::new(vec![]),
        };

        assert!(!props.allow_create);
        assert!(props.max_visible_items.is_none());
        assert!(props.on_create.is_none());
    }

    #[test]
    fn test_combobox_item_keywords_none() {
        let props = ComboboxItemProps {
            value: AttrValue::from("test"),
            keywords: None,
            selected: false,
            disabled: false,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.keywords.is_none());
    }
}
