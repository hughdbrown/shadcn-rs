//! Sidebar component showcase page

use shadcn_rs::{
    Sidebar as SidebarComponent, SidebarContent, SidebarFooter, SidebarGroup, SidebarGroupContent,
    SidebarGroupLabel, SidebarHeader, SidebarInset, SidebarMenu, SidebarMenuButton,
    SidebarMenuItem, SidebarProvider, SidebarTrigger,
};
use yew::prelude::*;

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
        Example {
            title: "With Provider",
            description: "SidebarTrigger toggles the provider; the Sidebar collapses and expands with it.",
            demo: html! {
                <div class="border rounded-lg h-[300px] overflow-hidden">
                    <SidebarProvider>
                        <SidebarComponent>
                            <SidebarContent>
                                <SidebarMenu>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton active={true}>{ "Inbox" }</SidebarMenuButton>
                                    </SidebarMenuItem>
                                    <SidebarMenuItem>
                                        <SidebarMenuButton>{ "Drafts" }</SidebarMenuButton>
                                    </SidebarMenuItem>
                                </SidebarMenu>
                            </SidebarContent>
                        </SidebarComponent>
                        <SidebarInset>
                            <div class="p-4">
                                <SidebarTrigger>{ "Toggle sidebar" }</SidebarTrigger>
                            </div>
                        </SidebarInset>
                    </SidebarProvider>
                </div>
            },
            code: r#"<SidebarProvider>
    <Sidebar>{ /* ... */ }</Sidebar>
    <SidebarInset>
        <SidebarTrigger>{ "Toggle sidebar" }</SidebarTrigger>
    </SidebarInset>
</SidebarProvider>"#,
        },
    ];

    let props = vec![
        PropDoc {
            name: "collapsed",
            prop_type: "bool",
            default: "false",
            description: "Whether sidebar is collapsed",
        },
        PropDoc {
            name: "class",
            prop_type: "Classes",
            default: "-",
            description: "Additional CSS classes",
        },
        PropDoc {
            name: "children",
            prop_type: "Children",
            default: "-",
            description: "Sidebar content",
        },
    ];

    html! { <ComponentPage name="Sidebar" description="A composable sidebar component." {examples} {props} /> }
}
