//! Sidebar component showcase page

use yew::prelude::*;
use shadcn_rs::{Sidebar as SidebarComponent, SidebarHeader, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupLabel, SidebarGroupContent, SidebarMenu, SidebarMenuItem, SidebarMenuButton};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(SidebarPage)]
pub fn sidebar_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A sidebar navigation component.",
            demo: html! {
                <div class="border rounded-lg h-[400px] w-[250px] overflow-hidden">
                    <SidebarComponent>
                        <SidebarHeader>
                            <div class="px-2 py-4 font-semibold">{ "App Name" }</div>
                        </SidebarHeader>
                        <SidebarContent>
                            <SidebarGroup>
                                <SidebarGroupLabel>{ "Main Menu" }</SidebarGroupLabel>
                                <SidebarGroupContent>
                                    <SidebarMenu>
                                        <SidebarMenuItem>
                                            <SidebarMenuButton active={true}>{ "Dashboard" }</SidebarMenuButton>
                                        </SidebarMenuItem>
                                        <SidebarMenuItem>
                                            <SidebarMenuButton>{ "Projects" }</SidebarMenuButton>
                                        </SidebarMenuItem>
                                        <SidebarMenuItem>
                                            <SidebarMenuButton>{ "Tasks" }</SidebarMenuButton>
                                        </SidebarMenuItem>
                                        <SidebarMenuItem>
                                            <SidebarMenuButton>{ "Settings" }</SidebarMenuButton>
                                        </SidebarMenuItem>
                                    </SidebarMenu>
                                </SidebarGroupContent>
                            </SidebarGroup>
                        </SidebarContent>
                        <SidebarFooter>
                            <div class="px-2 py-4 text-sm text-muted-foreground">{ "v1.0.0" }</div>
                        </SidebarFooter>
                    </SidebarComponent>
                </div>
            },
            code: r#"<Sidebar>
    <SidebarHeader>{ "App Name" }</SidebarHeader>
    <SidebarContent>
        <SidebarGroup>
            <SidebarGroupLabel>{ "Menu" }</SidebarGroupLabel>
            <SidebarGroupContent>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton>{ "Item" }</SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarGroupContent>
        </SidebarGroup>
    </SidebarContent>
    <SidebarFooter>{ "Footer" }</SidebarFooter>
</Sidebar>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "collapsed", prop_type: "bool", default: "false", description: "Whether sidebar is collapsed" },
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Sidebar content" },
    ];

    html! { <ComponentPage name="Sidebar" description="A composable sidebar component." {examples} {props} /> }
}
