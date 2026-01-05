//! Position enumeration for component placement

/// Component positioning options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Position {
    /// Top position
    Top,
    /// Right position
    Right,
    /// Bottom position (default)
    #[default]
    Bottom,
    /// Left position
    Left,
    /// Top-left corner
    TopLeft,
    /// Top-right corner
    TopRight,
    /// Bottom-left corner
    BottomLeft,
    /// Bottom-right corner
    BottomRight,
    /// Center position
    Center,
}

impl Position {
    /// Convert position to CSS class name
    pub fn to_class(&self) -> &'static str {
        match self {
            Position::Top => "position-top",
            Position::Right => "position-right",
            Position::Bottom => "position-bottom",
            Position::Left => "position-left",
            Position::TopLeft => "position-top-left",
            Position::TopRight => "position-top-right",
            Position::BottomLeft => "position-bottom-left",
            Position::BottomRight => "position-bottom-right",
            Position::Center => "position-center",
        }
    }
}
