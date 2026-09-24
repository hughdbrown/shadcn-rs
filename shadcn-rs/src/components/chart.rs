//! Chart component
//!
//! SVG-based charting system with multiple chart types.

use std::f64::consts::{PI, TAU};
use yew::prelude::*;

const DEFAULT_COLORS: [&str; 6] = [
    "hsl(var(--color-primary))",
    "hsl(var(--color-chart-2, 217 91% 60%))",
    "hsl(var(--color-chart-3, 142 71% 45%))",
    "hsl(var(--color-chart-4, 38 92% 50%))",
    "hsl(var(--color-chart-5, 262 83% 58%))",
    "hsl(var(--color-chart-6, 348 83% 47%))",
];

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

#[derive(Clone, Copy)]
struct ChartBounds {
    width: f64,
    height: f64,
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
}

impl ChartBounds {
    fn inner_width(self) -> f64 {
        (self.width - self.left - self.right).max(1.0)
    }

    fn inner_height(self) -> f64 {
        (self.height - self.top - self.bottom).max(1.0)
    }
}

fn palette(colors: Option<&Vec<AttrValue>>) -> Vec<String> {
    if let Some(colors) = colors
        && !colors.is_empty()
    {
        return colors.iter().map(ToString::to_string).collect();
    }
    DEFAULT_COLORS
        .iter()
        .map(|color| (*color).to_string())
        .collect()
}

fn axis_bounds(width: u32, height: u32, show_axis: bool) -> ChartBounds {
    ChartBounds {
        width: f64::from(width),
        height: f64::from(height),
        top: 16.0,
        right: 16.0,
        bottom: if show_axis { 48.0 } else { 24.0 },
        left: if show_axis { 48.0 } else { 24.0 },
    }
}

fn max_value(data: &[ChartData]) -> f64 {
    let value = data.iter().map(|item| item.value).fold(0.0_f64, f64::max);
    if value > 0.0 { value } else { 1.0 }
}

fn tick_values(max_value: f64) -> Vec<f64> {
    (0..=4)
        .map(|step| max_value * (step as f64) / 4.0)
        .collect()
}

fn value_y(bounds: ChartBounds, value: f64, max_value: f64) -> f64 {
    let ratio = value / max_value;
    // `clamp` passes NaN through, which would put "NaN" into SVG coordinates.
    let ratio = if ratio.is_finite() {
        ratio.clamp(0.0, 1.0)
    } else {
        0.0
    };
    bounds.top + bounds.inner_height() - (ratio * bounds.inner_height())
}

fn chart_type_label(chart_type: &ChartType) -> &'static str {
    match chart_type {
        ChartType::Bar => "Bar Chart",
        ChartType::Line => "Line Chart",
        ChartType::Area => "Area Chart",
        ChartType::Pie => "Pie Chart",
        ChartType::Donut => "Donut Chart",
    }
}

fn polar_to_cartesian(cx: f64, cy: f64, radius: f64, angle: f64) -> (f64, f64) {
    (cx + radius * angle.cos(), cy + radius * angle.sin())
}

/// SVG cannot draw an arc whose start and end points coincide, so a single
/// 100% slice would vanish; stop just short of a full turn instead.
fn visible_end_angle(start_angle: f64, end_angle: f64) -> f64 {
    end_angle.min(start_angle + TAU - 1e-4)
}

fn arc_path(cx: f64, cy: f64, radius: f64, start_angle: f64, end_angle: f64) -> String {
    let end_angle = visible_end_angle(start_angle, end_angle);
    let (start_x, start_y) = polar_to_cartesian(cx, cy, radius, start_angle);
    let (end_x, end_y) = polar_to_cartesian(cx, cy, radius, end_angle);
    let large_arc = if end_angle - start_angle > PI { 1 } else { 0 };
    format!(
        "M {cx:.2} {cy:.2} L {start_x:.2} {start_y:.2} A {radius:.2} {radius:.2} 0 {large_arc} 1 {end_x:.2} {end_y:.2} Z"
    )
}

fn donut_segment_path(
    cx: f64,
    cy: f64,
    outer_radius: f64,
    inner_radius: f64,
    start_angle: f64,
    end_angle: f64,
) -> String {
    let end_angle = visible_end_angle(start_angle, end_angle);
    let (outer_start_x, outer_start_y) = polar_to_cartesian(cx, cy, outer_radius, start_angle);
    let (outer_end_x, outer_end_y) = polar_to_cartesian(cx, cy, outer_radius, end_angle);
    let (inner_end_x, inner_end_y) = polar_to_cartesian(cx, cy, inner_radius, end_angle);
    let (inner_start_x, inner_start_y) = polar_to_cartesian(cx, cy, inner_radius, start_angle);
    let large_arc = if end_angle - start_angle > PI { 1 } else { 0 };
    format!(
        "M {outer_start_x:.2} {outer_start_y:.2} \
         A {outer_radius:.2} {outer_radius:.2} 0 {large_arc} 1 {outer_end_x:.2} {outer_end_y:.2} \
         L {inner_end_x:.2} {inner_end_y:.2} \
         A {inner_radius:.2} {inner_radius:.2} 0 {large_arc} 0 {inner_start_x:.2} {inner_start_y:.2} Z"
    )
}

fn render_cartesian_chart(
    chart_type: &ChartType,
    data: &[ChartData],
    width: u32,
    height: u32,
    show_grid: bool,
    show_axis: bool,
    colors: &[String],
) -> Html {
    let bounds = axis_bounds(width, height, show_axis);
    let max_data_value = max_value(data);
    let steps = data.len().max(1);
    let slot_width = bounds.inner_width() / steps as f64;
    let bar_width = (slot_width * 0.65).max(8.0);
    let baseline = bounds.top + bounds.inner_height();
    let stroke_color = colors
        .first()
        .cloned()
        .unwrap_or_else(|| DEFAULT_COLORS[0].to_string());
    let fill_color = colors
        .get(1)
        .cloned()
        .unwrap_or_else(|| "rgb(59 130 246 / 0.18)".to_string());

    let points: Vec<(f64, f64)> = data
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let x = bounds.left + slot_width * index as f64 + slot_width / 2.0;
            let y = value_y(bounds, item.value, max_data_value);
            (x, y)
        })
        .collect();

    let line_path = points
        .iter()
        .enumerate()
        .map(|(index, (x, y))| {
            if index == 0 {
                format!("M {x:.2} {y:.2}")
            } else {
                format!(" L {x:.2} {y:.2}")
            }
        })
        .collect::<String>();

    let area_path = if let (Some((first_x, _)), Some((last_x, _))) = (points.first(), points.last())
    {
        format!("{line_path} L {last_x:.2} {baseline:.2} L {first_x:.2} {baseline:.2} Z")
    } else {
        String::new()
    };

    html! {
        <>
            if show_grid {
                {
                    tick_values(max_data_value).iter().enumerate().map(|(index, tick)| {
                        let y = value_y(bounds, *tick, max_data_value);
                        html! {
                            <line
                                key={format!("grid-{index}")}
                                x1={bounds.left.to_string()}
                                y1={y.to_string()}
                                x2={(bounds.left + bounds.inner_width()).to_string()}
                                y2={y.to_string()}
                                stroke="hsl(var(--color-border))"
                                stroke-dasharray="4 4"
                            />
                        }
                    }).collect::<Html>()
                }
            }

            if show_axis {
                <line
                    x1={bounds.left.to_string()}
                    y1={baseline.to_string()}
                    x2={(bounds.left + bounds.inner_width()).to_string()}
                    y2={baseline.to_string()}
                    stroke="hsl(var(--color-border))"
                />
                <line
                    x1={bounds.left.to_string()}
                    y1={bounds.top.to_string()}
                    x2={bounds.left.to_string()}
                    y2={baseline.to_string()}
                    stroke="hsl(var(--color-border))"
                />
                {
                    tick_values(max_data_value).iter().enumerate().map(|(index, tick)| {
                        let y = value_y(bounds, *tick, max_data_value);
                        html! {
                            <text
                                key={format!("tick-{index}")}
                                x={(bounds.left - 8.0).to_string()}
                                y={(y + 4.0).to_string()}
                                text-anchor="end"
                                font-size="12"
                                fill="hsl(var(--color-muted-foreground))"
                            >
                                { format!("{tick:.0}") }
                            </text>
                        }
                    }).collect::<Html>()
                }
                {
                    data.iter().enumerate().map(|(index, item)| {
                        let x = bounds.left + slot_width * index as f64 + slot_width / 2.0;
                        html! {
                            <text
                                key={format!("label-{index}")}
                                x={x.to_string()}
                                y={(baseline + 22.0).to_string()}
                                text-anchor="middle"
                                font-size="12"
                                fill="hsl(var(--color-muted-foreground))"
                            >
                                { &item.label }
                            </text>
                        }
                    }).collect::<Html>()
                }
            }

            {
                match chart_type {
                    ChartType::Bar => html! {
                        <>
                            {
                                data.iter().enumerate().map(|(index, item)| {
                                    let x = bounds.left + slot_width * index as f64 + (slot_width - bar_width) / 2.0;
                                    let y = value_y(bounds, item.value, max_data_value);
                                    let bar_height = (baseline - y).max(0.0);
                                    let color = colors[index % colors.len()].clone();
                                    html! {
                                        <rect
                                            key={format!("bar-{index}")}
                                            x={x.to_string()}
                                            y={y.to_string()}
                                            width={bar_width.to_string()}
                                            height={bar_height.to_string()}
                                            rx="6"
                                            fill={color}
                                        />
                                    }
                                }).collect::<Html>()
                            }
                        </>
                    },
                    ChartType::Line => html! {
                        <>
                            <path
                                d={line_path.clone()}
                                fill="none"
                                stroke={stroke_color.clone()}
                                stroke-width="3"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                            {
                                points.iter().enumerate().map(|(index, (x, y))| {
                                    html! {
                                        <circle
                                            key={format!("point-{index}")}
                                            cx={x.to_string()}
                                            cy={y.to_string()}
                                            r="4"
                                            fill="hsl(var(--color-background))"
                                            stroke={stroke_color.clone()}
                                            stroke-width="2"
                                        />
                                    }
                                }).collect::<Html>()
                            }
                        </>
                    },
                    ChartType::Area => html! {
                        <>
                            <path d={area_path} fill={fill_color} />
                            <path
                                d={line_path}
                                fill="none"
                                stroke={stroke_color}
                                stroke-width="3"
                                stroke-linecap="round"
                                stroke-linejoin="round"
                            />
                        </>
                    },
                    ChartType::Pie | ChartType::Donut => Html::default(),
                }
            }
        </>
    }
}

fn render_radial_chart(
    chart_type: &ChartType,
    data: &[ChartData],
    width: u32,
    height: u32,
    colors: &[String],
) -> Html {
    let positive_total = data.iter().map(|item| item.value.max(0.0)).sum::<f64>();
    // Only guard against zero: fractional totals (e.g. 0.2 + 0.3) must still fill the circle.
    let total = if positive_total > 0.0 {
        positive_total
    } else {
        1.0
    };
    let center_x = f64::from(width) / 2.0;
    let center_y = f64::from(height) / 2.0;
    let radius = f64::from(width.min(height)) * 0.32;
    let inner_radius = radius * 0.58;
    let mut current_angle = -PI / 2.0;

    html! {
        <>
            {
                data.iter().enumerate().map(|(index, item)| {
                    let slice = item.value.max(0.0) / total;
                    let sweep = slice * PI * 2.0;
                    let start_angle = current_angle;
                    let end_angle = current_angle + sweep;
                    current_angle = end_angle;
                    let color = colors[index % colors.len()].clone();
                    let path = match chart_type {
                        ChartType::Pie => arc_path(center_x, center_y, radius, start_angle, end_angle),
                        ChartType::Donut => donut_segment_path(
                            center_x,
                            center_y,
                            radius,
                            inner_radius,
                            start_angle,
                            end_angle,
                        ),
                        _ => String::new(),
                    };
                    html! {
                        <path
                            key={format!("slice-{index}")}
                            d={path}
                            fill={color}
                            stroke="hsl(var(--color-background))"
                            stroke-width="2"
                        />
                    }
                }).collect::<Html>()
            }
            if matches!(chart_type, ChartType::Donut) {
                <text
                    x={center_x.to_string()}
                    y={(center_y - 4.0).to_string()}
                    text-anchor="middle"
                    font-size="14"
                    font-weight="600"
                    fill="hsl(var(--color-foreground))"
                >
                    { format!("{positive_total:.0}") }
                </text>
                <text
                    x={center_x.to_string()}
                    y={(center_y + 14.0).to_string()}
                    text-anchor="middle"
                    font-size="12"
                    fill="hsl(var(--color-muted-foreground))"
                >
                    { "Total" }
                </text>
            }
        </>
    }
}

/// Chart component
#[function_component(Chart)]
pub fn chart(props: &ChartProps) -> Html {
    let ChartProps {
        chart_type,
        data,
        width,
        height,
        show_legend,
        show_grid,
        show_axis,
        colors,
        class,
    } = props.clone();

    let classes: Classes = vec![Classes::from("chart"), class].into_iter().collect();
    let chart_type_str = chart_type_label(&chart_type);
    let palette = palette(colors.as_ref());
    let aria_label = format!("{} with {} data points", chart_type_str, data.len());
    let is_radial = matches!(chart_type, ChartType::Pie | ChartType::Donut);

    html! {
        <div class={classes}>
            <svg
                class="chart-svg"
                width={width.to_string()}
                height={height.to_string()}
                viewBox={format!("0 0 {width} {height}")}
                role="img"
                aria-label={aria_label}
            >
                if data.is_empty() {
                    <text
                        x={(width / 2).to_string()}
                        y={(height / 2).to_string()}
                        text-anchor="middle"
                        fill="hsl(var(--color-muted-foreground))"
                    >
                        { "No data available" }
                    </text>
                } else if is_radial {
                    { render_radial_chart(&chart_type, &data, width, height, &palette) }
                } else {
                    { render_cartesian_chart(&chart_type, &data, width, height, show_grid, show_axis, &palette) }
                }
            </svg>
            if show_legend {
                <div class="chart-legend">
                    {
                        data.iter().enumerate().map(|(index, item)| {
                            let marker_style = format!(
                                "background-color: {};",
                                palette[index % palette.len()]
                            );
                            html! {
                                <div key={index} class="chart-legend-item">
                                    <span class="chart-legend-marker" style={marker_style} />
                                    <span class="chart-legend-label">
                                        { format!("{}: {}", item.label, item.value) }
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
        let data = vec![ChartData {
            label: "A".to_string(),
            value: 10.0,
        }];

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
    fn test_default_palette() {
        assert_eq!(palette(None).len(), DEFAULT_COLORS.len());
    }

    #[test]
    fn test_tick_values() {
        assert_eq!(tick_values(100.0), vec![0.0, 25.0, 50.0, 75.0, 100.0]);
    }

    #[test]
    fn test_arc_path_commands() {
        let path = arc_path(100.0, 100.0, 40.0, -PI / 2.0, 0.0);
        assert!(path.starts_with("M 100.00 100.00"));
        assert!(path.contains("A 40.00 40.00"));
    }

    #[test]
    fn test_donut_path_commands() {
        let path = donut_segment_path(100.0, 100.0, 40.0, 24.0, -PI / 2.0, PI / 2.0);
        assert!(path.contains("A 40.00 40.00"));
        assert!(path.contains("A 24.00 24.00"));
    }
}
