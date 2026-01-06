//! Textarea component
//!
//! A multi-line text input field.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Textarea, Label};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let value = use_state(|| String::new());
//!
//!     let oninput = {
//!         let value = value.clone();
//!         Callback::from(move |e: InputEvent| {
//!             let textarea: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
//!             value.set(textarea.value());
//!         })
//!     };
//!
//!     html! {
//!         <div>
//!             <Label html_for="message">{ "Message" }</Label>
//!             <Textarea
//!                 id="message"
//!                 placeholder="Enter your message"
//!                 value={(*value).clone()}
//!                 {oninput}
//!             />
//!         </div>
//!     }
//! }
//! ```

use yew::prelude::*;
use web_sys::HtmlTextAreaElement;
use crate::types::Size;
use crate::utils::class_names;

/// Textarea resize behavior
#[derive(Debug, Clone, PartialEq)]
pub enum TextareaResize {
    /// No resize allowed
    None,
    /// Vertical resize only (default)
    Vertical,
    /// Horizontal resize only
    Horizontal,
    /// Both directions
    Both,
}

impl TextareaResize {
    /// Convert to CSS class
    pub fn to_class(&self) -> &'static str {
        match self {
            TextareaResize::None => "textarea-resize-none",
            TextareaResize::Vertical => "textarea-resize-vertical",
            TextareaResize::Horizontal => "textarea-resize-horizontal",
            TextareaResize::Both => "textarea-resize-both",
        }
    }
}

/// Textarea component properties
#[derive(Properties, PartialEq, Clone)]
pub struct TextareaProps {
    /// Textarea value
    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// Default value (for uncontrolled textareas)
    #[prop_or_default]
    pub default_value: Option<AttrValue>,

    /// Placeholder text
    #[prop_or_default]
    pub placeholder: Option<AttrValue>,

    /// Textarea size
    #[prop_or(Size::Md)]
    pub size: Size,

    /// Number of visible text rows
    #[prop_or(3)]
    pub rows: u32,

    /// Number of visible text columns
    #[prop_or_default]
    pub cols: Option<u32>,

    /// Resize behavior
    #[prop_or(TextareaResize::Vertical)]
    pub resize: TextareaResize,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Read-only state
    #[prop_or(false)]
    pub readonly: bool,

    /// Required field
    #[prop_or(false)]
    pub required: bool,

    /// Error state
    #[prop_or(false)]
    pub error: bool,

    /// Maximum length
    #[prop_or_default]
    pub maxlength: Option<u32>,

    /// Minimum length
    #[prop_or_default]
    pub minlength: Option<u32>,

    /// Name attribute
    #[prop_or_default]
    pub name: Option<AttrValue>,

    /// ID attribute
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Autocomplete attribute
    #[prop_or_default]
    pub autocomplete: Option<AttrValue>,

    /// Input event handler
    #[prop_or_default]
    pub oninput: Option<Callback<InputEvent>>,

    /// Change event handler
    #[prop_or_default]
    pub onchange: Option<Callback<Event>>,

    /// Focus event handler
    #[prop_or_default]
    pub onfocus: Option<Callback<FocusEvent>>,

    /// Blur event handler
    #[prop_or_default]
    pub onblur: Option<Callback<FocusEvent>>,

    /// Keydown event handler
    #[prop_or_default]
    pub onkeydown: Option<Callback<KeyboardEvent>>,

    /// Keyup event handler
    #[prop_or_default]
    pub onkeyup: Option<Callback<KeyboardEvent>>,

    /// ARIA label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// ARIA described by
    #[prop_or_default]
    pub aria_describedby: Option<AttrValue>,

    /// ARIA invalid
    #[prop_or_default]
    pub aria_invalid: Option<bool>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline styles
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Node ref
    #[prop_or_default]
    pub node_ref: NodeRef,
}

/// Textarea component
///
/// A multi-line text input component with support for various states and configurations.
///
/// # Features
/// - Configurable rows and columns
/// - Resize control (none, vertical, horizontal, both)
/// - Character limits (minlength, maxlength)
/// - Multiple states (disabled, readonly, error, required)
///
/// # Accessibility
/// - Supports ARIA attributes
/// - Keyboard navigation
/// - Screen reader friendly
#[function_component(Textarea)]
pub fn textarea(props: &TextareaProps) -> Html {
    let TextareaProps {
        value,
        default_value,
        placeholder,
        size,
        rows,
        cols,
        resize,
        disabled,
        readonly,
        required,
        error,
        maxlength,
        minlength,
        name,
        id,
        autocomplete,
        oninput,
        onchange,
        onfocus,
        onblur,
        onkeydown,
        onkeyup,
        aria_label,
        aria_describedby,
        aria_invalid,
        class,
        style,
        node_ref,
    } = props.clone();

    // Build class names
    let classes = class_names(&[
        Some("textarea"),
        Some(size.to_class()),
        Some(resize.to_class()),
        if error { Some("textarea-error") } else { None },
        if disabled { Some("textarea-disabled") } else { None },
        if readonly { Some("textarea-readonly") } else { None },
    ]);

    // Merge with custom classes
    let final_classes: Classes = vec![classes, class].into_iter().collect();

    // Determine aria-invalid
    let aria_invalid_value = aria_invalid.or(Some(error)).map(|v| v.to_string());

    html! {
        <textarea
            ref={node_ref}
            class={final_classes}
            value={value}
            placeholder={placeholder}
            rows={rows.to_string()}
            cols={cols.map(|c| c.to_string())}
            disabled={disabled}
            readonly={readonly}
            required={required}
            maxlength={maxlength.map(|m| m.to_string())}
            minlength={minlength.map(|m| m.to_string())}
            name={name}
            id={id}
            autocomplete={autocomplete}
            oninput={oninput}
            onchange={onchange}
            onfocus={onfocus}
            onblur={onblur}
            onkeydown={onkeydown}
            onkeyup={onkeyup}
            aria-label={aria_label}
            aria-describedby={aria_describedby}
            aria-invalid={aria_invalid_value}
            style={style}
        />
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_textarea_resize_class() {
        assert_eq!(TextareaResize::None.to_class(), "textarea-resize-none");
        assert_eq!(
            TextareaResize::Vertical.to_class(),
            "textarea-resize-vertical"
        );
        assert_eq!(
            TextareaResize::Horizontal.to_class(),
            "textarea-resize-horizontal"
        );
        assert_eq!(TextareaResize::Both.to_class(), "textarea-resize-both");
    }

    #[test]
    fn test_textarea_props_default() {
        let props = TextareaProps {
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            rows: 3,
            cols: None,
            resize: TextareaResize::Vertical,
            disabled: false,
            readonly: false,
            required: false,
            error: false,
            maxlength: None,
            minlength: None,
            name: None,
            id: None,
            autocomplete: None,
            oninput: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            onkeydown: None,
            onkeyup: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert_eq!(props.size, Size::Md);
        assert_eq!(props.rows, 3);
        assert_eq!(props.resize, TextareaResize::Vertical);
        assert!(!props.disabled);
        assert!(!props.error);
    }

    #[test]
    fn test_textarea_with_rows_cols() {
        let props = TextareaProps {
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            rows: 5,
            cols: Some(50),
            resize: TextareaResize::Both,
            disabled: false,
            readonly: false,
            required: false,
            error: false,
            maxlength: None,
            minlength: None,
            name: None,
            id: None,
            autocomplete: None,
            oninput: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            onkeydown: None,
            onkeyup: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert_eq!(props.rows, 5);
        assert_eq!(props.cols, Some(50));
    }

    #[test]
    fn test_textarea_character_limits() {
        let props = TextareaProps {
            value: None,
            default_value: None,
            placeholder: None,
            size: Size::Md,
            rows: 3,
            cols: None,
            resize: TextareaResize::Vertical,
            disabled: false,
            readonly: false,
            required: false,
            error: false,
            maxlength: Some(500),
            minlength: Some(10),
            name: None,
            id: None,
            autocomplete: None,
            oninput: None,
            onchange: None,
            onfocus: None,
            onblur: None,
            onkeydown: None,
            onkeyup: None,
            aria_label: None,
            aria_describedby: None,
            aria_invalid: None,
            class: Classes::new(),
            style: None,
            node_ref: NodeRef::default(),
        };

        assert_eq!(props.maxlength, Some(500));
        assert_eq!(props.minlength, Some(10));
    }
}
