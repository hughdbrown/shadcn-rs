//! Size enumeration for component sizing

/// Standard component sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Size {
    /// Extra small size
    Xs,
    /// Small size
    Sm,
    /// Medium size (default)
    #[default]
    Md,
    /// Large size
    Lg,
    /// Extra large size
    Xl,
    /// 2X large size
    Xl2,
}

impl Size {
    /// Convert size to CSS class name
    pub fn to_class(&self) -> &'static str {
        match self {
            Size::Xs => "size-xs",
            Size::Sm => "size-sm",
            Size::Md => "size-md",
            Size::Lg => "size-lg",
            Size::Xl => "size-xl",
            Size::Xl2 => "size-2xl",
        }
    }
}
