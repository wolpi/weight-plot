use crate::model::WeightLine;
use crate::model::PlotType;

use chrono::{Duration, NaiveDate, NaiveDateTime, Datelike};
use plotters::prelude::*;
use std::f32;

pub fn plot(
        data :&Vec<&WeightLine>,
        path :&str,
        title :&str,
        plot_type :PlotType) -> Result<(), Box<dyn std::error::Error>> {
    let res = match plot_type {
        PlotType::FULL => (4000, 900),
        PlotType::YEAR => (2000, 800),
        PlotType::MONTH => (800, 600),
    };
    let backend = BitMapBackend::new(path, res);
    let root = backend.into_drawing_area();
    root.fill(&WHITE)?;

    let start_date = &data[0].timestamp.date();
    let end_timestamp = &data[data.len() - 1].timestamp;
    let (from_date, to_date) = (
        to_duration(&data[0].timestamp, start_date),
        to_duration(&data[data.len() - 1].timestamp, start_date),
    );

    let mut val_min = f32::MAX;
    let mut val_max = f32::MIN;
    let mut val_sum = 0.0;
    for line in data {
        let val = line.weight;
        if val < val_min {val_min = val;}
        if val > val_max {val_max = val;}
        val_sum += val;
    }
    let val_avg = val_sum / data.len() as f32;
    let y_min = val_min - 5.0;
    let y_max = val_max + 5.0;

    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(50)
        .caption(title, ("sans-serif", 30.0).into_font())
        .build_cartesian_2d(from_date..to_date, y_min..y_max)?;

    let mut mesh = chart.configure_mesh();
    mesh.light_line_style(&WHITE)
        .x_label_formatter(&|x| format!("{}", format_duration_as_date(x, start_date)))
        .y_label_formatter(&|y| format!("{}", y))
        .x_labels(20)
        .draw()?;

    // draw dots, collect data for lines and trend
    let mut line_dots = Vec::new();
    let mut last_x = zero_duration();
    let mut trend :Vec<(Duration, f32)> = Vec::new();
    let mut trend_tmp = Vec::new();
    let trend_abstraction = 3;
    trend.push((from_date, data[0].weight));
    chart.draw_series(
        data.iter()
            .map(|x| {
                let y = x.weight;
                let x = to_duration(&x.timestamp, start_date);
                if last_x != x || last_x == zero_duration() {
                    line_dots.push((x, y));
                    last_x = x;

                    // calc trend
                    if trend_tmp.len() < trend_abstraction {
                        trend_tmp.push(y);
                    } else {
                        trend_value(&mut trend, &mut trend_tmp, x, y);
                    }
                }
                Circle::new((x, y), 2, BLACK.filled())
            }),
    )?;
    // draw lines
    chart.draw_series(LineSeries::new(line_dots, &BLACK))?;

    // last trend value
    if data.len() > trend_abstraction {
        if trend[trend.len() -1].0 != to_date {
            trend_tmp.clear();
            for i in trend_abstraction..1 {
                trend_tmp.push(data[data.len() - i].weight);
            }
            let x = to_date;
            let y = data[data.len() - 1].weight;
            trend_value(&mut trend, &mut trend_tmp, x, y);
        }
    }
    // draw trend
    chart.draw_series(trend.iter().map(|(x,y)| {
        Circle::new((*x, *y), 5, BLUE.filled())
    }))?;
    let trend_style :ShapeStyle = ShapeStyle {
        color: BLUE.to_rgba(),
        filled: true,
        stroke_width: 3,
    };
    chart.draw_series(LineSeries::new(trend, trend_style))?;

    // draw min, max, avg
    if data.len() > 2 {
        chart.plotting_area().draw(&Rectangle::new([
                (zero_duration(), val_min),
                (max_width_duration(end_timestamp, start_date), val_min)
            ],
            GREEN.filled()))?;
        chart.plotting_area().draw(&Rectangle::new([
                (zero_duration(), val_max),
                (max_width_duration(end_timestamp, start_date), val_max)
            ],
            RED.filled()))?;
        chart.plotting_area().draw(&Rectangle::new([
                (zero_duration(), val_avg),
                (max_width_duration(end_timestamp, start_date), val_avg)
            ],
            YELLOW.filled()))?;

        let font_min: FontDesc = ("sans-serif", 14).into_font();
        let font_max: FontDesc = ("sans-serif", 14).into_font();
        let font_avg: FontDesc = ("sans-serif", 14).into_font();
        let text_x = to_duration(&data[1].timestamp, start_date);
        chart.plotting_area().draw(&Text::new(((val_min*10.0).round()/10.0).to_string(), (text_x, (val_min - 0.2)), font_min))?;
        chart.plotting_area().draw(&Text::new(((val_max*10.0).round()/10.0).to_string(), (text_x, (val_max + 0.4)), font_max))?;
        chart.plotting_area().draw(&Text::new(((val_avg*10.0).round()/10.0).to_string(), (text_x, (val_avg - 0.2)), font_avg))?;
    }

    Ok(())
}

fn to_duration(timestamp :&NaiveDateTime, start_date :&NaiveDate) -> Duration {
    timestamp.date().signed_duration_since(*start_date)
}

fn zero_duration() -> Duration {
    Duration::seconds(0)
}

fn max_width_duration(end_timestamp :&NaiveDateTime, start_date :&NaiveDate) -> Duration {
    to_duration(end_timestamp, start_date)
}

fn format_duration_as_date(duration :&Duration, start_date :&NaiveDate) -> String {
    let date = start_date.checked_add_signed(*duration).unwrap();
    [date.day().to_string(), ".".to_string(), date.month().to_string(), ".".to_string()].concat()
}

fn trend_value(trend: &mut Vec<(Duration, f32)>, trend_tmp: &mut Vec<f32>, x: Duration, y: f32) {
    let sum: f32 = trend_tmp.iter().sum::<f32>() + y;
    let avg = sum / (trend_tmp.len() as f32 + 1.0);
    trend_tmp.clear();
    trend.push((x, avg));
}
