//! DataTable component showcase page

use yew::prelude::*;
use shadcn_rs::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell, Checkbox};

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(DataTablePage)]
pub fn data_table_page() -> Html {
    let examples = vec![
        Example {
            title: "Default",
            description: "A data table with sorting and selection.",
            demo: html! {
                <div class="rounded-md border">
                    <Table>
                        <TableHeader>
                            <TableRow>
                                <TableHead class="w-[50px]">
                                    <Checkbox />
                                </TableHead>
                                <TableHead>{ "Status" }</TableHead>
                                <TableHead>{ "Email" }</TableHead>
                                <TableHead class="text-right">{ "Amount" }</TableHead>
                            </TableRow>
                        </TableHeader>
                        <TableBody>
                            <TableRow>
                                <TableCell><Checkbox /></TableCell>
                                <TableCell>{ "Success" }</TableCell>
                                <TableCell>{ "ken99@example.com" }</TableCell>
                                <TableCell class="text-right">{ "$316.00" }</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Checkbox /></TableCell>
                                <TableCell>{ "Processing" }</TableCell>
                                <TableCell>{ "abe45@example.com" }</TableCell>
                                <TableCell class="text-right">{ "$242.00" }</TableCell>
                            </TableRow>
                            <TableRow>
                                <TableCell><Checkbox /></TableCell>
                                <TableCell>{ "Failed" }</TableCell>
                                <TableCell>{ "monserrat44@example.com" }</TableCell>
                                <TableCell class="text-right">{ "$837.00" }</TableCell>
                            </TableRow>
                        </TableBody>
                    </Table>
                </div>
            },
            code: r#"<DataTable
    columns={columns}
    data={data}
    sortable={true}
    selectable={true}
/>"#,
        },
    ];

    let props = vec![
        PropDoc { name: "columns", prop_type: "Vec<Column>", default: "-", description: "Column definitions" },
        PropDoc { name: "data", prop_type: "Vec<T>", default: "-", description: "Data rows" },
        PropDoc { name: "sortable", prop_type: "bool", default: "false", description: "Enable sorting" },
        PropDoc { name: "selectable", prop_type: "bool", default: "false", description: "Enable row selection" },
        PropDoc { name: "pagination", prop_type: "bool", default: "false", description: "Enable pagination" },
        PropDoc { name: "page_size", prop_type: "usize", default: "10", description: "Rows per page" },
    ];

    html! { <ComponentPage name="Data Table" description="Powerful table with sorting, filtering, and pagination." {examples} {props} /> }
}
