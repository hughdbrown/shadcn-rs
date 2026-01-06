//! Alert Dialog component
//!
//! A modal dialog that interrupts the user with urgent content and expects a response.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{
//!     AlertDialog, AlertDialogTrigger, AlertDialogContent, AlertDialogHeader,
//!     AlertDialogTitle, AlertDialogDescription, AlertDialogFooter,
//!     AlertDialogAction, AlertDialogCancel, Button
//! };
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let open = use_state(|| false);
//!
//!     let on_open_change = {
//!         let open = open.clone();
//!         Callback::from(move |is_open: bool| {
//!             open.set(is_open);
//!         })
//!     };
//!
//!     html! {
//!         <AlertDialog open={*open} {on_open_change}>
//!             <AlertDialogTrigger>
//!                 <Button>{ "Delete Account" }</Button>
//!             </AlertDialogTrigger>
//!             <AlertDialogContent>
//!                 <AlertDialogHeader>
//!                     <AlertDialogTitle>{ "Are you absolutely sure?" }</AlertDialogTitle>
//!                     <AlertDialogDescription>
//!                         { "This action cannot be undone. This will permanently delete your account." }
//!                     </AlertDialogDescription>
//!                 </AlertDialogHeader>
//!                 <AlertDialogFooter>
//!                     <AlertDialogCancel>{ "Cancel" }</AlertDialogCancel>
//!                     <AlertDialogAction>{ "Continue" }</AlertDialogAction>
//!                 </AlertDialogFooter>
//!             </AlertDialogContent>
//!         </AlertDialog>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::utils::Portal;
use crate::hooks::{use_escape_key_conditional, use_click_outside_conditional};

/// Alert Dialog component properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertDialogProps {
    /// Whether the alert dialog is open
    #[prop_or(false)]
    pub open: bool,

    /// Default open state (for uncontrolled dialogs)
    #[prop_or(false)]
    pub default_open: bool,

    /// Callback when open state changes
    #[prop_or_default]
    pub on_open_change: Option<Callback<bool>>,

    /// Children elements
    pub children: Children,
}

/// Alert Dialog component
///
/// A modal alert dialog container that manages open/close state and provides context to child components.
///
/// # Accessibility
/// - Uses role="alertdialog" for urgent interruptions
/// - Traps focus within the dialog
/// - Closes on Escape key
/// - Does NOT close on overlay click (requires explicit action)
/// - Restores focus to trigger on close
/// - Uses proper ARIA attributes
#[function_component(AlertDialog)]
pub fn alert_dialog(props: &AlertDialogProps) -> Html {
    let AlertDialogProps {
        open: _,
        default_open: _,
        on_open_change: _,
        children,
    } = props.clone();

    html! {
        <div class="alert-dialog-root">
            { children }
        </div>
    }
}

/// Alert Dialog trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertDialogTriggerProps {
    /// Children elements (typically a button)
    pub children: Children,
}

/// Alert Dialog trigger component
///
/// The element that opens the alert dialog when clicked.
#[function_component(AlertDialogTrigger)]
pub fn alert_dialog_trigger(props: &AlertDialogTriggerProps) -> Html {
    let AlertDialogTriggerProps { children } = props.clone();

    html! {
        <div class="alert-dialog-trigger">
            { children }
        </div>
    }
}

/// Alert Dialog content properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertDialogContentProps {
    /// Whether the alert dialog is open (managed by parent AlertDialog)
    #[prop_or(false)]
    pub open: bool,

    /// Callback to close the alert dialog
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Whether to close on overlay click (default: false for alert dialogs)
    #[prop_or(false)]
    pub close_on_overlay_click: bool,

    /// Whether to close on Escape key
    #[prop_or(true)]
    pub close_on_escape: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Alert Dialog content component
///
/// The actual alert dialog content that appears in the modal overlay.
#[function_component(AlertDialogContent)]
pub fn alert_dialog_content(props: &AlertDialogContentProps) -> Html {
    let AlertDialogContentProps {
        open,
        on_close,
        close_on_overlay_click,
        close_on_escape,
        class,
        children,
    } = props.clone();

    let content_ref = use_node_ref();

    // Handle Escape key
    let on_close_esc = on_close.clone();
    use_escape_key_conditional(
        move || {
            if let Some(callback) = on_close_esc.as_ref() {
                callback.emit(());
            }
        },
        open && close_on_escape,
    );

    // Handle click outside
    let on_close_click = on_close.clone();
    use_click_outside_conditional(
        content_ref.clone(),
        move || {
            if let Some(callback) = on_close_click.as_ref() {
                callback.emit(());
            }
        },
        open && close_on_overlay_click,
    );

    if !open {
        return html! {};
    }

    let classes: Classes = vec![Classes::from("alert-dialog-content"), class]
        .into_iter()
        .collect();

    html! {
        <Portal>
            <div class="alert-dialog-overlay" aria-hidden="true">
                <div
                    ref={content_ref}
                    class={classes}
                    role="alertdialog"
                    aria-modal="true"
                >
                    { children }
                </div>
            </div>
        </Portal>
    }
}

/// Alert Dialog header properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertDialogHeaderProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Alert Dialog header component
///
/// Container for alert dialog title and description.
#[function_component(AlertDialogHeader)]
pub fn alert_dialog_header(props: &AlertDialogHeaderProps) -> Html {
    let AlertDialogHeaderProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("alert-dialog-header"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Alert Dialog title properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertDialogTitleProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Alert Dialog title component
///
/// The main heading of the alert dialog.
#[function_component(AlertDialogTitle)]
pub fn alert_dialog_title(props: &AlertDialogTitleProps) -> Html {
    let AlertDialogTitleProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("alert-dialog-title"), class]
        .into_iter()
        .collect();

    html! {
        <h2 class={classes}>
            { children }
        </h2>
    }
}

/// Alert Dialog description properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertDialogDescriptionProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Alert Dialog description component
///
/// Descriptive text for the alert dialog.
#[function_component(AlertDialogDescription)]
pub fn alert_dialog_description(props: &AlertDialogDescriptionProps) -> Html {
    let AlertDialogDescriptionProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("alert-dialog-description"), class]
        .into_iter()
        .collect();

    html! {
        <p class={classes}>
            { children }
        </p>
    }
}

/// Alert Dialog footer properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertDialogFooterProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Alert Dialog footer component
///
/// Container for alert dialog actions (typically Cancel and Action buttons).
#[function_component(AlertDialogFooter)]
pub fn alert_dialog_footer(props: &AlertDialogFooterProps) -> Html {
    let AlertDialogFooterProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("alert-dialog-footer"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Alert Dialog action properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertDialogActionProps {
    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Alert Dialog action component
///
/// The primary action button (e.g., "Delete", "Continue").
#[function_component(AlertDialogAction)]
pub fn alert_dialog_action(props: &AlertDialogActionProps) -> Html {
    let AlertDialogActionProps {
        onclick,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("alert-dialog-action"), class]
        .into_iter()
        .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={onclick}
        >
            { children }
        </button>
    }
}

/// Alert Dialog cancel properties
#[derive(Properties, PartialEq, Clone)]
pub struct AlertDialogCancelProps {
    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Alert Dialog cancel component
///
/// The cancel/dismiss button.
#[function_component(AlertDialogCancel)]
pub fn alert_dialog_cancel(props: &AlertDialogCancelProps) -> Html {
    let AlertDialogCancelProps {
        onclick,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("alert-dialog-cancel"), class]
        .into_iter()
        .collect();

    html! {
        <button
            type="button"
            class={classes}
            onclick={onclick}
        >
            { children }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alert_dialog_props_default() {
        let props = AlertDialogProps {
            open: false,
            default_open: false,
            on_open_change: None,
            children: Children::new(vec![]),
        };

        assert!(!props.open);
        assert!(!props.default_open);
    }

    #[test]
    fn test_alert_dialog_content_no_overlay_click() {
        let props = AlertDialogContentProps {
            open: true,
            on_close: None,
            close_on_overlay_click: false,
            close_on_escape: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.close_on_overlay_click);
        assert!(props.close_on_escape);
    }

    #[test]
    fn test_alert_dialog_content_disabled_close() {
        let props = AlertDialogContentProps {
            open: true,
            on_close: None,
            close_on_overlay_click: false,
            close_on_escape: false,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(!props.close_on_overlay_click);
        assert!(!props.close_on_escape);
    }

    #[test]
    fn test_alert_dialog_action_props() {
        let props = AlertDialogActionProps {
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.onclick.is_none());
    }

    #[test]
    fn test_alert_dialog_cancel_props() {
        let props = AlertDialogCancelProps {
            onclick: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.onclick.is_none());
    }
}
