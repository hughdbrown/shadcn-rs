//! Utility functions and helpers
//!
//! Common utilities used across components.

pub mod aria;
pub mod class_name;
pub mod portal;
pub mod touch;

// Re-export commonly used utilities
pub use aria::{
    AriaAutoComplete, AriaCurrent, AriaLive, aria_describedby, aria_labelledby, aria_list,
    generate_id, generate_id_with_separator,
};
pub use class_name::{class_if, class_names, classes_optional, merge_classes};
pub use portal::{Portal, create_portal, use_portal};
pub use touch::{
    SwipeConfig, SwipeDirection, TouchPoint, detect_swipe, get_first_touch, touch_point_from_event,
};
