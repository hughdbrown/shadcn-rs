//! Message component
//!
//! Displays a message in a conversation, with optional avatar, header, footer,
//! bubble content, and alignment.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{
//!     Message, MessageAvatar, MessageContent, MessageHeader, MessageFooter,
//!     Bubble, BubbleContent, MessageAlign
//! };
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <Message align={MessageAlign::Start}>
//!             <MessageAvatar>
//!                 <span>{ "🤖" }</span>
//!             </MessageAvatar>
//!             <MessageContent>
//!                 <MessageHeader>
//!                     <span class="font-semibold">{ "Assistant" }</span>
//!                 </MessageHeader>
//!                 <Bubble>
//!                     <BubbleContent>{ "How can I assist you?" }</BubbleContent>
//!                 </Bubble>
//!                 <MessageFooter>
//!                     <span class="text-xs text-muted-foreground">{ "Just now" }</span>
//!                 </MessageFooter>
//!             </MessageContent>
//!         </Message>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Horizontal alignment of the message row
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MessageAlign {
    /// Start alignment (typically assistant / incoming message)
    #[default]
    Start,
    /// End alignment (typically user / outgoing message)
    End,
}

impl MessageAlign {
    /// Convert alignment to CSS class name
    pub fn to_class(self) -> &'static str {
        match self {
            MessageAlign::Start => "message-align-start",
            MessageAlign::End => "message-align-end",
        }
    }
}

/// Properties for [`Message`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageProps {
    /// Alignment of message turn
    #[prop_or_default]
    pub align: MessageAlign,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Message subcomponents
    pub children: Children,
}

/// Message row container
#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    let classes = classes!("message", props.align.to_class(), props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`MessageAvatar`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageAvatarProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Avatar or empty placeholder
    #[prop_or_default]
    pub children: Children,
}

/// Container for the sender avatar
#[function_component(MessageAvatar)]
pub fn message_avatar(props: &MessageAvatarProps) -> Html {
    let classes = classes!("message-avatar", props.class.clone());

    html! {
        <div class={classes} aria-hidden="true">
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`MessageContent`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageContentProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Inner message header, bubbles, footer
    pub children: Children,
}

/// Container for message body, bubbles, and metadata
#[function_component(MessageContent)]
pub fn message_content(props: &MessageContentProps) -> Html {
    let classes = classes!("message-content", props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`MessageHeader`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageHeaderProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Sender name, timestamp, or title
    pub children: Children,
}

/// Header for a message (sender name, timestamp)
#[function_component(MessageHeader)]
pub fn message_header(props: &MessageHeaderProps) -> Html {
    let classes = classes!("message-header", props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`MessageFooter`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageFooterProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Status metadata, actions, copy button
    pub children: Children,
}

/// Footer for a message (actions, status indicators)
#[function_component(MessageFooter)]
pub fn message_footer(props: &MessageFooterProps) -> Html {
    let classes = classes!("message-footer", props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`MessageGroup`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageGroupProps {
    /// Alignment of messages in group
    #[prop_or_default]
    pub align: MessageAlign,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Consecutive messages
    pub children: Children,
}

/// Group container for stacking consecutive messages from the same sender
#[function_component(MessageGroup)]
pub fn message_group(props: &MessageGroupProps) -> Html {
    let classes = classes!(
        "message-group",
        match props.align {
            MessageAlign::Start => "message-group-align-start",
            MessageAlign::End => "message-group-align-end",
        },
        props.class.clone()
    );

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_classes_have_css() {
        let css = include_str!("../../styles/components.css");
        for class in [
            "message",
            "message-align-start",
            "message-align-end",
            "message-avatar",
            "message-content",
            "message-header",
            "message-footer",
            "message-group",
            "message-group-align-start",
            "message-group-align-end",
        ] {
            assert!(css.contains(&format!(".{class} {{")), "missing .{class}");
        }
    }

    #[test]
    fn test_message_align_to_class() {
        assert_eq!(MessageAlign::Start.to_class(), "message-align-start");
        assert_eq!(MessageAlign::End.to_class(), "message-align-end");
    }
}
