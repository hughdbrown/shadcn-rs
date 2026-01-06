//! UI Components
//!
//! This module contains all shadcn-rs UI components organized by tier.

// Tier 1 - Foundational components
pub mod alert;
pub mod badge;
pub mod button;
pub mod card;
pub mod separator;
pub mod skeleton;

// Tier 2 - Form components
pub mod checkbox;
pub mod form;
pub mod input;
pub mod label;
pub mod radio;
pub mod select;
pub mod switch;
pub mod textarea;

// Re-export Tier 1 components
pub use alert::{Alert, AlertDescription, AlertTitle};
pub use badge::Badge;
pub use button::Button;
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use separator::{Separator, SeparatorOrientation};
pub use skeleton::{Skeleton, SkeletonShape};

// Re-export Tier 2 components
pub use checkbox::Checkbox;
pub use form::{Form, FormField, FormMessage, FormMessageType};
pub use input::Input;
pub use label::Label;
pub use radio::{Radio, RadioGroup};
pub use select::Select;
pub use switch::Switch;
pub use textarea::{Textarea, TextareaResize};
