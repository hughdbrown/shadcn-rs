//! Chart component showcase page

use shadcn_rs::{Chart, ChartData, ChartType};
use yew::prelude::*;

use crate::components::{ComponentPage, Example, PropDoc};

#[function_component(ChartPage)]
pub fn chart_page() -> Html {
    let revenue_data = vec![
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
            value: 209.0,
        },
        ChartData {
            label: "Jun".to_string(),
            value: 214.0,
        },
    ];

    let traffic_share = vec![
        ChartData {
            label: "Chrome".to_string(),
            value: 52.0,
        },
        ChartData {
            label: "Safari".to_string(),
            value: 24.0,
        },
        ChartData {
            label: "Firefox".to_string(),
            value: 12.0,
        },
        ChartData {
            label: "Edge".to_string(),
            value: 8.0,
        },
        ChartData {
            label: "Other".to_string(),
            value: 4.0,
        },
    ];

    let examples = vec![
        Example {
            title: "Cartesian Charts",
            description: "Bar, line, and area charts now render actual SVG data with grid and axis support.",
            demo: html! {
                <div class="space-y-6">
                    <Chart
                        chart_type={ChartType::Bar}
                        data={revenue_data.clone()}
                        width={560}
                        height={280}
                        show_grid={true}
                        show_axis={true}
                    />
                    <Chart
                        chart_type={ChartType::Line}
                        data={revenue_data.clone()}
                        width={560}
                        height={280}
                        show_grid={false}
                        show_axis={true}
                        colors={Some(vec!["#0f766e".into(), "#99f6e4".into()])}
                    />
                    <Chart
                        chart_type={ChartType::Area}
                        data={revenue_data.clone()}
                        width={560}
                        height={280}
                        show_grid={true}
                        show_axis={false}
                    />
                </div>
            },
            code: r##"<Chart
    chart_type={ChartType::Bar}
    data={revenue_data.clone()}
    width={560}
    height={280}
    show_grid={true}
    show_axis={true}
/>

<Chart
    chart_type={ChartType::Line}
    data={revenue_data.clone()}
    colors={Some(vec!["#0f766e".into(), "#99f6e4".into()])}
/>"##,
        },
        Example {
            title: "Pie And Donut",
            description: "Radial chart types use the same data model and legend system.",
            demo: html! {
                <div class="grid gap-6 md:grid-cols-2">
                    <Chart
                        chart_type={ChartType::Pie}
                        data={traffic_share.clone()}
                        width={360}
                        height={280}
                    />
                    <Chart
                        chart_type={ChartType::Donut}
                        data={traffic_share.clone()}
                        width={360}
                        height={280}
                        colors={Some(vec![
                            "#1d4ed8".into(),
                            "#2563eb".into(),
                            "#60a5fa".into(),
                            "#93c5fd".into(),
                            "#bfdbfe".into(),
                        ])}
                    />
                </div>
            },
            code: r##"<Chart
    chart_type={ChartType::Donut}
    data={traffic_share}
    width={360}
    height={280}
/>"##,
        },
    ];

    let props = vec![
        PropDoc {
            name: "chart_type",
            prop_type: "ChartType",
            default: "ChartType::Bar",
            description: "Selects the SVG rendering strategy: bar, line, area, pie, or donut.",
        },
        PropDoc {
            name: "data",
            prop_type: "Vec<ChartData>",
            default: "-",
            description: "The label/value pairs rendered into the chart.",
        },
        PropDoc {
            name: "show_grid",
            prop_type: "bool",
            default: "true",
            description: "Draws cartesian grid lines for bar, line, and area charts.",
        },
        PropDoc {
            name: "show_axis",
            prop_type: "bool",
            default: "true",
            description: "Controls cartesian axis labels and tick marks.",
        },
        PropDoc {
            name: "colors",
            prop_type: "Option<Vec<AttrValue>>",
            default: "None",
            description: "Overrides the default palette used for bars, slices, and line/area accents.",
        },
    ];

    let notes = html! {
        <div class="space-y-3">
            <p>{ "Cartesian charts use shared scaling logic, so toggling grid and axis rendering changes the actual SVG output rather than only CSS." }</p>
            <p>{ "Radial charts use the same legend markup and data structure, which keeps bar/line and pie/donut examples interchangeable." }</p>
        </div>
    };

    html! {
        <ComponentPage
            name="Chart"
            description="SVG charts with real rendering for cartesian and radial layouts."
            {examples}
            {props}
            notes={Some(notes)}
        />
    }
}
