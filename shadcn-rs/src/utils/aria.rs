//! ARIA (Accessible Rich Internet Applications) utilities
//!
//! Helper functions for managing ARIA attributes and accessibility.

use std::sync::atomic::{AtomicUsize, Ordering};

/// Global counter for generating unique IDs
static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// Generate a unique ID for ARIA attributes
///
/// This is useful for `aria-labelledby`, `aria-describedby`, and other
/// attributes that reference element IDs.
///
/// # Examples
///
/// ```
/// use shadcn_rs::utils::generate_id;
///
/// let id1 = generate_id("input");
/// let id2 = generate_id("input");
/// assert_ne!(id1, id2); // Each call generates a unique ID
/// assert!(id1.starts_with("input-"));
/// ```
pub fn generate_id(prefix: &str) -> String {
    let count = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}-{}", prefix, count)
}

/// Generate a unique ID with a custom separator
///
/// # Examples
///
/// ```
/// use shadcn_rs::utils::generate_id_with_separator;
///
/// let id = generate_id_with_separator("label", "_");
/// assert!(id.starts_with("label_"));
/// ```
pub fn generate_id_with_separator(prefix: &str, separator: &str) -> String {
    let count = ID_COUNTER.fetch_add(1, Ordering::Relaxed);
    format!("{}{}{}", prefix, separator, count)
}

/// ARIA live region politeness levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaLive {
    /// Changes are announced immediately
    Assertive,
    /// Changes are announced at the next opportunity
    Polite,
    /// Changes are not announced
    Off,
}

impl AriaLive {
    /// Convert to ARIA attribute value
    pub fn to_attr(&self) -> &'static str {
        match self {
            AriaLive::Assertive => "assertive",
            AriaLive::Polite => "polite",
            AriaLive::Off => "off",
        }
    }
}

/// ARIA autocomplete values
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaAutoComplete {
    /// No autocomplete
    None,
    /// Inline autocomplete
    Inline,
    /// List autocomplete
    List,
    /// Both inline and list
    Both,
}

impl AriaAutoComplete {
    /// Convert to ARIA attribute value
    pub fn to_attr(&self) -> &'static str {
        match self {
            AriaAutoComplete::None => "none",
            AriaAutoComplete::Inline => "inline",
            AriaAutoComplete::List => "list",
            AriaAutoComplete::Both => "both",
        }
    }
}

/// ARIA current values for navigation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AriaCurrent {
    /// Current page in navigation
    Page,
    /// Current step in a process
    Step,
    /// Current location
    Location,
    /// Current date
    Date,
    /// Current time
    Time,
    /// True (generic current)
    True,
    /// False (not current)
    False,
}

impl AriaCurrent {
    /// Convert to ARIA attribute value
    pub fn to_attr(&self) -> &'static str {
        match self {
            AriaCurrent::Page => "page",
            AriaCurrent::Step => "step",
            AriaCurrent::Location => "location",
            AriaCurrent::Date => "date",
            AriaCurrent::Time => "time",
            AriaCurrent::True => "true",
            AriaCurrent::False => "false",
        }
    }
}

/// Helper to build space-separated ARIA attribute lists
///
/// # Examples
///
/// ```
/// use shadcn_rs::utils::aria_list;
///
/// let ids = aria_list(&["id1", "id2", "id3"]);
/// assert_eq!(ids, "id1 id2 id3");
/// ```
pub fn aria_list(items: &[&str]) -> String {
    items.join(" ")
}

/// Helper to build ARIA labelledby from multiple IDs
///
/// # Examples
///
/// ```
/// use shadcn_rs::utils::aria_labelledby;
///
/// let attr = aria_labelledby(&["title-1", "subtitle-2"]);
/// assert_eq!(attr, "title-1 subtitle-2");
/// ```
pub fn aria_labelledby(ids: &[&str]) -> String {
    aria_list(ids)
}

/// Helper to build ARIA describedby from multiple IDs
///
/// # Examples
///
/// ```
/// use shadcn_rs::utils::aria_describedby;
///
/// let attr = aria_describedby(&["description-1", "help-text-2"]);
/// assert_eq!(attr, "description-1 help-text-2");
/// ```
pub fn aria_describedby(ids: &[&str]) -> String {
    aria_list(ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_id() {
        let id1 = generate_id("test");
        let id2 = generate_id("test");

        assert!(id1.starts_with("test-"));
        assert!(id2.starts_with("test-"));
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_generate_id_with_separator() {
        let id = generate_id_with_separator("custom", "_");
        assert!(id.starts_with("custom_"));

        let id2 = generate_id_with_separator("another", "::");
        assert!(id2.starts_with("another::"));
    }

    #[test]
    fn test_aria_live_to_attr() {
        assert_eq!(AriaLive::Assertive.to_attr(), "assertive");
        assert_eq!(AriaLive::Polite.to_attr(), "polite");
        assert_eq!(AriaLive::Off.to_attr(), "off");
    }

    #[test]
    fn test_aria_autocomplete_to_attr() {
        assert_eq!(AriaAutoComplete::None.to_attr(), "none");
        assert_eq!(AriaAutoComplete::Inline.to_attr(), "inline");
        assert_eq!(AriaAutoComplete::List.to_attr(), "list");
        assert_eq!(AriaAutoComplete::Both.to_attr(), "both");
    }

    #[test]
    fn test_aria_current_to_attr() {
        assert_eq!(AriaCurrent::Page.to_attr(), "page");
        assert_eq!(AriaCurrent::Step.to_attr(), "step");
        assert_eq!(AriaCurrent::True.to_attr(), "true");
        assert_eq!(AriaCurrent::False.to_attr(), "false");
    }

    #[test]
    fn test_aria_list() {
        assert_eq!(aria_list(&["a", "b", "c"]), "a b c");
        assert_eq!(aria_list(&["single"]), "single");
        assert_eq!(aria_list(&[]), "");
    }

    #[test]
    fn test_aria_labelledby() {
        let result = aria_labelledby(&["label-1", "label-2"]);
        assert_eq!(result, "label-1 label-2");
    }

    #[test]
    fn test_aria_describedby() {
        let result = aria_describedby(&["desc-1", "desc-2", "desc-3"]);
        assert_eq!(result, "desc-1 desc-2 desc-3");
    }
}
