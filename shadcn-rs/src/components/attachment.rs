//! Attachment component
//!
//! Displays a file or image attachment with media, metadata, upload state, and actions.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{
//!     Attachment, AttachmentMedia, AttachmentContent, AttachmentTitle,
//!     AttachmentDescription, AttachmentActions, AttachmentAction,
//! };
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Attachment>
//!             <AttachmentMedia>
//!                 <span>{ "📄" }</span>
//!             </AttachmentMedia>
//!             <AttachmentContent>
//!                 <AttachmentTitle>{ "document.pdf" }</AttachmentTitle>
//!                 <AttachmentDescription>{ "PDF · 2.4 MB" }</AttachmentDescription>
//!             </AttachmentContent>
//!             <AttachmentActions>
//!                 <AttachmentAction aria_label="Remove document.pdf">
//!                     <span>{ "✕" }</span>
//!                 </AttachmentAction>
//!             </AttachmentActions>
//!         </Attachment>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Status of an attachment upload or processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttachmentStatus {
    /// Default idle state
    #[default]
    Default,
    /// Upload in progress
    Uploading,
    /// Processing / conversion in progress
    Processing,
    /// Upload or processing failed
    Error,
    /// Upload complete
    Done,
}

impl AttachmentStatus {
    /// Convert status to CSS class name
    pub fn to_class(self) -> &'static str {
        match self {
            AttachmentStatus::Default => "",
            AttachmentStatus::Uploading => "attachment-status-uploading",
            AttachmentStatus::Processing => "attachment-status-processing",
            AttachmentStatus::Error => "attachment-status-error",
            AttachmentStatus::Done => "attachment-status-done",
        }
    }
}

/// Size variant for attachments
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttachmentSize {
    /// Standard attachment size
    #[default]
    Default,
    /// Compact attachment size
    Sm,
    /// Extra compact attachment size
    Xs,
}

impl AttachmentSize {
    /// Convert size to CSS class name
    pub fn to_class(self) -> &'static str {
        match self {
            AttachmentSize::Default => "",
            AttachmentSize::Sm => "attachment-size-sm",
            AttachmentSize::Xs => "attachment-size-xs",
        }
    }
}

/// Orientation of the attachment card
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttachmentOrientation {
    /// Horizontal row layout
    #[default]
    Horizontal,
    /// Vertical stacked layout
    Vertical,
}

impl AttachmentOrientation {
    /// Convert orientation to CSS class name
    pub fn to_class(self) -> &'static str {
        match self {
            AttachmentOrientation::Horizontal => "",
            AttachmentOrientation::Vertical => "attachment-vertical",
        }
    }
}

/// Variant for media preview
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AttachmentMediaVariant {
    /// Default icon/file presentation
    #[default]
    Default,
    /// Image thumbnail presentation
    Image,
}

impl AttachmentMediaVariant {
    /// Convert media variant to CSS class name
    pub fn to_class(self) -> &'static str {
        match self {
            AttachmentMediaVariant::Default => "",
            AttachmentMediaVariant::Image => "attachment-media-image",
        }
    }
}

/// Properties for [`Attachment`]
#[derive(Properties, PartialEq, Clone)]
pub struct AttachmentProps {
    /// Upload/processing status
    #[prop_or_default]
    pub status: AttachmentStatus,

    /// Size of the attachment
    #[prop_or_default]
    pub size: AttachmentSize,

    /// Orientation of layout
    #[prop_or_default]
    pub orientation: AttachmentOrientation,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Attachment container component
#[function_component(Attachment)]
pub fn attachment(props: &AttachmentProps) -> Html {
    let classes = classes!(
        "attachment",
        props.status.to_class(),
        props.size.to_class(),
        props.orientation.to_class(),
        props.class.clone()
    );

    html! {
        <div class={classes} role="group">
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`AttachmentMedia`]
#[derive(Properties, PartialEq, Clone)]
pub struct AttachmentMediaProps {
    /// Presentation style of the media preview
    #[prop_or_default]
    pub variant: AttachmentMediaVariant,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Media preview content (icon, img, etc.)
    pub children: Children,
}

/// Media container for thumbnail, icon, or preview
#[function_component(AttachmentMedia)]
pub fn attachment_media(props: &AttachmentMediaProps) -> Html {
    let classes = classes!(
        "attachment-media",
        props.variant.to_class(),
        props.class.clone()
    );

    html! {
        <div class={classes} aria-hidden="true">
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`AttachmentContent`]
#[derive(Properties, PartialEq, Clone)]
pub struct AttachmentContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Text content (title, description)
    pub children: Children,
}

/// Container for attachment title and metadata description
#[function_component(AttachmentContent)]
pub fn attachment_content(props: &AttachmentContentProps) -> Html {
    let classes = classes!("attachment-content", props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`AttachmentTitle`]
#[derive(Properties, PartialEq, Clone)]
pub struct AttachmentTitleProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// File or attachment name
    pub children: Children,
}

/// Attachment file or item title
#[function_component(AttachmentTitle)]
pub fn attachment_title(props: &AttachmentTitleProps) -> Html {
    let classes = classes!("attachment-title", props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`AttachmentDescription`]
#[derive(Properties, PartialEq, Clone)]
pub struct AttachmentDescriptionProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// File metadata (size, format, timestamp)
    pub children: Children,
}

/// Attachment metadata description
#[function_component(AttachmentDescription)]
pub fn attachment_description(props: &AttachmentDescriptionProps) -> Html {
    let classes = classes!("attachment-description", props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`AttachmentActions`]
#[derive(Properties, PartialEq, Clone)]
pub struct AttachmentActionsProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Action buttons
    pub children: Children,
}

/// Container for attachment actions
#[function_component(AttachmentActions)]
pub fn attachment_actions(props: &AttachmentActionsProps) -> Html {
    let classes = classes!("attachment-actions", props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`AttachmentAction`]
#[derive(Properties, PartialEq, Clone)]
pub struct AttachmentActionProps {
    /// Accessible label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Disabled state
    #[prop_or(false)]
    pub disabled: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Button content
    pub children: Children,
}

/// Individual action button inside an attachment
#[function_component(AttachmentAction)]
pub fn attachment_action(props: &AttachmentActionProps) -> Html {
    let classes = classes!("attachment-action", props.class.clone());

    html! {
        <button
            type="button"
            class={classes}
            aria-label={props.aria_label.clone()}
            onclick={props.onclick.clone()}
            disabled={props.disabled}
        >
            { props.children.clone() }
        </button>
    }
}

/// Properties for [`AttachmentTrigger`]
#[derive(Properties, PartialEq, Clone)]
pub struct AttachmentTriggerProps {
    /// Accessible label
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// Click handler
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    #[prop_or_default]
    pub children: Children,
}

/// Trigger element (e.g. for preview dialog or download)
#[function_component(AttachmentTrigger)]
pub fn attachment_trigger(props: &AttachmentTriggerProps) -> Html {
    let classes = classes!("attachment-trigger", props.class.clone());

    html! {
        <button
            type="button"
            class={classes}
            aria-label={props.aria_label.clone()}
            onclick={props.onclick.clone()}
        >
            { props.children.clone() }
        </button>
    }
}

/// Properties for [`AttachmentGroup`]
#[derive(Properties, PartialEq, Clone)]
pub struct AttachmentGroupProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Children elements
    pub children: Children,
}

/// Group container for multiple attachments
#[function_component(AttachmentGroup)]
pub fn attachment_group(props: &AttachmentGroupProps) -> Html {
    let classes = classes!("attachment-group", props.class.clone());

    html! {
        <div class={classes} role="list">
            { props.children.clone() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attachment_classes_have_css() {
        let css = include_str!("../../styles/components.css");
        for class in [
            "attachment",
            "attachment-status-uploading",
            "attachment-status-processing",
            "attachment-status-error",
            "attachment-status-done",
            "attachment-size-sm",
            "attachment-size-xs",
            "attachment-vertical",
            "attachment-media",
            "attachment-media-image",
            "attachment-content",
            "attachment-title",
            "attachment-description",
            "attachment-actions",
            "attachment-action",
            "attachment-trigger",
            "attachment-group",
        ] {
            assert!(css.contains(&format!(".{class} {{")), "missing .{class}");
        }
    }

    #[test]
    fn test_attachment_status_to_class() {
        assert_eq!(AttachmentStatus::Default.to_class(), "");
        assert_eq!(
            AttachmentStatus::Uploading.to_class(),
            "attachment-status-uploading"
        );
        assert_eq!(
            AttachmentStatus::Processing.to_class(),
            "attachment-status-processing"
        );
        assert_eq!(
            AttachmentStatus::Error.to_class(),
            "attachment-status-error"
        );
        assert_eq!(AttachmentStatus::Done.to_class(), "attachment-status-done");
    }

    #[test]
    fn test_attachment_size_to_class() {
        assert_eq!(AttachmentSize::Default.to_class(), "");
        assert_eq!(AttachmentSize::Sm.to_class(), "attachment-size-sm");
        assert_eq!(AttachmentSize::Xs.to_class(), "attachment-size-xs");
    }

    #[test]
    fn test_attachment_orientation_to_class() {
        assert_eq!(AttachmentOrientation::Horizontal.to_class(), "");
        assert_eq!(
            AttachmentOrientation::Vertical.to_class(),
            "attachment-vertical"
        );
    }
}
