# Component Composition Patterns

This guide covers how to compose shadcn-rs components together to build complex UIs.

## Basic Composition

shadcn-rs components follow a compound component pattern where a parent component wraps related child components:

```rust
use yew::prelude::*;
use shadcn_rs::{Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter};
use shadcn_rs::Button;

#[function_component(UserCard)]
fn user_card() -> Html {
    html! {
        <Card>
            <CardHeader>
                <CardTitle>{ "User Profile" }</CardTitle>
                <CardDescription>{ "Manage your account settings" }</CardDescription>
            </CardHeader>
            <CardContent>
                <p>{ "Content goes here" }</p>
            </CardContent>
            <CardFooter>
                <Button>{ "Save" }</Button>
            </CardFooter>
        </Card>
    }
}
```

## Nesting Components

Components can be freely nested. For example, placing form elements inside a dialog:

```rust
use yew::prelude::*;
use shadcn_rs::{Dialog, DialogTrigger, DialogContent, DialogHeader, DialogTitle};
use shadcn_rs::{Input, Label, Button};

#[function_component(EditDialog)]
fn edit_dialog() -> Html {
    html! {
        <Dialog>
            <DialogTrigger>
                <Button>{ "Edit" }</Button>
            </DialogTrigger>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>{ "Edit Item" }</DialogTitle>
                </DialogHeader>
                <Label for_id="name">{ "Name" }</Label>
                <Input id="name" placeholder="Enter name" />
                <Button>{ "Save Changes" }</Button>
            </DialogContent>
        </Dialog>
    }
}
```

## Passing Props Through

Use the `class` prop to customize any component's styling:

```rust
use yew::prelude::*;
use shadcn_rs::{Button, Variant, Size};

html! {
    <Button
        variant={Variant::Destructive}
        size={Size::Sm}
        class={classes!("my-custom-class")}
    >
        { "Delete" }
    </Button>
};
```

## Event Handling

Components expose callbacks for user interactions:

```rust
use yew::prelude::*;
use shadcn_rs::{Button, Switch, Checkbox};

#[function_component(InteractiveExample)]
fn interactive_example() -> Html {
    let checked = use_state(|| false);
    let on_toggle = {
        let checked = checked.clone();
        Callback::from(move |val: bool| checked.set(val))
    };

    html! {
        <>
            <Switch checked={*checked} onchange={on_toggle} />
            <Button onclick={Callback::from(|_| log::info!("Clicked!"))}>
                { "Click me" }
            </Button>
        </>
    }
}
```

## Controlled vs Uncontrolled

Many components support both controlled (value passed via props) and uncontrolled (internal state) patterns:

```rust
use yew::prelude::*;
use shadcn_rs::Input;

#[function_component(ControlledInput)]
fn controlled_input() -> Html {
    let value = use_state(|| String::from("initial"));

    let oninput = {
        let value = value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            value.set(input.value());
        })
    };

    html! {
        <Input value={(*value).clone()} oninput={oninput} />
    }
}
```

## Compound Component Groups

Several components use context to share state between parent and children:

- **Select/SelectTrigger/SelectContent/SelectItem** - Selection state shared via context
- **Accordion/AccordionItem** - Open/close state managed by parent
- **Tabs/TabsList/TabsTrigger/TabsContent** - Active tab shared via context
- **Command/CommandInput/CommandList/CommandItem** - Search query shared via context

```rust
use yew::prelude::*;
use shadcn_rs::{Tabs, TabsList, TabsTrigger, TabsContent};

#[function_component(TabsExample)]
fn tabs_example() -> Html {
    html! {
        <Tabs default_value="tab1">
            <TabsList>
                <TabsTrigger value="tab1">{ "Account" }</TabsTrigger>
                <TabsTrigger value="tab2">{ "Settings" }</TabsTrigger>
            </TabsList>
            <TabsContent value="tab1">
                <p>{ "Account settings here" }</p>
            </TabsContent>
            <TabsContent value="tab2">
                <p>{ "General settings here" }</p>
            </TabsContent>
        </Tabs>
    }
}
```

## Data Table Composition

`DataTable` is column-driven. Define stable column metadata once, then pass row data separately:

```rust
use shadcn_rs::{DataTable, DataTableColumn, SortDirection};

#[derive(Clone, PartialEq)]
struct Invoice {
    status: &'static str,
    email: &'static str,
    amount: u32,
}

let columns = vec![
    DataTableColumn::text(
        "status",
        "Status",
        Callback::from(|invoice: Invoice| AttrValue::from(invoice.status)),
    ),
    DataTableColumn::text(
        "email",
        "Email",
        Callback::from(|invoice: Invoice| AttrValue::from(invoice.email)),
    ),
    DataTableColumn {
        id: "amount".into(),
        header: "Amount".into(),
        accessor: Callback::from(|invoice: Invoice| AttrValue::from(invoice.amount.to_string())),
        cell: Some(Callback::from(|invoice: Invoice| html! { <strong>{ invoice.amount }</strong> })),
        sortable: true,
        searchable: false,
        class: classes!("text-right"),
    },
];

html! {
    <DataTable<Invoice>
        columns={columns}
        data={invoices}
        sortable={true}
        filterable={true}
        paginated={true}
        rows_per_page={10}
        default_sort_column={Some(AttrValue::from("email"))}
        default_sort_direction={SortDirection::Ascending}
    />
}
```

## Calendar And Date Picker Composition

`DatePicker` is now built on top of `Calendar`, so the same selection rules and date bounds flow through both components:

```rust
use shadcn_rs::{Calendar, CalendarMode, DatePicker};

html! {
    <>
        <Calendar
            mode={CalendarMode::Range}
            selected={Some(AttrValue::from("2026-04-17..2026-04-22"))}
        />

        <DatePicker
            value={Some(AttrValue::from("2026-04-17"))}
            min_date={Some(AttrValue::from("2026-04-10"))}
            max_date={Some(AttrValue::from("2026-04-30"))}
            format="DD/MM/YYYY"
        />
    </>
}
```

## Custom Wrappers

Create reusable component wrappers by accepting `Children`:

```rust
use yew::prelude::*;
use shadcn_rs::{Card, CardHeader, CardTitle, CardContent};

#[derive(Properties, PartialEq)]
pub struct SectionProps {
    pub title: AttrValue,
    pub children: Children,
}

#[function_component(Section)]
fn section(props: &SectionProps) -> Html {
    html! {
        <Card>
            <CardHeader>
                <CardTitle>{ &props.title }</CardTitle>
            </CardHeader>
            <CardContent>
                { props.children.clone() }
            </CardContent>
        </Card>
    }
}
```
