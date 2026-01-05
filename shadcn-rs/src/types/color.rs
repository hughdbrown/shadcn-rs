//! Color enumeration for component theming

/// Component color options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Color {
    /// Default color (from theme)
    #[default]
    Default,
    /// Primary color
    Primary,
    /// Secondary color
    Secondary,
    /// Success color (green)
    Success,
    /// Warning color (yellow/orange)
    Warning,
    /// Danger/Error color (red)
    Danger,
    /// Info color (blue)
    Info,
}

impl Color {
    /// Convert color to CSS class name
    pub fn to_class(&self) -> &'static str {
        match self {
            Color::Default => "color-default",
            Color::Primary => "color-primary",
            Color::Secondary => "color-secondary",
            Color::Success => "color-success",
            Color::Warning => "color-warning",
            Color::Danger => "color-danger",
            Color::Info => "color-info",
        }
    }
}
