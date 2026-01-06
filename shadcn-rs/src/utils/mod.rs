//! Utility functions and helpers
//!
//! Common utilities used across components.

pub mod aria;
pub mod class_name;
pub mod portal;
pub mod touch;

// Re-export commonly used utilities
pub use aria::{
    generate_id, generate_id_with_separator, aria_list, aria_labelledby, aria_describedby,
    AriaLive, AriaAutoComplete, AriaCurrent,
};
pub use class_name::{
    class_names, class_if, merge_classes, classes_optional,
};
pub use portal::{Portal, create_portal, use_portal};
pub use touch::{
    SwipeDirection, SwipeConfig, TouchPoint, detect_swipe,
    touch_point_from_event, get_first_touch,
};
