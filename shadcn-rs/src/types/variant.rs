//! Variant enumeration for component styling

/// Component visual variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Variant {
    /// Default variant
    #[default]
    Default,
    /// Primary variant (accent color)
    Primary,
    /// Secondary variant (muted)
    Secondary,
    /// Destructive variant (danger/error)
    Destructive,
    /// Outline variant (bordered)
    Outline,
    /// Ghost variant (minimal)
    Ghost,
    /// Link variant (text-only)
    Link,
}

impl Variant {
    /// Convert variant to CSS class name
    pub fn to_class(&self) -> &'static str {
        match self {
            Variant::Default => "variant-default",
            Variant::Primary => "variant-primary",
            Variant::Secondary => "variant-secondary",
            Variant::Destructive => "variant-destructive",
            Variant::Outline => "variant-outline",
            Variant::Ghost => "variant-ghost",
            Variant::Link => "variant-link",
        }
    }
}
