//! Command component showcase page

use yew::prelude::*;
use shadcn_rs::{Command, CommandInput, CommandList, CommandEmpty, CommandGroup, CommandItem, CommandSeparator};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(CommandPage)]
pub fn command_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A command palette interface.",
            demo: html! {
                <Command class="rounded-lg border shadow-md">
                    <CommandInput placeholder="Type a command or search..." />
                    <CommandList>
                        <CommandEmpty>{ "No results found." }</CommandEmpty>
                        <CommandGroup heading="Suggestions">
                            <CommandItem>{ "Calendar" }</CommandItem>
                            <CommandItem>{ "Search Emoji" }</CommandItem>
                            <CommandItem>{ "Calculator" }</CommandItem>
                        </CommandGroup>
                        <CommandSeparator />
                        <CommandGroup heading="Settings">
                            <CommandItem>{ "Profile" }</CommandItem>
                            <CommandItem>{ "Billing" }</CommandItem>
                            <CommandItem>{ "Settings" }</CommandItem>
                        </CommandGroup>
                    </CommandList>
                </Command>
            },
            code: r#"<Command>
    <CommandInput placeholder="Type a command..." />
    <CommandList>
        <CommandEmpty>{ "No results found." }</CommandEmpty>
        <CommandGroup heading="Suggestions">
            <CommandItem>{ "Calendar" }</CommandItem>
            <CommandItem>{ "Search" }</CommandItem>
        </CommandGroup>
    </CommandList>
</Command>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Command content" },
    ];

    html! { <ComponentPage name="Command" description="Fast, composable command menu." {examples} {props} /> }
}
