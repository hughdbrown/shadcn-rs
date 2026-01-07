//! Chart component showcase page

use shadcn_rs::{Chart, ChartData, ChartType};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ChartPage)]
pub fn chart_page() -> Html {
    let bar_data = vec![
        ChartData {
            label: "Jan".to_string(),
            value: 186.0,
        },
        ChartData {
            label: "Feb".to_string(),
            value: 305.0,
        },
        ChartData {
            label: "Mar".to_string(),
            value: 237.0,
        },
        ChartData {
            label: "Apr".to_string(),
            value: 73.0,
        },
        ChartData {
            label: "May".to_string(),
            value: 209.0,
        },
        ChartData {
            label: "Jun".to_string(),
            value: 214.0,
        },
    ];

    let line_data = vec![
        ChartData {
            label: "Jan".to_string(),
            value: 186.0,
        },
        ChartData {
            label: "Feb".to_string(),
            value: 305.0,
        },
        ChartData {
            label: "Mar".to_string(),
            value: 237.0,
        },
        ChartData {
            label: "Apr".to_string(),
            value: 273.0,
        },
        ChartData {
            label: "May".to_string(),
            value: 309.0,
        },
        ChartData {
            label: "Jun".to_string(),
            value: 314.0,
        },
    ];

    let pie_data = vec![
        ChartData {
            label: "Chrome".to_string(),
            value: 275.0,
        },
        ChartData {
            label: "Safari".to_string(),
            value: 200.0,
        },
        ChartData {
            label: "Firefox".to_string(),
            value: 187.0,
        },
        ChartData {
            label: "Edge".to_string(),
            value: 173.0,
        },
        ChartData {
            label: "Other".to_string(),
            value: 90.0,
        },
    ];

    let examples = vec![
        Example {
            title: "Bar Chart",
            description: "A simple bar chart.",
            demo: html! {
                <Chart
                    chart_type={ChartType::Bar}
                    data={bar_data.clone()}
                    width={500}
                    height={300}
                />
            },
            code: r##"<Chart
    chart_type={ChartType::Bar}
    data={data}
    width={500}
    height={300}
/>"##,
        },
        Example {
            title: "Line Chart",
            description: "A line chart for trends.",
            demo: html! {
                <Chart
                    chart_type={ChartType::Line}
                    data={line_data.clone()}
                    width={500}
                    height={300}
                />
            },
            code: r##"<Chart
    chart_type={ChartType::Line}
    data={data}
    width={500}
    height={300}
/>"##,
        },
        Example {
            title: "Pie Chart",
            description: "A pie chart for proportions.",
            demo: html! {
                <Chart
                    chart_type={ChartType::Pie}
                    data={pie_data.clone()}
                    width={400}
                    height={300}
                />
            },
            code: r##"<Chart
    chart_type={ChartType::Pie}
    data={data}
    width={400}
    height={300}
/>"##,
        },
        Example {
            title: "Area Chart",
            description: "An area chart showing trends.",
            demo: html! {
                <Chart
                    chart_type={ChartType::Area}
                    data={line_data.clone()}
                    width={500}
                    height={300}
                />
            },
            code: r##"<Chart
    chart_type={ChartType::Area}
    data={data}
    width={500}
    height={300}
/>"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "chart_type",
            prop_type: "ChartType",
            default: "Bar",
            description: "Type of chart (Bar, Line, Area, Pie, Donut)",
        },
        PropDoc {
            name: "data",
            prop_type: "Vec<ChartData>",
            default: "-",
            description: "Chart data points",
        },
        PropDoc {
            name: "width",
            prop_type: "u32",
            default: "500",
            description: "Chart width in pixels",
        },
        PropDoc {
            name: "height",
            prop_type: "u32",
            default: "300",
            description: "Chart height in pixels",
        },
        PropDoc {
            name: "show_legend",
            prop_type: "bool",
            default: "true",
            description: "Show legend",
        },
        PropDoc {
            name: "show_grid",
            prop_type: "bool",
            default: "true",
            description: "Show grid lines",
        },
        PropDoc {
            name: "show_axis",
            prop_type: "bool",
            default: "true",
            description: "Show axis labels",
        },
        PropDoc {
            name: "colors",
            prop_type: "Option<Vec<AttrValue>>",
            default: "-",
            description: "Custom color scheme",
        },
    ];

    html! { <ComponentPage name="Chart" description="Beautiful, responsive charts built with SVG." {examples} {props} /> }
}
