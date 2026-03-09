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

use crate::hooks::{use_click_outside_conditional, use_escape_key_conditional, use_toggle};
use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Context for sharing combobox state with children
#[derive(Clone, PartialEq)]
pub struct ComboboxContext {
    /// Current filter query string
    pub filter_query: String,
    /// Callback to update the filter query
    pub set_filter_query: Callback<String>,
    /// Whether the combobox dropdown is open
    pub is_open: bool,
    /// Callback to toggle the dropdown open/closed
    pub toggle: Callback<()>,
}

/// Combobox container properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxProps {
    /// Open state (controlled)
    #[prop_or_default]
    pub open: Option<bool>,

    /// Default open state (uncontrolled)
    #[prop_or(false)]
    pub default_open: bool,

    /// Open state change handler
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Combobox container component
///
/// The main container for a combobox (searchable select).
///
/// # Accessibility
/// - Full keyboard navigation
/// - Screen reader support
/// - ARIA attributes
#[function_component(Combobox)]
pub fn combobox(props: &ComboboxProps) -> Html {
    let ComboboxProps {
        open,
        default_open,
        on_open_change,
        class,
        children,
    } = props.clone();

    let (is_open, internal_toggle, _set_open) = use_toggle(open.unwrap_or(default_open));
    let filter_query = use_state(String::new);

    // Wrap toggle to also emit on_open_change
    let toggle = {
        let internal_toggle = internal_toggle.clone();
        let on_open_change = on_open_change.clone();
        let current_open = is_open;
        Callback::from(move |_: ()| {
            internal_toggle.emit(());
            if let Some(cb) = on_open_change.as_ref() {
                cb.emit(!current_open);
            }
        })
    };

    let set_filter_query = {
        let filter_query = filter_query.clone();
        Callback::from(move |query: String| {
            filter_query.set(query);
        })
    };

    let context = ComboboxContext {
        filter_query: (*filter_query).clone(),
        set_filter_query,
        is_open,
        toggle,
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
            <div class={classes}>
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

    /// Children elements
    pub children: Children,
}

/// Combobox trigger component
///
/// Triggers the combobox dropdown.
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
    let id = use_memo((), |_| crate::generate_id("combobox"));

    let handle_click = {
        let context = context.clone();
        let onclick = onclick.clone();
        Callback::from(move |e: MouseEvent| {
            // Toggle open state via context
            if let Some(ctx) = context.as_ref() {
                ctx.toggle.emit(());
            }
            // Also call user's handler if provided
            if let Some(cb) = onclick.as_ref() {
                cb.emit(e);
            }
        })
    };

    html! {
        <button
            type="button"
            class={classes}
            onclick={handle_click}
            role="combobox"
            aria-expanded={is_open.to_string()}
            aria-controls={(*id).clone()}
        >
            { children }
        </button>
    }
}

/// Combobox input properties
#[derive(Properties, PartialEq, Clone)]
pub struct ComboboxInputProps {
    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    /// Current value
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
/// Search input for filtering combobox items.
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
            }
            if let Some(cb) = oninput.as_ref() {
                cb.emit(e);
            }
        })
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

    /// Whether to allow creating new items from the search query
    #[prop_or(false)]
    pub allow_create: bool,

    /// Callback invoked when the user creates a new item
    #[prop_or_default]
    pub on_create: Option<Callback<String>>,

    /// Children elements
    pub children: Children,
}

/// Combobox content component
///
/// Container for combobox items.
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
    let content_ref = use_node_ref();
    let is_open = context.as_ref().map(|ctx| ctx.is_open).unwrap_or(false);

    // Close on click outside
    let context_click = context.clone();
    use_click_outside_conditional(
        content_ref.clone(),
        move || {
            if let Some(ctx) = context_click.as_ref() {
                ctx.toggle.emit(());
            }
        },
        is_open,
    );

    // Close on Escape key
    let context_esc = context.clone();
    use_escape_key_conditional(
        move || {
            if let Some(ctx) = context_esc.as_ref() {
                ctx.toggle.emit(());
            }
        },
        is_open,
    );

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
            .map(|ctx| ctx.filter_query.clone())
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
            ref={content_ref}
            class={classes}
            id="combobox-content"
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
/// Displays when no items match the search.
#[function_component(ComboboxEmpty)]
pub fn combobox_empty(props: &ComboboxEmptyProps) -> Html {
    let ComboboxEmptyProps { class, children } = props.clone();

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

    /// Selected state
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
/// A selectable item in the combobox.
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

    // Filter visibility based on context filter_query
    let is_visible = context
        .as_ref()
        .map(|ctx| {
            if ctx.filter_query.is_empty() {
                true
            } else {
                let query = ctx.filter_query.to_lowercase();
                let val_str = value.to_string().to_lowercase();
                let kw_match = keywords
                    .as_ref()
                    .map(|k| k.to_lowercase().contains(&query))
                    .unwrap_or(false);
                val_str.contains(&query) || kw_match
            }
        })
        .unwrap_or(true);

    if !is_visible {
        return html! {};
    }

    // Keyboard navigation
    let onkeydown = {
        let onclick = onclick.clone();
        Callback::from(move |e: KeyboardEvent| {
            if disabled {
                return;
            }
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    if let Some(target) = e.target()
                        && let Ok(el) = target.dyn_into::<web_sys::HtmlElement>()
                    {
                        el.click();
                    }
                    let _ = onclick.as_ref();
                }
                _ => {}
            }
        })
    };

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
            class={classes}
            role="option"
            aria-selected={selected.to_string()}
            aria-disabled={disabled.to_string()}
            onclick={onclick}
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
        };

        assert_eq!(ctx.filter_query, "test");
        assert!(ctx.is_open);
    }

    #[test]
    fn test_combobox_context_empty_query() {
        let ctx = ComboboxContext {
            filter_query: String::new(),
            set_filter_query: Callback::from(|_: String| {}),
            is_open: false,
            toggle: Callback::from(|_: ()| {}),
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
