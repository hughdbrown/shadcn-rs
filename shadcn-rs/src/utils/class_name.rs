//! CSS class name utilities
//!
//! Helpers for combining and manipulating CSS class names.

use yew::Classes;

/// Combine multiple class names, filtering out empty strings and None values
///
/// # Examples
///
/// ```
/// use shadcn_rs::utils::class_names;
/// use yew::Classes;
///
/// let classes = class_names(&[
///     Some("btn"),
///     Some("btn-primary"),
///     None, // Filtered out
/// ]);
/// assert_eq!(classes.to_string(), "btn btn-primary");
/// ```
pub fn class_names(classes: &[Option<&str>]) -> Classes {
    classes
        .iter()
        .filter_map(|&c| c)
        .filter(|c| !c.is_empty())
        .map(|s| s.to_string())
        .collect::<Vec<_>>()
        .into()
}

/// Conditionally include a class name
///
/// # Examples
///
/// ```
/// use shadcn_rs::utils::class_if;
///
/// assert_eq!(class_if(true, "active"), Some("active"));
/// assert_eq!(class_if(false, "active"), None);
/// ```
pub fn class_if(condition: bool, class_name: &str) -> Option<&str> {
    if condition { Some(class_name) } else { None }
}

/// Combine multiple Classes instances
///
/// # Examples
///
/// ```
/// use shadcn_rs::utils::merge_classes;
/// use yew::Classes;
///
/// let classes1 = Classes::from("btn");
/// let classes2 = Classes::from("btn-primary");
/// let merged = merge_classes(&[classes1, classes2]);
///
/// assert!(merged.contains("btn"));
/// assert!(merged.contains("btn-primary"));
/// ```
pub fn merge_classes(classes_list: &[Classes]) -> Classes {
    let mut result = Classes::new();
    for classes in classes_list {
        result.extend(classes.clone());
    }
    result
}

/// Create a Classes instance from optional class names
///
/// # Examples
///
/// ```
/// use shadcn_rs::utils::classes_optional;
///
/// let classes = classes_optional(&[
///     Some("btn"),
///     None,
///     Some("btn-lg"),
/// ]);
/// assert_eq!(classes.to_string(), "btn btn-lg");
/// ```
pub fn classes_optional(classes: &[Option<&str>]) -> Classes {
    class_names(classes)
}

/// Helper macro for building class names with conditions
///
/// # Examples
///
/// ```
/// use shadcn_rs::class_list;
///
/// let is_active = true;
/// let is_disabled = false;
///
/// let classes = class_list![
///     "btn",
///     "btn-primary";
///     is_active => "active",
///     is_disabled => "disabled",
/// ];
/// ```
#[macro_export]
macro_rules! class_list {
    // Base case: just strings
    ($($class:expr),* $(,)?) => {
        {
            use yew::Classes;
            let mut classes = Classes::new();
            $(
                classes.push($class);
            )*
            classes
        }
    };

    // With conditions: condition => class (separated by semicolon)
    ($($class:expr),* ; $($cond:expr => $cond_class:expr),* $(,)?) => {
        {
            use yew::Classes;
            let mut classes = Classes::new();
            $(
                classes.push($class);
            )*
            $(
                if $cond {
                    classes.push($cond_class);
                }
            )*
            classes
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_class_names() {
        let result = class_names(&[Some("a"), Some("b"), None, Some("c")]);
        assert!(result.contains("a"));
        assert!(result.contains("b"));
        assert!(result.contains("c"));
    }

    #[test]
    fn test_class_names_filters_empty() {
        let result = class_names(&[Some(""), Some("valid"), None]);
        assert!(result.contains("valid"));
        assert!(!result.contains(""));
    }

    #[test]
    fn test_class_if() {
        assert_eq!(class_if(true, "test"), Some("test"));
        assert_eq!(class_if(false, "test"), None);
    }

    #[test]
    fn test_merge_classes() {
        let c1 = Classes::from("a");
        let c2 = Classes::from("b");
        let c3 = Classes::from("c");

        let merged = merge_classes(&[c1, c2, c3]);
        assert!(merged.contains("a"));
        assert!(merged.contains("b"));
        assert!(merged.contains("c"));
    }

    #[test]
    fn test_classes_optional() {
        let result = classes_optional(&[Some("x"), None, Some("y")]);
        assert!(result.contains("x"));
        assert!(result.contains("y"));
    }

    #[test]
    fn test_class_list_macro() {
        let classes = class_list!["a", "b", "c"];
        assert!(classes.contains("a"));
        assert!(classes.contains("b"));
        assert!(classes.contains("c"));
    }

    #[test]
    fn test_class_list_macro_with_conditions() {
        let is_active = true;
        let is_disabled = false;

        let classes = class_list![
            "base";
            is_active => "active",
            is_disabled => "disabled",
        ];

        assert!(classes.contains("base"));
        assert!(classes.contains("active"));
        assert!(!classes.contains("disabled"));
    }
}
