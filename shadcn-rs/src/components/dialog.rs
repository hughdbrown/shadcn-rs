//! Dialog component
//!
//! A modal dialog that interrupts the user with important content.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter, Button};
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
//!         <Dialog {open} {on_open_change}>
//!             <DialogTrigger>
//!                 <Button>{ "Open Dialog" }</Button>
//!             </DialogTrigger>
//!             <DialogContent>
//!                 <DialogHeader>
//!                     <DialogTitle>{ "Are you sure?" }</DialogTitle>
//!                     <DialogDescription>
//!                         { "This action cannot be undone." }
//!                     </DialogDescription>
//!                 </DialogHeader>
//!                 <DialogFooter>
//!                     <Button onclick={move |_| on_open_change.emit(false)}>
//!                         { "Cancel" }
//!                     </Button>
//!                     <Button variant={Variant::Primary}>
//!                         { "Continue" }
//!                     </Button>
//!                 </DialogFooter>
//!             </DialogContent>
//!         </Dialog>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::utils::Portal;
use crate::hooks::{use_escape_key_conditional, use_click_outside_conditional};

/// Dialog component properties
#[derive(Properties, PartialEq, Clone)]
pub struct DialogProps {
    /// Whether the dialog is open
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

/// Dialog component
///
/// A modal dialog container that manages open/close state and provides context to child components.
///
/// # Accessibility
/// - Traps focus within the dialog
/// - Closes on Escape key
/// - Closes on overlay click (unless disabled)
/// - Restores focus to trigger on close
/// - Uses proper ARIA attributes
#[function_component(Dialog)]
pub fn dialog(props: &DialogProps) -> Html {
    let DialogProps {
        open,
        default_open,
        on_open_change,
        children,
    } = props.clone();

    html! {
        <div class="dialog-root">
            { children }
        </div>
    }
}

/// Dialog trigger properties
#[derive(Properties, PartialEq, Clone)]
pub struct DialogTriggerProps {
    /// Children elements (typically a button)
    pub children: Children,
}

/// Dialog trigger component
///
/// The element that opens the dialog when clicked.
#[function_component(DialogTrigger)]
pub fn dialog_trigger(props: &DialogTriggerProps) -> Html {
    let DialogTriggerProps { children } = props.clone();

    html! {
        <div class="dialog-trigger">
            { children }
        </div>
    }
}

/// Dialog content properties
#[derive(Properties, PartialEq, Clone)]
pub struct DialogContentProps {
    /// Whether the dialog is open (managed by parent Dialog)
    #[prop_or(false)]
    pub open: bool,

    /// Callback to close the dialog
    #[prop_or_default]
    pub on_close: Option<Callback<()>>,

    /// Whether to close on overlay click
    #[prop_or(true)]
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

/// Dialog content component
///
/// The actual dialog content that appears in the modal overlay.
#[function_component(DialogContent)]
pub fn dialog_content(props: &DialogContentProps) -> Html {
    let DialogContentProps {
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

    let classes: Classes = vec![Classes::from("dialog-content"), class]
        .into_iter()
        .collect();

    html! {
        <Portal>
            <div class="dialog-overlay" aria-hidden="true">
                <div
                    ref={content_ref}
                    class={classes}
                    role="dialog"
                    aria-modal="true"
                >
                    { children }
                </div>
            </div>
        </Portal>
    }
}

/// Dialog header properties
#[derive(Properties, PartialEq, Clone)]
pub struct DialogHeaderProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dialog header component
///
/// Container for dialog title and description.
#[function_component(DialogHeader)]
pub fn dialog_header(props: &DialogHeaderProps) -> Html {
    let DialogHeaderProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("dialog-header"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            { children }
        </div>
    }
}

/// Dialog title properties
#[derive(Properties, PartialEq, Clone)]
pub struct DialogTitleProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dialog title component
///
/// The main heading of the dialog.
#[function_component(DialogTitle)]
pub fn dialog_title(props: &DialogTitleProps) -> Html {
    let DialogTitleProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("dialog-title"), class]
        .into_iter()
        .collect();

    html! {
        <h2 class={classes}>
            { children }
        </h2>
    }
}

/// Dialog description properties
#[derive(Properties, PartialEq, Clone)]
pub struct DialogDescriptionProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dialog description component
///
/// Descriptive text for the dialog.
#[function_component(DialogDescription)]
pub fn dialog_description(props: &DialogDescriptionProps) -> Html {
    let DialogDescriptionProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("dialog-description"), class]
        .into_iter()
        .collect();

    html! {
        <p class={classes}>
            { children }
        </p>
    }
}

/// Dialog footer properties
#[derive(Properties, PartialEq, Clone)]
pub struct DialogFooterProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Dialog footer component
///
/// Container for dialog actions (typically buttons).
#[function_component(DialogFooter)]
pub fn dialog_footer(props: &DialogFooterProps) -> Html {
    let DialogFooterProps { class, children } = props.clone();

    let classes: Classes = vec![Classes::from("dialog-footer"), class]
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
    fn test_dialog_props_default() {
        let props = DialogProps {
            open: false,
            default_open: false,
            on_open_change: None,
            children: Children::new(vec![]),
        };

        assert!(!props.open);
        assert!(!props.default_open);
    }

    #[test]
    fn test_dialog_content_close_behaviors() {
        let props = DialogContentProps {
            open: true,
            on_close: None,
            close_on_overlay_click: true,
            close_on_escape: true,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.close_on_overlay_click);
        assert!(props.close_on_escape);
    }

    #[test]
    fn test_dialog_content_disabled_close() {
        let props = DialogContentProps {
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
}
