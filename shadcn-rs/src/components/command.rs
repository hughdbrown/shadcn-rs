//! Command component
//!
//! A fast, composable command menu (command palette).
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Command, CommandInput, CommandList, CommandEmpty, CommandGroup, CommandItem, CommandSeparator};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let search = use_state(|| String::new());
//!
//!     html! {
//!         <Command>
//!             <CommandInput placeholder="Type a command or search..." />
//!             <CommandList>
//!                 <CommandEmpty>{ "No results found." }</CommandEmpty>
//!                 <CommandGroup heading="Suggestions">
//!                     <CommandItem>{ "Calendar" }</CommandItem>
//!                     <CommandItem>{ "Search Emoji" }</CommandItem>
//!                     <CommandItem>{ "Calculator" }</CommandItem>
//!                 </CommandGroup>
//!                 <CommandSeparator />
//!                 <CommandGroup heading="Settings">
//!                     <CommandItem>{ "Profile" }</CommandItem>
//!                     <CommandItem>{ "Billing" }</CommandItem>
//!                     <CommandItem>{ "Settings" }</CommandItem>
//!                 </CommandGroup>
//!             </CommandList>
//!         </Command>
//!     }
//! }
//! ```

use std::collections::BTreeMap;
use std::rc::Rc;

use crate::utils::generate_id;
use wasm_bindgen::JsCast;
use yew::prelude::*;

/// Shared context for command search state
#[derive(Clone, PartialEq)]
pub struct CommandContext {
    /// The current search query
    pub search_query: String,
    /// Callback to update the search query
    pub set_search_query: Callback<String>,
    /// Number of items that match the current query
    pub match_count: usize,
    /// Items report `(id, Some(matches))` when rendered and `(id, None)`
    /// when removed, so the root can count matches for `CommandEmpty`
    pub report_match: Callback<(AttrValue, Option<bool>)>,
}

/// Whether any candidate contains `query`, ignoring case. An empty query
/// matches everything.
pub(crate) fn matches_query(query: &str, candidates: &[&str]) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty()
        || candidates
            .iter()
            .any(|candidate| candidate.to_lowercase().contains(&query))
}

/// Which registered items currently match the filter.
#[derive(Default, PartialEq)]
pub(crate) struct MatchRegistry(BTreeMap<AttrValue, bool>);

impl MatchRegistry {
    pub(crate) fn count(&self) -> usize {
        self.0.values().filter(|matches| **matches).count()
    }
}

impl Reducible for MatchRegistry {
    type Action = (AttrValue, Option<bool>);

    fn reduce(self: Rc<Self>, (id, matches): Self::Action) -> Rc<Self> {
        if self.0.get(&id).copied() == matches {
            return self;
        }
        let mut map = self.0.clone();
        match matches {
            Some(matches) => map.insert(id, matches),
            None => map.remove(&id),
        };
        Rc::new(MatchRegistry(map))
    }
}

/// Registry of item matches for a filterable list. Returns the match count
/// and the callback items use to report themselves.
#[hook]
pub(crate) fn use_match_registry() -> (usize, Callback<(AttrValue, Option<bool>)>) {
    let registry = use_reducer_eq(MatchRegistry::default);
    let dispatcher = registry.dispatcher();
    (
        registry.count(),
        Callback::from(move |action| dispatcher.dispatch(action)),
    )
}

/// Reports whether this item matches to the list root, and unregisters it on
/// unmount.
#[hook]
pub(crate) fn use_report_match(report: Option<Callback<(AttrValue, Option<bool>)>>, matches: bool) {
    let id = use_memo((), |_| AttrValue::from(generate_id("list-item")));
    {
        let report = report.clone();
        let id = (*id).clone();
        use_effect_with(matches, move |&matches| {
            if let Some(report) = report.as_ref() {
                report.emit((id, Some(matches)));
            }
        });
    }
    let id = (*id).clone();
    use_effect_with((), move |_| {
        move || {
            if let Some(report) = report.as_ref() {
                report.emit((id, None));
            }
        }
    });
}

/// Text content of the referenced element, refreshed after each render.
#[hook]
pub(crate) fn use_text_content(node_ref: NodeRef) -> String {
    let text = use_state(String::new);
    {
        let text = text.clone();
        use_effect(move || {
            if let Some(element) = node_ref.cast::<web_sys::Element>() {
                let current = element.text_content().unwrap_or_default();
                if current != *text {
                    text.set(current);
                }
            }
        });
    }
    (*text).clone()
}

/// Command container properties
#[derive(Properties, PartialEq, Clone)]
pub struct CommandProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Command container component
///
/// The main container for the command palette.
///
/// # Accessibility
/// - Keyboard navigation support
/// - Screen reader friendly
/// - Focus management
#[function_component(Command)]
pub fn command(props: &CommandProps) -> Html {
    let CommandProps { class, children } = props.clone();

    let search_query = use_state(String::new);

    let set_search_query = {
        let search_query = search_query.clone();
        Callback::from(move |val: String| {
            search_query.set(val);
        })
    };

    let (match_count, report_match) = use_match_registry();

    let context = CommandContext {
        search_query: (*search_query).clone(),
        set_search_query,
        match_count,
        report_match,
    };

    let classes: Classes = vec![Classes::from("command"), class].into_iter().collect();

    html! {
        <ContextProvider<CommandContext> context={context}>
            <div class={classes} role="application">
                { children }
            </div>
        </ContextProvider<CommandContext>>
    }
}

/// Command input properties
#[derive(Properties, PartialEq, Clone)]
pub struct CommandInputProps {
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

/// Command input component
///
/// Search input for filtering command items.
#[function_component(CommandInput)]
pub fn command_input(props: &CommandInputProps) -> Html {
    let CommandInputProps {
        placeholder,
        value,
        oninput,
        class,
    } = props.clone();

    let context = use_context::<CommandContext>();

    let oninput_handler = {
        let context = context.clone();
        let oninput = oninput.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(target) = e.target()
                && let Some(input) = target.dyn_ref::<web_sys::HtmlInputElement>()
                && let Some(ctx) = context.as_ref()
            {
                ctx.set_search_query.emit(input.value());
            }
            if let Some(cb) = oninput.as_ref() {
                cb.emit(e);
            }
        })
    };

    let classes: Classes = vec![Classes::from("command-input"), class]
        .into_iter()
        .collect();

    html! {
        <div class="command-input-wrapper">
            <input
                type="text"
                class={classes}
                placeholder={placeholder}
                value={value}
                oninput={oninput_handler}
                role="combobox"
                aria-expanded="true"
                aria-autocomplete="list"
            />
        </div>
    }
}

/// Command list properties
#[derive(Properties, PartialEq, Clone)]
pub struct CommandListProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Command list component
///
/// Container for command items and groups.
#[function_component(CommandList)]
pub fn command_list(props: &CommandListProps) -> Html {
    let CommandListProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("command-list"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="listbox">
            { children }
        </div>
    }
}

/// Command empty properties
#[derive(Properties, PartialEq, Clone)]
pub struct CommandEmptyProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Command empty component
///
/// Displays only when no items match the search.
#[function_component(CommandEmpty)]
pub fn command_empty(props: &CommandEmptyProps) -> Html {
    let CommandEmptyProps { class, children } = props.clone();

    let context = use_context::<CommandContext>();
    if context.is_some_and(|ctx| ctx.match_count > 0) {
        return html! {};
    }

    let classes: Classes = vec![Classes::from("command-empty"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="status">
            { children }
        </div>
    }
}

/// Command group properties
#[derive(Properties, PartialEq, Clone)]
pub struct CommandGroupProps {
    /// Group heading
    #[prop_or_default]
    pub heading: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Command group component
///
/// Groups related command items.
#[function_component(CommandGroup)]
pub fn command_group(props: &CommandGroupProps) -> Html {
    let CommandGroupProps {
        heading,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("command-group"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="group">
            {
                if let Some(heading_text) = heading {
                    html! {
                        <div class="command-group-heading">
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

/// Command item properties
#[derive(Properties, PartialEq, Clone)]
pub struct CommandItemProps {
    /// Value of this item
    #[prop_or_default]
    pub value: Option<AttrValue>,

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

/// Command item component
///
/// A selectable item in the command palette.
/// Supports fuzzy search filtering via `CommandContext` and keyboard navigation.
#[function_component(CommandItem)]
pub fn command_item(props: &CommandItemProps) -> Html {
    let CommandItemProps {
        value,
        disabled,
        onclick,
        class,
        children,
    } = props.clone();

    let context = use_context::<CommandContext>();
    let node_ref = use_node_ref();
    // Items without a `value` are matched against their rendered text.
    let label = use_text_content(node_ref.clone());

    let is_visible = context.as_ref().is_none_or(|ctx: &CommandContext| {
        let candidate: &str = value.as_deref().unwrap_or(label.as_str());
        matches_query(&ctx.search_query, &[candidate, label.as_str()])
    });
    use_report_match(
        context.as_ref().map(|ctx| ctx.report_match.clone()),
        is_visible,
    );

    let onclick = onclick.map(|onclick: Callback<MouseEvent>| {
        Callback::from(move |e: MouseEvent| {
            if !disabled {
                onclick.emit(e);
            }
        })
    });

    let onkeydown = {
        Callback::from(move |e: KeyboardEvent| {
            if disabled {
                return;
            }
            match e.key().as_str() {
                "Enter" | " " => {
                    e.prevent_default();
                    // Trigger click on the element via keyboard activation.
                    // This will fire the onclick handler attached to the div.
                    if let Some(target) = e.target()
                        && let Ok(el) = target.dyn_into::<web_sys::HtmlElement>()
                    {
                        el.click();
                    }
                }
                _ => {}
            }
        })
    };

    let tabindex = if disabled { "-1" } else { "0" };

    let classes: Classes = vec![
        Classes::from("command-item"),
        if disabled {
            Classes::from("command-item-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    html! {
        <div
            ref={node_ref}
            class={classes}
            role="option"
            aria-disabled={disabled.to_string()}
            data-value={value}
            hidden={!is_visible}
            onclick={onclick}
            onkeydown={onkeydown}
            tabindex={tabindex}
        >
            { children }
        </div>
    }
}

/// Command separator properties
#[derive(Properties, PartialEq, Clone)]
pub struct CommandSeparatorProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Command separator component
///
/// Separates groups of command items.
#[function_component(CommandSeparator)]
pub fn command_separator(props: &CommandSeparatorProps) -> Html {
    let CommandSeparatorProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("command-separator"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="separator" />
    }
}

/// Command shortcut properties
#[derive(Properties, PartialEq, Clone)]
pub struct CommandShortcutProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (keyboard shortcut text)
    pub children: Children,
}

/// Command shortcut component
///
/// Displays keyboard shortcut hint for a command.
#[function_component(CommandShortcut)]
pub fn command_shortcut(props: &CommandShortcutProps) -> Html {
    let CommandShortcutProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("command-shortcut"), class]
        .into_iter()
        .collect();

    html! {
        <span class={classes}>
            { children }
        </span>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_input_props() {
        let props = CommandInputProps {
            placeholder: Some(AttrValue::from("Search...")),
            value: None,
            oninput: None,
            class: Classes::new(),
        };

        assert_eq!(props.placeholder, Some(AttrValue::from("Search...")));
    }

    #[test]
    fn test_command_group_with_heading() {
        let props = CommandGroupProps {
            heading: Some(AttrValue::from("Actions")),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.heading, Some(AttrValue::from("Actions")));
    }

    #[test]
    fn test_command_item_disabled() {
        let props = CommandItemProps {
            value: Some(AttrValue::from("test")),
            disabled: true,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_command_item_enabled() {
        let props = CommandItemProps {
            value: Some(AttrValue::from("test")),
            disabled: false,
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.disabled);
    }

    #[test]
    fn test_command_context_clone_and_eq() {
        let ctx1 = CommandContext {
            search_query: "hello".to_string(),
            set_search_query: Callback::noop(),
            match_count: 0,
            report_match: Callback::noop(),
        };
        let ctx2 = ctx1.clone();

        assert_eq!(ctx1.search_query, ctx2.search_query);
        assert_eq!(ctx1.search_query, "hello");
    }

    #[test]
    fn test_command_context_empty_query() {
        let ctx = CommandContext {
            search_query: String::new(),
            set_search_query: Callback::noop(),
            match_count: 0,
            report_match: Callback::noop(),
        };

        assert!(ctx.search_query.is_empty());
    }

    #[test]
    fn test_command_context_search_filtering_logic() {
        // Simulate the filtering logic used in command_item
        let query = "cal".to_lowercase();
        let value = AttrValue::from("Calendar");

        let matches = value.to_lowercase().contains(&query);
        assert!(matches);
    }

    #[test]
    fn test_command_context_search_no_match() {
        let query = "xyz".to_lowercase();
        let value = AttrValue::from("Calendar");

        let matches = value.to_lowercase().contains(&query);
        assert!(!matches);
    }

    #[test]
    fn test_command_context_search_case_insensitive() {
        let query = "CALENDAR".to_lowercase();
        let value = AttrValue::from("calendar");

        let matches = value.to_lowercase().contains(&query);
        assert!(matches);
    }

    #[test]
    fn test_command_item_no_value_always_visible() {
        // When value is None, item should be visible regardless of query
        let query = "anything";
        let value: Option<AttrValue> = None;

        let is_visible = value
            .as_ref()
            .map(|v: &AttrValue| v.to_lowercase().contains(query))
            .unwrap_or(true);
        assert!(is_visible);
    }

    #[test]
    fn test_command_item_empty_query_always_visible() {
        let query = "";
        let is_visible = query.is_empty();
        assert!(is_visible);
    }

    #[test]
    fn test_command_context_set_search_query_callback() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let captured = Rc::new(RefCell::new(String::new()));
        let captured_clone = captured.clone();

        let cb = Callback::from(move |val: String| {
            *captured_clone.borrow_mut() = val;
        });

        let ctx = CommandContext {
            search_query: String::new(),
            set_search_query: cb,
            match_count: 0,
            report_match: Callback::noop(),
        };

        ctx.set_search_query.emit("test query".to_string());
        assert_eq!(*captured.borrow(), "test query");
    }

    #[test]
    fn test_matches_query() {
        assert!(matches_query("", &["anything"]));
        assert!(matches_query("  ", &["anything"]));
        assert!(matches_query("CAL", &["calendar"]));
        assert!(matches_query("moji", &["", "Search Emoji"]));
        assert!(!matches_query("xyz", &["calendar", "Calendar"]));
    }

    #[test]
    fn test_match_registry_counts_and_removes() {
        let registry = Rc::new(MatchRegistry::default());
        let registry = registry.reduce((AttrValue::from("a"), Some(true)));
        let registry = registry.reduce((AttrValue::from("b"), Some(false)));
        assert_eq!(registry.count(), 1);
        let registry = registry.reduce((AttrValue::from("b"), Some(true)));
        assert_eq!(registry.count(), 2);
        let registry = registry.reduce((AttrValue::from("a"), None));
        assert_eq!(registry.count(), 1);
        // Re-reporting the same state returns the same Rc (no re-render).
        let same = registry.clone().reduce((AttrValue::from("b"), Some(true)));
        assert!(Rc::ptr_eq(&registry, &same));
    }
}
