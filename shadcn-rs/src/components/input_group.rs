//! Input Group component
//!
//! Groups an input with prefix/suffix elements or add-ons.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{InputGroup, Input};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <InputGroup
//!             prefix={html! { <span>{ "@" }</span> }}
//!             suffix={html! { <span>{ ".com" }</span> }}
//!         >
//!             <Input placeholder="username" />
//!         </InputGroup>
//!     }
//! }
//! ```

use yew::prelude::*;
use crate::types::Size;

/// Input group component properties
#[derive(Properties, PartialEq, Clone)]
pub struct InputGroupProps {
    /// Prefix content (appears before input)
    #[prop_or_default]
    pub prefix: Option<Html>,

    /// Suffix content (appears after input)
    #[prop_or_default]
    pub suffix: Option<Html>,

    /// Add-on before input (button or text)
    #[prop_or_default]
    pub addon_before: Option<Html>,

    /// Add-on after input (button or text)
    #[prop_or_default]
    pub addon_after: Option<Html>,

    /// Size of the input group
    #[prop_or_default]
    pub size: Option<Size>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements (input)
    pub children: Children,
}

/// Input group component
///
/// Enhances inputs with prefix/suffix icons or add-on elements.
///
/// # Accessibility
/// - Maintains input accessibility
/// - Prefix/suffix marked as decorative when appropriate
#[function_component(InputGroup)]
pub fn input_group(props: &InputGroupProps) -> Html {
    let InputGroupProps {
        prefix,
        suffix,
        addon_before,
        addon_after,
        size: _,
        class,
        children,
    } = props.clone();

    let classes: Classes = vec![Classes::from("input-group"), class]
        .into_iter()
        .collect();

    html! {
        <div class={classes}>
            if let Some(addon) = addon_before {
                <div class="input-group-addon input-group-addon-before">
                    { addon }
                </div>
            }
            <div class="input-group-wrapper">
                if let Some(prefix_content) = prefix {
                    <span class="input-group-prefix">
                        { prefix_content }
                    </span>
                }
                { children }
                if let Some(suffix_content) = suffix {
                    <span class="input-group-suffix">
                        { suffix_content }
                    </span>
                }
            </div>
            if let Some(addon) = addon_after {
                <div class="input-group-addon input-group-addon-after">
                    { addon }
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_group_default() {
        let props = InputGroupProps {
            prefix: None,
            suffix: None,
            addon_before: None,
            addon_after: None,
            size: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.prefix, None);
        assert_eq!(props.suffix, None);
    }

    #[test]
    fn test_input_group_with_prefix() {
        let prefix_html = html! { <span>{ "@" }</span> };
        let props = InputGroupProps {
            prefix: Some(prefix_html.clone()),
            suffix: None,
            addon_before: None,
            addon_after: None,
            size: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.prefix.is_some());
    }

    #[test]
    fn test_input_group_with_suffix() {
        let suffix_html = html! { <span>{ ".com" }</span> };
        let props = InputGroupProps {
            prefix: None,
            suffix: Some(suffix_html.clone()),
            addon_before: None,
            addon_after: None,
            size: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.suffix.is_some());
    }

    #[test]
    fn test_input_group_with_addons() {
        let addon_html = html! { <button>{ "Search" }</button> };
        let props = InputGroupProps {
            prefix: None,
            suffix: None,
            addon_before: Some(addon_html.clone()),
            addon_after: Some(addon_html.clone()),
            size: None,
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert!(props.addon_before.is_some());
        assert!(props.addon_after.is_some());
    }

    #[test]
    fn test_input_group_with_size() {
        let props = InputGroupProps {
            prefix: None,
            suffix: None,
            addon_before: None,
            addon_after: None,
            size: Some(Size::Lg),
            class: Classes::new(),
            children: Children::new(vec![]),
        };

        assert_eq!(props.size, Some(Size::Lg));
    }
}
