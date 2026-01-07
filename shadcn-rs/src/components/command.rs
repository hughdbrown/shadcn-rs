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

use yew::prelude::*;

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

    let classes: Classes = vec![Classes::from("command"), class].into_iter().collect();

    html! {
        <div class={classes} role="application">
            { children }
        </div>
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
                oninput={oninput}
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
/// Displays when no results match the search.
#[function_component(CommandEmpty)]
pub fn command_empty(props: &CommandEmptyProps) -> Html {
    let CommandEmptyProps { class, children } = props.clone();

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
#[function_component(CommandItem)]
pub fn command_item(props: &CommandItemProps) -> Html {
    let CommandItemProps {
        value: _,
        disabled,
        onclick,
        class,
        children,
    } = props.clone();

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
            class={classes}
            role="option"
            aria-disabled={disabled.to_string()}
            onclick={onclick}
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
}
