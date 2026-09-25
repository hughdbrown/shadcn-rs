//! Message Scroller component
//!
//! A chat scroll container that anchors turns, follows streamed responses,
//! and provides smooth scrolling with a scroll-to-bottom action.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{
//!     MessageScrollerProvider, MessageScroller, MessageScrollerViewport,
//!     MessageScrollerContent, MessageScrollerItem, MessageScrollerButton,
//! };
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     html! {
//!         <MessageScrollerProvider auto_scroll=true>
//!             <MessageScroller>
//!                 <MessageScrollerViewport>
//!                     <MessageScrollerContent>
//!                         <MessageScrollerItem message_id="msg-1" scroll_anchor=true>
//!                             <div>{ "Message 1" }</div>
//!                         </MessageScrollerItem>
//!                     </MessageScrollerContent>
//!                 </MessageScrollerViewport>
//!                 <MessageScrollerButton />
//!             </MessageScroller>
//!         </MessageScrollerProvider>
//!     }
//! }
//! ```

use yew::prelude::*;

/// Shared context for message scroller state and controls
#[derive(Clone, PartialEq)]
pub struct MessageScrollerContext {
    /// Scroll to the bottom of the message viewport
    pub scroll_to_bottom: Callback<()>,
    /// Scroll to the top of the message viewport
    pub scroll_to_top: Callback<()>,
    /// Whether auto-scroll is enabled
    pub auto_scroll: bool,
    /// Whether the viewport is currently scrolled to the bottom
    pub is_at_bottom: bool,
}

/// Hook to consume the [`MessageScrollerContext`]
#[hook]
pub fn use_message_scroller() -> MessageScrollerContext {
    use_context::<MessageScrollerContext>().unwrap_or_else(|| MessageScrollerContext {
        scroll_to_bottom: Callback::noop(),
        scroll_to_top: Callback::noop(),
        auto_scroll: true,
        is_at_bottom: true,
    })
}

/// Properties for [`MessageScrollerProvider`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageScrollerProviderProps {
    /// Whether to automatically scroll down when new messages arrive
    #[prop_or(true)]
    pub auto_scroll: bool,

    /// Children elements
    pub children: Children,
}

/// Context provider for chat scrolling behaviors
#[function_component(MessageScrollerProvider)]
pub fn message_scroller_provider(props: &MessageScrollerProviderProps) -> Html {
    let is_at_bottom = use_state(|| true);

    let scroll_to_bottom = {
        let is_at_bottom = is_at_bottom.clone();
        Callback::from(move |()| {
            is_at_bottom.set(true);
        })
    };

    let scroll_to_top = {
        let is_at_bottom = is_at_bottom.clone();
        Callback::from(move |()| {
            is_at_bottom.set(false);
        })
    };

    let context = MessageScrollerContext {
        scroll_to_bottom,
        scroll_to_top,
        auto_scroll: props.auto_scroll,
        is_at_bottom: *is_at_bottom,
    };

    html! {
        <ContextProvider<MessageScrollerContext> context={context}>
            { props.children.clone() }
        </ContextProvider<MessageScrollerContext>>
    }
}

/// Properties for [`MessageScroller`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageScrollerProps {
    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Scroller components
    pub children: Children,
}

/// Root scroll container
#[function_component(MessageScroller)]
pub fn message_scroller(props: &MessageScrollerProps) -> Html {
    let classes = classes!("message-scroller", props.class.clone());

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`MessageScrollerViewport`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageScrollerViewportProps {
    /// Optional element ID
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Node reference for DOM element access
    #[prop_or_default]
    pub node_ref: NodeRef,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Scroller content
    pub children: Children,
}

/// Scrollable viewport element
#[function_component(MessageScrollerViewport)]
pub fn message_scroller_viewport(props: &MessageScrollerViewportProps) -> Html {
    let classes = classes!("message-scroller-viewport", props.class.clone());

    html! {
        <div
            id={props.id.clone()}
            ref={props.node_ref.clone()}
            class={classes}
            tabindex="0"
        >
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`MessageScrollerContent`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageScrollerContentProps {
    /// Indicates streamed responses in progress
    #[prop_or_default]
    pub aria_busy: Option<bool>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// List of message items
    pub children: Children,
}

/// Content wrapper inside the scroller viewport
#[function_component(MessageScrollerContent)]
pub fn message_scroller_content(props: &MessageScrollerContentProps) -> Html {
    let classes = classes!("message-scroller-content", props.class.clone());

    let busy_str = props.aria_busy.map(|b| if b { "true" } else { "false" });

    html! {
        <div class={classes} aria-busy={busy_str}>
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`MessageScrollerItem`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageScrollerItemProps {
    /// Identifier for this message turn
    #[prop_or_default]
    pub message_id: Option<AttrValue>,

    /// Whether this turn should act as a scroll anchor
    #[prop_or(false)]
    pub scroll_anchor: bool,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Row content (message, marker, etc.)
    pub children: Children,
}

/// Message row container inside the scroller
#[function_component(MessageScrollerItem)]
pub fn message_scroller_item(props: &MessageScrollerItemProps) -> Html {
    let classes = classes!("message-scroller-item", props.class.clone());

    html! {
        <div
            class={classes}
            data-message-id={props.message_id.clone()}
            data-scroll-anchor={if props.scroll_anchor { Some("true") } else { None }}
        >
            { props.children.clone() }
        </div>
    }
}

/// Properties for [`MessageScrollerButton`]
#[derive(Properties, PartialEq, Clone)]
pub struct MessageScrollerButtonProps {
    /// Accessible label
    #[prop_or(AttrValue::from("Scroll to bottom"))]
    pub aria_label: AttrValue,

    /// Click callback (defaults to scroller context `scroll_to_bottom`)
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,

    /// Custom button content (defaults to downward arrow)
    #[prop_or_default]
    pub children: Option<Children>,
}

/// Floating button to scroll directly to the latest message
#[function_component(MessageScrollerButton)]
pub fn message_scroller_button(props: &MessageScrollerButtonProps) -> Html {
    let context = use_message_scroller();

    let onclick = {
        let custom_click = props.onclick.clone();
        let scroll_cb = context.scroll_to_bottom;
        Callback::from(move |e: MouseEvent| {
            if let Some(ref cb) = custom_click {
                cb.emit(e);
            } else {
                scroll_cb.emit(());
            }
        })
    };

    let classes = classes!("message-scroller-button", props.class.clone());

    html! {
        <button
            type="button"
            class={classes}
            aria-label={props.aria_label.clone()}
            {onclick}
        >
            if let Some(ref custom_children) = props.children {
                { custom_children.clone() }
            } else {
                <span class="message-scroller-arrow" aria-hidden="true">{ "↓" }</span>
            }
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_scroller_classes_have_css() {
        let css = include_str!("../../styles/components.css");
        for class in [
            "message-scroller",
            "message-scroller-viewport",
            "message-scroller-content",
            "message-scroller-item",
            "message-scroller-button",
        ] {
            assert!(css.contains(&format!(".{class} {{")), "missing .{class}");
        }
    }
}
