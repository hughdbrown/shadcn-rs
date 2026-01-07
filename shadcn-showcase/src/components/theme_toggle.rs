//! Theme toggle component for switching between light and dark modes

use web_sys::window;
use yew::prelude::*;

/// Theme options
#[derive(Clone, Copy, PartialEq, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
}

impl Theme {
    fn as_str(&self) -> &'static str {
        match self {
            Theme::Light => "light",
            Theme::Dark => "dark",
        }
    }

    fn toggle(&self) -> Self {
        match self {
            Theme::Light => Theme::Dark,
            Theme::Dark => Theme::Light,
        }
    }
}

/// Properties for ThemeToggle component
#[derive(Properties, PartialEq)]
pub struct ThemeToggleProps {
    /// Optional class name
    #[prop_or_default]
    pub class: Classes,
}

/// Theme toggle button component
#[function_component(ThemeToggle)]
pub fn theme_toggle(props: &ThemeToggleProps) -> Html {
    let theme = use_state(|| {
        // Try to get theme from localStorage or system preference
        if let Some(window) = window()
            && let Ok(Some(storage)) = window.local_storage()
            && let Ok(Some(stored)) = storage.get_item("theme")
        {
            return if stored == "dark" {
                Theme::Dark
            } else {
                Theme::Light
            };
        }
        // Check system preference
        if let Some(window) = window()
            && let Ok(Some(query)) = window.match_media("(prefers-color-scheme: dark)")
            && query.matches()
        {
            return Theme::Dark;
        }
        Theme::Light
    });

    // Apply theme on mount and changes
    {
        let theme = *theme;
        use_effect_with(theme, move |theme| {
            if let Some(window) = window()
                && let Some(document) = window.document()
                && let Some(root) = document.document_element()
            {
                let _ = root.set_attribute("data-theme", theme.as_str());
            }
            if let Some(window) = window()
                && let Ok(Some(storage)) = window.local_storage()
            {
                let _ = storage.set_item("theme", theme.as_str());
            }
            || ()
        });
    }

    let onclick = {
        let theme = theme.clone();
        Callback::from(move |_: MouseEvent| {
            theme.set(theme.toggle());
        })
    };

    html! {
        <button
            class={classes!("theme-toggle", props.class.clone())}
            onclick={onclick}
            title={if *theme == Theme::Dark { "Switch to light mode" } else { "Switch to dark mode" }}
            aria-label="Toggle theme"
        >
            if *theme == Theme::Dark {
                // Sun icon
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <circle cx="12" cy="12" r="5"></circle>
                    <line x1="12" y1="1" x2="12" y2="3"></line>
                    <line x1="12" y1="21" x2="12" y2="23"></line>
                    <line x1="4.22" y1="4.22" x2="5.64" y2="5.64"></line>
                    <line x1="18.36" y1="18.36" x2="19.78" y2="19.78"></line>
                    <line x1="1" y1="12" x2="3" y2="12"></line>
                    <line x1="21" y1="12" x2="23" y2="12"></line>
                    <line x1="4.22" y1="19.78" x2="5.64" y2="18.36"></line>
                    <line x1="18.36" y1="5.64" x2="19.78" y2="4.22"></line>
                </svg>
            } else {
                // Moon icon
                <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z"></path>
                </svg>
            }
        </button>
    }
}
