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

// Re-export Tier 1 components
pub use alert::{Alert, AlertDescription, AlertTitle};
pub use badge::Badge;
pub use button::Button;
pub use card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
pub use separator::{Separator, SeparatorOrientation};
pub use skeleton::{Skeleton, SkeletonShape};
