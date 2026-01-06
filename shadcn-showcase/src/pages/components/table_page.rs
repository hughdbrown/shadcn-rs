//! Table component showcase page

use yew::prelude::*;
use shadcn_rs::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(TablePage)]
pub fn table_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A basic data table.",
            demo: html! {
                <Table>
                    <TableHeader>
                        <TableRow>
                            <TableHead>{ "Invoice" }</TableHead>
                            <TableHead>{ "Status" }</TableHead>
                            <TableHead>{ "Method" }</TableHead>
                            <TableHead class="text-right">{ "Amount" }</TableHead>
                        </TableRow>
                    </TableHeader>
                    <TableBody>
                        <TableRow>
                            <TableCell class="font-medium">{ "INV001" }</TableCell>
                            <TableCell>{ "Paid" }</TableCell>
                            <TableCell>{ "Credit Card" }</TableCell>
                            <TableCell class="text-right">{ "$250.00" }</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell class="font-medium">{ "INV002" }</TableCell>
                            <TableCell>{ "Pending" }</TableCell>
                            <TableCell>{ "PayPal" }</TableCell>
                            <TableCell class="text-right">{ "$150.00" }</TableCell>
                        </TableRow>
                        <TableRow>
                            <TableCell class="font-medium">{ "INV003" }</TableCell>
                            <TableCell>{ "Unpaid" }</TableCell>
                            <TableCell>{ "Bank Transfer" }</TableCell>
                            <TableCell class="text-right">{ "$350.00" }</TableCell>
                        </TableRow>
                    </TableBody>
                </Table>
            },
            code: r#"<Table>
    <TableHeader>
        <TableRow>
            <TableHead>{ "Invoice" }</TableHead>
            <TableHead>{ "Status" }</TableHead>
            <TableHead>{ "Amount" }</TableHead>
        </TableRow>
    </TableHeader>
    <TableBody>
        <TableRow>
            <TableCell>{ "INV001" }</TableCell>
            <TableCell>{ "Paid" }</TableCell>
            <TableCell>{ "$250.00" }</TableCell>
        </TableRow>
    </TableBody>
</Table>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "class", prop_type: "Classes", default: "-", description: "Additional CSS classes" },
        PropDoc { name: "children", prop_type: "Children", default: "-", description: "Table content" },
    ];

    html! { <ComponentPage name="Table" description="A responsive table component." {examples} {props} /> }
}
