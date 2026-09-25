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

use crate::components::tabs::{TabsOrientation, query_all, roving_index};
use crate::hooks::{use_click_outside_conditional, use_escape_key_conditional};
use crate::utils::{focus_element, generate_id};
use wasm_bindgen::JsCast;
use web_sys::Element;
use yew::prelude::*;

/// Context shared by `Menubar` with its menus: which menu is open.
#[derive(Clone, PartialEq)]
pub struct MenubarContext {
    /// Value of the open menu, if any (only one menu is open at a time)
    pub open_menu: Option<AttrValue>,
    /// Opens the menu with the given value, or closes all menus with `None`
    pub set_open_menu: Callback<Option<AttrValue>>,
}

/// Context shared by a `MenubarMenu` with its trigger, content and items.
#[derive(Clone, PartialEq)]
pub struct MenubarMenuContext {
    /// Identifier of this menu within the menubar
    pub value: AttrValue,
    /// Whether this menu is open
    pub open: bool,
    /// Opens or closes this menu
    pub set_open: Callback<bool>,
    /// Id of the trigger button
    pub trigger_id: AttrValue,
    /// Id of the content element
    pub content_id: AttrValue,
}

/// Moves focus to the next/previous enabled item of the menu that contains
/// `from`, following `key` (ArrowUp/ArrowDown/Home/End). Returns true if the
/// key was handled.
fn move_item_focus(from: &Element, key: &str) -> bool {
    let Some(menu) = from.closest("[role='menu']").ok().flatten() else {
        return false;
    };
    let items = query_all(&menu, "[role^='menuitem']:not([aria-disabled='true'])");
    let current = items.iter().position(|item| item == from).unwrap_or(0);
    match roving_index(current, items.len(), key, TabsOrientation::Vertical) {
        Some(next) => {
            focus_element(&items[next]);
            true
        }
        None => false,
    }
}

/// Resolves the element that received a delegated keyboard event.
fn event_element(event: &KeyboardEvent, selector: &str) -> Option<Element> {
    event
        .target()
        .and_then(|target| target.dyn_into::<Element>().ok())
        .and_then(|target| target.closest(selector).ok().flatten())
}

/// Shared keydown handling for menu items: Enter/Space activate, arrows move.
fn item_keydown() -> Callback<KeyboardEvent> {
    Callback::from(|e: KeyboardEvent| {
        let Some(item) = event_element(&e, "[role^='menuitem']") else {
            return;
        };
        match e.key().as_str() {
            "Enter" | " " => {
                e.prevent_default();
                if let Some(item) = item.dyn_ref::<web_sys::HtmlElement>() {
                    item.click();
                }
            }
            key => {
                if move_item_focus(&item, key) {
                    e.prevent_default();
                }
            }
        }
    })
}

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
/// A persistent menu bar for application-style interfaces. Tracks which
/// menu is open; only one is open at a time. Clicking outside or pressing
/// Escape closes it.
///
/// # Accessibility
/// - Uses proper ARIA attributes (role="menubar", aria-expanded)
/// - ArrowLeft/ArrowRight move between triggers, ArrowUp/ArrowDown between items
/// - Escape closes the open menu and returns focus to its trigger
#[function_component(Menubar)]
pub fn menubar(props: &MenubarProps) -> Html {
    let MenubarProps { class, children } = props.clone();

    let open_menu = use_state(|| None::<AttrValue>);
    let root_ref = use_node_ref();
    let is_open = open_menu.is_some();

    {
        let open_menu = open_menu.clone();
        let root_ref = root_ref.clone();
        use_escape_key_conditional(
            move || {
                // Return focus to the trigger of the menu being closed.
                if let Some(root) = root_ref.cast::<Element>()
                    && let Ok(Some(trigger)) =
                        root.query_selector(".menubar-trigger[data-state='open']")
                {
                    focus_element(&trigger);
                }
                open_menu.set(None);
            },
            is_open,
        );
    }

    {
        let open_menu = open_menu.clone();
        use_click_outside_conditional(root_ref.clone(), move || open_menu.set(None), is_open);
    }

    let set_open_menu = {
        let open_menu = open_menu.clone();
        Callback::from(move |value: Option<AttrValue>| open_menu.set(value))
    };

    let context = MenubarContext {
        open_menu: (*open_menu).clone(),
        set_open_menu,
    };

    let classes: Classes = vec![Classes::from("menubar"), class].into_iter().collect();

    html! {
        <ContextProvider<MenubarContext> context={context}>
            <div ref={root_ref} class={classes} role="menubar" aria-orientation="horizontal">
                { children }
            </div>
        </ContextProvider<MenubarContext>>
    }
}

/// Menubar menu properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarMenuProps {
    /// Identifier of this menu. Generated when omitted.
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Menubar menu component
///
/// A single menu in the menubar: a trigger plus its content.
#[function_component(MenubarMenu)]
pub fn menubar_menu(props: &MenubarMenuProps) -> Html {
    let MenubarMenuProps {
        value,
        class,
        children,
    } = props.clone();

    let generated = use_state(|| AttrValue::from(generate_id("menubar-menu")));
    let value = value.unwrap_or_else(|| (*generated).clone());
    let bar = use_context::<MenubarContext>();
    // Standalone menus (outside a Menubar) keep their own open state.
    let local_open = use_state(|| false);

    let open = match bar.as_ref() {
        Some(bar) => bar.open_menu.as_ref() == Some(&value),
        None => *local_open,
    };

    let set_open = {
        let bar = bar.clone();
        let value = value.clone();
        let local_open = local_open.clone();
        Callback::from(move |open: bool| match bar.as_ref() {
            Some(bar) => bar
                .set_open_menu
                .emit(if open { Some(value.clone()) } else { None }),
            None => local_open.set(open),
        })
    };

    let context = MenubarMenuContext {
        trigger_id: AttrValue::from(format!("{}-trigger", *generated)),
        content_id: AttrValue::from(format!("{}-content", *generated)),
        value,
        open,
        set_open,
    };

    let classes: Classes = vec![Classes::from("menubar-menu"), class]
        .into_iter()
        .collect();

    html! {
        <ContextProvider<MenubarMenuContext> context={context}>
            <div class={classes} data-state={if open { "open" } else { "closed" }}>
                { children }
            </div>
        </ContextProvider<MenubarMenuContext>>
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
/// Toggles its menu on click. While any menu is open, hovering another
/// trigger switches to that menu.
#[function_component(MenubarTrigger)]
pub fn menubar_trigger(props: &MenubarTriggerProps) -> Html {
    let MenubarTriggerProps {
        class,
        onclick,
        children,
    } = props.clone();

    let menu = use_context::<MenubarMenuContext>();
    let bar = use_context::<MenubarContext>();
    let open = menu.as_ref().is_some_and(|menu| menu.open);

    let classes: Classes = vec![Classes::from("menubar-trigger"), class]
        .into_iter()
        .collect();

    let handle_click = {
        let menu = menu.clone();
        Callback::from(move |e: MouseEvent| {
            if let Some(callback) = onclick.as_ref() {
                callback.emit(e);
            }
            if let Some(menu) = menu.as_ref() {
                menu.set_open.emit(!menu.open);
            }
        })
    };

    let onmouseenter = {
        let menu = menu.clone();
        let bar = bar.clone();
        Callback::from(move |_: MouseEvent| {
            if let (Some(menu), Some(bar)) = (menu.as_ref(), bar.as_ref())
                && bar.open_menu.is_some()
                && !menu.open
            {
                menu.set_open.emit(true);
            }
        })
    };

    let onkeydown = {
        let menu = menu.clone();
        let bar = bar.clone();
        Callback::from(move |e: KeyboardEvent| {
            let Some(trigger) = event_element(&e, ".menubar-trigger") else {
                return;
            };
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if let Some(menu) = menu.as_ref() {
                        menu.set_open.emit(true);
                    }
                }
                key @ ("ArrowLeft" | "ArrowRight" | "Home" | "End") => {
                    let Some(root) = trigger.closest("[role='menubar']").ok().flatten() else {
                        return;
                    };
                    let triggers = query_all(&root, ".menubar-trigger:not([disabled])");
                    let Some(current) = triggers.iter().position(|t| t == &trigger) else {
                        return;
                    };
                    let Some(next) =
                        roving_index(current, triggers.len(), key, TabsOrientation::Horizontal)
                    else {
                        return;
                    };
                    e.prevent_default();
                    let next_trigger = &triggers[next];
                    focus_element(next_trigger);
                    // Keep a menu open while moving along the bar.
                    if let Some(bar) = bar.as_ref()
                        && bar.open_menu.is_some()
                        && let Some(value) = next_trigger.get_attribute("data-menu")
                    {
                        bar.set_open_menu.emit(Some(AttrValue::from(value)));
                    }
                }
                _ => {}
            }
        })
    };

    let (id, controls, menu_value) = match menu.as_ref() {
        Some(menu) => (
            Some(menu.trigger_id.clone()),
            open.then(|| menu.content_id.clone()),
            Some(menu.value.clone()),
        ),
        None => (None, None, None),
    };

    html! {
        <button
            type="button"
            class={classes}
            {id}
            onclick={handle_click}
            {onmouseenter}
            {onkeydown}
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded={open.to_string()}
            aria-controls={controls}
            data-state={if open { "open" } else { "closed" }}
            data-menu={menu_value}
        >
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
/// Contains the menu items. Rendered only while its menu is open.
#[function_component(MenubarContent)]
pub fn menubar_content(props: &MenubarContentProps) -> Html {
    let MenubarContentProps { class, children } = props.clone();

    let menu = use_context::<MenubarMenuContext>();
    // Without a MenubarMenu there is no open state to follow; always render.
    if menu.as_ref().is_some_and(|menu| !menu.open) {
        return html! {};
    }

    let classes: Classes = vec![Classes::from("menubar-content"), class]
        .into_iter()
        .collect();

    let (id, labelledby) = match menu.as_ref() {
        Some(menu) => (Some(menu.content_id.clone()), Some(menu.trigger_id.clone())),
        None => (None, None),
    };

    html! {
        <div
            class={classes}
            role="menu"
            {id}
            aria-labelledby={labelledby}
            aria-orientation="vertical"
            data-state="open"
        >
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
/// A clickable item in the menubar dropdown. Selecting it closes the menu.
#[function_component(MenubarItem)]
pub fn menubar_item(props: &MenubarItemProps) -> Html {
    let MenubarItemProps {
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let menu = use_context::<MenubarMenuContext>();

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

    let handle_click = Callback::from(move |e: MouseEvent| {
        if disabled {
            return;
        }
        if let Some(callback) = onclick.as_ref() {
            callback.emit(e);
        }
        if let Some(menu) = menu.as_ref() {
            menu.set_open.emit(false);
        }
    });

    let tabindex: &str = if disabled { "-1" } else { "0" };

    html! {
        <div
            class={classes}
            role="menuitem"
            onclick={handle_click}
            onkeydown={item_keydown()}
            tabindex={tabindex}
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
/// A menubar item with a checkbox. Toggling it keeps the menu open.
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

    let tabindex: &str = if disabled { "-1" } else { "0" };

    html! {
        <div
            class={classes}
            role="menuitemcheckbox"
            aria-checked={checked.to_string()}
            aria-disabled={disabled.to_string()}
            data-state={if checked { "checked" } else { "unchecked" }}
            tabindex={tabindex}
            onclick={onclick}
            onkeydown={item_keydown()}
        >
            <span class="menubar-checkbox-indicator" aria-hidden="true">
                { if checked { "✓" } else { "" } }
            </span>
            { children }
        </div>
    }
}

/// Context for sharing radio group state within menubar
#[derive(Clone, PartialEq)]
pub struct MenubarRadioContext {
    /// Currently selected value
    pub value: Option<AttrValue>,
    /// Callback when value changes
    pub onchange: Option<Callback<AttrValue>>,
}

/// Menubar radio group properties
#[derive(Properties, PartialEq, Clone)]
pub struct MenubarRadioGroupProps {
    /// Currently selected value (controlled)
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Initially selected value (uncontrolled)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Change handler (receives the newly selected value)
    #[prop_or_default]
    pub onchange: Option<Callback<AttrValue>>,

    /// Children elements
    pub children: Children,
}

/// Menubar radio group component
///
/// Groups radio items in a menubar. Works controlled (`value` + `onchange`)
/// or uncontrolled (`default_value`).
#[function_component(MenubarRadioGroup)]
pub fn menubar_radio_group(props: &MenubarRadioGroupProps) -> Html {
    let MenubarRadioGroupProps {
        value,
        default_value,
        class,
        onchange,
        children,
    } = props.clone();

    let internal = use_state(|| default_value);
    let is_controlled = value.is_some();
    let current = if is_controlled {
        value
    } else {
        (*internal).clone()
    };

    let set_value = {
        let internal = internal.clone();
        Callback::from(move |new_value: AttrValue| {
            if !is_controlled {
                internal.set(Some(new_value.clone()));
            }
            if let Some(callback) = onchange.as_ref() {
                callback.emit(new_value);
            }
        })
    };

    let radio_context = MenubarRadioContext {
        value: current,
        onchange: Some(set_value),
    };

    let classes: Classes = vec![Classes::from("menubar-radio-group"), class]
        .into_iter()
        .collect();

    html! {
        <ContextProvider<MenubarRadioContext> context={radio_context}>
            <div class={classes} role="group">
                { children }
            </div>
        </ContextProvider<MenubarRadioContext>>
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
/// A radio option in a menubar radio group. Reads the group's value to show
/// its checked state and reports selection through the group's `onchange`.
#[function_component(MenubarRadioItem)]
pub fn menubar_radio_item(props: &MenubarRadioItemProps) -> Html {
    let MenubarRadioItemProps {
        value,
        disabled,
        class,
        onclick,
        children,
    } = props.clone();

    let group = use_context::<MenubarRadioContext>();
    let checked = group
        .as_ref()
        .is_some_and(|group| group.value.as_ref() == Some(&value));

    let classes: Classes = vec![
        Classes::from("menubar-radio-item"),
        if checked {
            Classes::from("menubar-radio-item-checked")
        } else {
            Classes::new()
        },
        if disabled {
            Classes::from("menubar-radio-item-disabled")
        } else {
            Classes::new()
        },
        class,
    ]
    .into_iter()
    .collect();

    let handle_click = {
        let value = value.clone();
        Callback::from(move |e: MouseEvent| {
            if disabled {
                return;
            }
            if let Some(callback) = onclick.as_ref() {
                callback.emit(e);
            }
            if let Some(callback) = group.as_ref().and_then(|group| group.onchange.as_ref()) {
                callback.emit(value.clone());
            }
        })
    };

    let tabindex: &str = if disabled { "-1" } else { "0" };

    html! {
        <div
            class={classes}
            role="menuitemradio"
            aria-checked={checked.to_string()}
            aria-disabled={disabled.to_string()}
            data-state={if checked { "checked" } else { "unchecked" }}
            data-value={value}
            tabindex={tabindex}
            onclick={handle_click}
            onkeydown={item_keydown()}
        >
            <span class="menubar-radio-indicator" aria-hidden="true">
                { if checked { "●" } else { "" } }
            </span>
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
    fn test_menubar_trigger_props() {
        let props = MenubarTriggerProps {
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(props.onclick.is_none());
    }

    #[test]
    fn test_menubar_item_enabled_props() {
        let props = MenubarItemProps {
            disabled: false,
            class: Classes::new(),
            onclick: None,
            children: Children::new(vec![]),
        };

        assert!(!props.disabled);
        assert!(props.onclick.is_none());
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
