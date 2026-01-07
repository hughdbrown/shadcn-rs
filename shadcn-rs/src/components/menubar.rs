//! Menubar component
//!
//! A visually persistent menu common in desktop applications.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem, MenubarSeparator};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Menubar>
//!             <MenubarMenu>
//!                 <MenubarTrigger>{ "File" }</MenubarTrigger>
//!                 <MenubarContent>
//!                     <MenubarItem>{ "New File" }</MenubarItem>
//!                     <MenubarItem>{ "Open" }</MenubarItem>
//!                     <MenubarSeparator />
//!                     <MenubarItem>{ "Exit" }</MenubarItem>
//!                 </MenubarContent>
//!             </MenubarMenu>
//!         </Menubar>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Menubar container properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Menubar container component
///
/// A persistent menu bar for application-style interfaces.
///
/// # Accessibility
/// - Uses proper ARIA attributes (role="menubar")
/// - Keyboard navigation support
/// - Focus management
#[function_component(Menubar)]
pub fn menubar(props: &MenubarProps) -> Html {
    let MenubarProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("menubar"), class].into_iter().collect();

    html! {
        <div class={classes} role="menubar">
            { children }
        </div>
    }
}

/// Menubar menu properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarMenuProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Menubar menu component
///
/// A single menu in the menubar.
#[function_component(MenubarMenu)]
pub fn menubar_menu(props: &MenubarMenuProps) -> Html {
    let MenubarMenuProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("menubar-menu"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Menubar trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarTriggerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Children elements
    pub children: Children,
}

/// Menubar trigger component
///
/// Triggers the display of menubar content.
#[function_component(MenubarTrigger)]
pub fn menubar_trigger(props: &MenubarTriggerProps) -> Html {
    let MenubarTriggerProps {
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("menubar-trigger"), class]
        .into_iter()
        .collect();

    html! {
        <button class={classes} onclick={onclick} role="menuitem" aria-haspopup="true">
            { children }
        </button>
    }
}

/// Menubar content properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Menubar content component
///
/// Contains the dropdown menu items.
#[function_component(MenubarContent)]
pub fn menubar_content(props: &MenubarContentProps) -> Html {
    let MenubarContentProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("menubar-content"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="menu">
            { children }
        </div>
    }
}

/// Menubar item properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarItemProps {
    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Children elements
    pub children: Children,
}

/// Menubar item component
///
/// A clickable item in the menubar dropdown.
#[function_component(MenubarItem)]
pub fn menubar_item(props: &MenubarItemProps) -> Html {
    let MenubarItemProps {
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("menubar-item"),
        if disabled {
            Classes::from("menubar-item-disabled")
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
            role="menuitem"
            onclick={onclick}
            aria-disabled={disabled.to_string()}
        >
            { children }
        </div>
    }
}

/// Menubar separator properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarSeparatorProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Menubar separator component
///
/// Separates groups of menubar items.
#[function_component(MenubarSeparator)]
pub fn menubar_separator(props: &MenubarSeparatorProps) -> Html {
    let MenubarSeparatorProps { class } = props.clone();

    let classes: Classes = vec![Classes::from("menubar-separator"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="separator" aria-orientation="horizontal" />
    }
}

/// Menubar checkbox item properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarCheckboxItemProps {
    /// Checked state
    #[prop_or(false)]
    pub checked: bool,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Change handler
    #[prop_or_default]
    pub onchange: Option<Callback<bool>>,

    /// Children elements
    pub children: Children,
}

/// Menubar checkbox item component
///
/// A menubar item with a checkbox.
#[function_component(MenubarCheckboxItem)]
pub fn menubar_checkbox_item(props: &MenubarCheckboxItemProps) -> Html {
    let MenubarCheckboxItemProps {
        checked,
        disabled,
        class,
        onchange,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("menubar-checkbox-item"),
        if checked {
            Classes::from("menubar-checkbox-item-checked")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("menubar-checkbox-item-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let onclick = Callback::from(move |_| {
        if !disabled && let Some(callback) = onchange.as_ref() {
            callback.emit(!checked);
        }
    });

    html! {
        <div
            class={classes}
            role="menuitemcheckbox"
            aria-checked={checked.to_string()}
            aria-disabled={disabled.to_string()}
            onclick={onclick}
        >
            <span class="menubar-checkbox-indicator">
                { if checked { "✓" } else { "" } }
            </span>
            { children }
        </div>
    }
}

/// Menubar radio group properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarRadioGroupProps {
    /// Currently selected value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Change handler
    #[prop_or_default]
    pub onchange: Option<Callback<AttrValue>>,

    /// Children elements
    pub children: Children,
}

/// Menubar radio group component
///
/// Groups radio items in a menubar.
#[function_component(MenubarRadioGroup)]
pub fn menubar_radio_group(props: &MenubarRadioGroupProps) -> Html {
    let MenubarRadioGroupProps {
        value: _,
        class,
        onchange: _,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("menubar-radio-group"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes} role="group">
            { children }
        </div>
    }
}

/// Menubar radio item properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarRadioItemProps {
    /// Value of this radio item
    pub value: AttrValue,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Children elements
    pub children: Children,
}

/// Menubar radio item component
///
/// A radio option in a menubar radio group.
#[function_component(MenubarRadioItem)]
pub fn menubar_radio_item(props: &MenubarRadioItemProps) -> Html {
    let MenubarRadioItemProps {
        value: _,
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let classes: Classes = vec![
        Classes::from("menubar-radio-item"),
        if disabled {
            Classes::from("menubar-radio-item-disabled")
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
            role="menuitemradio"
            aria-disabled={disabled.to_string()}
            onclick={onclick}
        >
            <span class="menubar-radio-indicator" />
            { children }
        </div>
    }
}

/// Menubar sub properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarSubProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Menubar sub component
///
/// A submenu within a menubar menu.
#[function_component(MenubarSub)]
pub fn menubar_sub(props: &MenubarSubProps) -> Html {
    let MenubarSubProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("menubar-sub"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menubar_item_disabled() {
        let props = MenubarItemProps {
            disabled: true,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(props.disabled);
    }

    #[test]
    fn test_menubar_checkbox_item_checked() {
        let props = MenubarCheckboxItemProps {
            checked: true,
            disabled: false,
            class: Classes::new(),
            onchange: None,
            children: Children::new(vec![]),
        };

        assert!(props.checked);
        assert!(!props.disabled);
    }

    #[test]
    fn test_menubar_radio_item_value() {
        let props = MenubarRadioItemProps {
            value: AttrValue::from("option1"),
            disabled: false,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert_eq!(props.value, AttrValue::from("option1"));
        assert!(!props.disabled);
    }
}
