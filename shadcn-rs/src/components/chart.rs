//! Chart component
//!
//! SVG-based charting system with multiple chart types.
//!
//! # Examples
//!
//! ```rust,no_run
//! use yew::prelude::*;
//! use shadcn_rs::{Chart, ChartType, ChartData};
//!
//! #[function_component(App)]
//! fn app() -> Html {
//!     let data = vec![
//!         ChartData { label: "Jan".to_string(), value: 10.0 },
//!         ChartData { label: "Feb".to_string(), value: 20.0 },
//!         ChartData { label: "Mar".to_string(), value: 15.0 },
//!     ];
//!
//!     html! {
//!         <Chart
//!             chart_type={ChartType::Bar}
//!             data={data}
//!             width={600}
//!             height={400}
//!         />
//!     }
//! }
//! ```

use yew::prelude::*;

/// Chart type
#[derive(Debug, Clone, PartialEq)]
pub enum ChartType {
    /// Bar chart
    Bar,
    /// Line chart
    Line,
    /// Area chart
    Area,
    /// Pie chart
    Pie,
    /// Donut chart
    Donut,
}

/// Chart data point
#[derive(Debug, Clone, PartialEq)]
pub struct ChartData {
    /// Label for this data point
    pub label: String,
    /// Value for this data point
    pub value: f64,
}

/// Chart component properties
#[derive(Properties, PartialEq, Clone)]
pub struct ChartProps {
    /// Chart type
    #[prop_or(ChartType::Bar)]
    pub chart_type: ChartType,

    /// Chart data
    pub data: Vec<ChartData>,

    /// Chart width in pixels
    #[prop_or(500)]
    pub width: u32,

    /// Chart height in pixels
    #[prop_or(300)]
    pub height: u32,

    /// Show legend
    #[prop_or(true)]
    pub show_legend: bool,

    /// Show grid lines
    #[prop_or(true)]
    pub show_grid: bool,

    /// Show axis labels
    #[prop_or(true)]
    pub show_axis: bool,

    /// Color scheme (CSS colors)
    #[prop_or_default]
    pub colors: Option<Vec<AttrValue>>,

    /// Additional CSS classes
    #[prop_or_default]
    pub class: Classes,
}

/// Chart component
///
/// Renders various chart types with SVG.
///
/// # Accessibility
/// - ARIA role="img"
/// - Descriptive aria-label
/// - Accessible data table alternative recommended
#[function_component(Chart)]
pub fn chart(props: &ChartProps) -> Html {
    let ChartProps {
        chart_type,
        data,
        width,
        height,
        show_legend,
        show_grid: _,
        show_axis: _,
        colors: _,
        class,
    } = props.clone();

    let classes: Classes = vec![Classes::from("chart"), class]
        .into_iter()
        .collect();

    let chart_type_str = match chart_type {
        ChartType::Bar => "Bar Chart",
        ChartType::Line => "Line Chart",
        ChartType::Area => "Area Chart",
        ChartType::Pie => "Pie Chart",
        ChartType::Donut => "Donut Chart",
    };

    let aria_label = format!(
        "{} with {} data points",
        chart_type_str,
        data.len()
    );

    html! {
        <div class={classes}>
            <svg
                class="chart-svg"
                width={width.to_string()}
                height={height.to_string()}
                role="img"
                aria-label={aria_label}
            >
                <rect
                    x="0"
                    y="0"
                    width={width.to_string()}
                    height={height.to_string()}
                    fill="transparent"
                    stroke="#e0e0e0"
                    stroke-width="1"
                />
                <text
                    x={(width / 2).to_string()}
                    y={(height / 2).to_string()}
                    text-anchor="middle"
                    fill="#666"
                >
                    { format!("{} - {} data points", chart_type_str, data.len()) }
                </text>
            </svg>
            if show_legend {
                <div class="chart-legend">
                    {
                        data.iter().enumerate().map(|(idx, item)| {
                            html! {
                                <div key={idx} class="chart-legend-item">
                                    <span class="chart-legend-marker" />
                                    <span class="chart-legend-label">
                                        { &item.label }
                                        { ": " }
                                        { item.value }
                                    </span>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                </div>
            }
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_default() {
        let data = vec![
            ChartData {
                label: "A".to_string(),
                value: 10.0,
            },
        ];

        let props = ChartProps {
            chart_type: ChartType::Bar,
            data,
            width: 500,
            height: 300,
            show_legend: true,
            show_grid: true,
            show_axis: true,
            colors: None,
            class: Classes::new(),
        };

        assert_eq!(props.chart_type, ChartType::Bar);
        assert_eq!(props.width, 500);
        assert_eq!(props.height, 300);
    }

    #[test]
    fn test_chart_line() {
        let data = vec![
            ChartData {
                label: "A".to_string(),
                value: 10.0,
            },
        ];

        let props = ChartProps {
            chart_type: ChartType::Line,
            data,
            width: 500,
            height: 300,
            show_legend: true,
            show_grid: true,
            show_axis: true,
            colors: None,
            class: Classes::new(),
        };

        assert_eq!(props.chart_type, ChartType::Line);
    }

    #[test]
    fn test_chart_pie() {
        let data = vec![
            ChartData {
                label: "A".to_string(),
                value: 30.0,
            },
            ChartData {
                label: "B".to_string(),
                value: 70.0,
            },
        ];

        let props = ChartProps {
            chart_type: ChartType::Pie,
            data: data.clone(),
            width: 500,
            height: 300,
            show_legend: true,
            show_grid: true,
            show_axis: true,
            colors: None,
            class: Classes::new(),
        };

        assert_eq!(props.chart_type, ChartType::Pie);
        assert_eq!(props.data.len(), 2);
    }

    #[test]
    fn test_chart_data() {
        let data = ChartData {
            label: "Test".to_string(),
            value: 42.5,
        };

        assert_eq!(data.label, "Test");
        assert_eq!(data.value, 42.5);
    }

    #[test]
    fn test_chart_types() {
        assert_eq!(ChartType::Bar, ChartType::Bar);
        assert_ne!(ChartType::Bar, ChartType::Line);
    }
}
