//! Tabs component showcase page

use shadcn_rs::{
    Button, Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle, Input, Label,
    Tabs, TabsContent, TabsList, TabsTrigger,
};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(TabsPage)]
pub fn tabs_page() -> Html {
    let examples = vec![Example {
        title: "Default",
        description: "A basic tabs component.",
        demo: html! {
            <Tabs default_value="account" class="w-[400px]">
                <TabsList>
                    <TabsTrigger value="account">{ "Account" }</TabsTrigger>
                    <TabsTrigger value="password">{ "Password" }</TabsTrigger>
                </TabsList>
                <TabsContent value="account">
                    <Card>
                        <CardHeader>
                            <CardTitle>{ "Account" }</CardTitle>
                            <CardDescription>
                                { "Make changes to your account here." }
                            </CardDescription>
                        </CardHeader>
                        <CardContent class="space-y-2">
                            <div class="space-y-1">
                                <Label html_for="name">{ "Name" }</Label>
                                <Input id="name" value="Pedro Duarte" />
                            </div>
                        </CardContent>
                        <CardFooter>
                            <Button>{ "Save changes" }</Button>
                        </CardFooter>
                    </Card>
                </TabsContent>
                <TabsContent value="password">
                    <Card>
                        <CardHeader>
                            <CardTitle>{ "Password" }</CardTitle>
                            <CardDescription>
                                { "Change your password here." }
                            </CardDescription>
                        </CardHeader>
                        <CardContent class="space-y-2">
                            <div class="space-y-1">
                                <Label html_for="current">{ "Current password" }</Label>
                                <Input id="current" r#type="password" />
                            </div>
                            <div class="space-y-1">
                                <Label html_for="new">{ "New password" }</Label>
                                <Input id="new" r#type="password" />
                            </div>
                        </CardContent>
                        <CardFooter>
                            <Button>{ "Save password" }</Button>
                        </CardFooter>
                    </Card>
                </TabsContent>
            </Tabs>
        },
        code: r#"<Tabs default_value="account">
    <TabsList>
        <TabsTrigger value="account">{ "Account" }</TabsTrigger>
        <TabsTrigger value="password">{ "Password" }</TabsTrigger>
    </TabsList>
    <TabsContent value="account">
        { /* Account content */ }
    </TabsContent>
    <TabsContent value="password">
        { /* Password content */ }
    </TabsContent>
</Tabs>"#,
    }];

    let props = vec![
        PropDoc {
            name: "value",
            prop_type: "Option<String>",
            default: "-",
            description: "Controlled active tab",
        },
        PropDoc {
            name: "default_value",
            prop_type: "Option<String>",
            default: "-",
            description: "Default active tab",
        },
        PropDoc {
            name: "on_value_change",
            prop_type: "Callback<String>",
            default: "-",
            description: "Tab change handler",
        },
        PropDoc {
            name: "orientation",
            prop_type: "&str",
            default: "\"horizontal\"",
            description: "Tab orientation",
        },
    ];

    html! { <ComponentPage name="Tabs" description="A set of layered sections of content." {examples} {props} /> }
}
