//! Sidebar navigation component

use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::{get_nav_groups, Route};

/// Properties for Sidebar component
#[derive(Properties, PartialEq)]
pub struct SidebarProps {
    /// Whether the sidebar is open (for mobile)
    #[prop_or(true)]
    pub open: bool,
    /// Callback when sidebar should close (mobile)
    #[prop_or_default]
    pub on_close: Callback<()>,
}

/// Navigation sidebar component
#[function_component(Sidebar)]
pub fn sidebar(props: &SidebarProps) -> Html {
    let nav_groups = get_nav_groups();
    let current_route = use_route::<Route>();

    let collapsed_groups = use_state(|| std::collections::HashSet::<String>::new());

    let toggle_group = {
        let collapsed_groups = collapsed_groups.clone();
        Callback::from(move |group_title: String| {
            let mut new_collapsed = (*collapsed_groups).clone();
            if new_collapsed.contains(&group_title) {
                new_collapsed.remove(&group_title);
            } else {
                new_collapsed.insert(group_title);
            }
            collapsed_groups.set(new_collapsed);
        })
    };

    html! {
        <aside class={classes!("sidebar", props.open.then_some("sidebar-open"))}>
            <div class="sidebar-header">
                <Link<Route> to={Route::Home} classes="sidebar-logo">
                    <span class="sidebar-logo-text">{ "shadcn-rs" }</span>
                </Link<Route>>
            </div>

            <nav class="sidebar-nav">
                { for nav_groups.iter().map(|group| {
                    let is_collapsed = collapsed_groups.contains(group.title);
                    let group_title = group.title.to_string();
                    let toggle = {
                        let toggle_group = toggle_group.clone();
                        let title = group_title.clone();
                        Callback::from(move |_: MouseEvent| {
                            toggle_group.emit(title.clone());
                        })
                    };

                    html! {
                        <div class="sidebar-group">
                            <button
                                class="sidebar-group-header"
                                onclick={toggle}
                                aria-expanded={(!is_collapsed).to_string()}
                            >
                                <span>{ group.title }</span>
                                <svg
                                    class={classes!("sidebar-chevron", (!is_collapsed).then_some("sidebar-chevron-open"))}
                                    xmlns="http://www.w3.org/2000/svg"
                                    width="16"
                                    height="16"
                                    viewBox="0 0 24 24"
                                    fill="none"
                                    stroke="currentColor"
                                    stroke-width="2"
                                    stroke-linecap="round"
                                    stroke-linejoin="round"
                                >
                                    <polyline points="6 9 12 15 18 9"></polyline>
                                </svg>
                            </button>
                            if !is_collapsed {
                                <ul class="sidebar-items">
                                    { for group.items.iter().map(|item| {
                                        let is_active = current_route.as_ref() == Some(&item.route);
                                        html! {
                                            <li class="sidebar-item">
                                                <Link<Route>
                                                    to={item.route.clone()}
                                                    classes={classes!("sidebar-link", is_active.then_some("sidebar-link-active"))}
                                                >
                                                    { item.label }
                                                </Link<Route>>
                                            </li>
                                        }
                                    })}
                                </ul>
                            }
                        </div>
                    }
                })}
            </nav>

            <div class="sidebar-footer">
                <a
                    href="https://github.com/hughdbrown/shadcn-rs"
                    target="_blank"
                    rel="noopener noreferrer"
                    class="sidebar-link"
                >
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="currentColor">
                        <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                    </svg>
                    <span>{ "GitHub" }</span>
                </a>
            </div>
        </aside>
    }
}
