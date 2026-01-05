//! Alignment enumeration for content alignment

/// Content alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    /// Start alignment (left in LTR, right in RTL)
    #[default]
    Start,
    /// Center alignment
    Center,
    /// End alignment (right in LTR, left in RTL)
    End,
    /// Stretch to fill container
    Stretch,
}

impl Alignment {
    /// Convert alignment to CSS class name
    pub fn to_class(&self) -> &'static str {
        match self {
            Alignment::Start => "align-start",
            Alignment::Center => "align-center",
            Alignment::End => "align-end",
            Alignment::Stretch => "align-stretch",
        }
    }
}
