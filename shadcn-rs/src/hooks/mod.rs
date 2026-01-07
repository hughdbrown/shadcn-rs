//! Custom Yew hooks
//!
//! Reusable hooks for component logic.

pub mod use_click_outside;
pub mod use_controllable_state;
pub mod use_escape_key;
pub mod use_toggle;

// Re-export hooks
pub use use_click_outside::{use_click_outside, use_click_outside_conditional};
pub use use_controllable_state::{
    use_controllable_bool, use_controllable_state, use_controllable_state_optional,
};
pub use use_escape_key::{use_escape_key, use_escape_key_conditional, use_key_press};
pub use use_toggle::{use_toggle, use_toggle_with_controls};
