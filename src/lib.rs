use anyhow::{self};
use plotters::drawing::DrawingAreaErrorKind;
use plotters::prelude::*;
use std::error::Error;
use std::fmt;

pub struct MakePlotError {
    pub label: String,
    source: anyhow::Error,
}

impl fmt::Display for MakePlotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.source)
    }
}

impl fmt::Debug for MakePlotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}:{}", file!(), line!(), self.source)
    }
}

trait ResultToMakePlotError<T, S: Into<String>> {
    fn to_makeplot_err(self, label: S) -> Result<T, MakePlotError>;
}

impl<T, S: Into<String>, U: Error + Send + Sync> ResultToMakePlotError<T, S>
    for Result<T, DrawingAreaErrorKind<U>>
where
    U: 'static,
{
    fn to_makeplot_err(self, label: S) -> Result<T, MakePlotError> {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(MakePlotError {
                label: label.into(),
                source: e.into(),
            }),
        }
    }
}

pub fn make_plot(values: Vec<(f32, f32)>) -> Result<(), MakePlotError> {
    // Find the min and max values
    let mut min_x = f32::MAX;
    let mut max_x = f32::MIN;
    let mut min_y = f32::MAX;
    let mut max_y = f32::MIN;
    for (x, y) in values.iter() {
        if *x < min_x {
            min_x = *x;
        }
        if *x > max_x {
            max_x = *x;
        }
        if *y < min_y {
            min_y = *y;
        }
        if *y > max_y {
            max_y = *y;
        }
    }

    let domain = max_x - min_x;
    let domain_buffer = domain / 10.;
    let range = max_y - min_y;
    let range_buffer = range / 10.;

    let plot_min_x = min_x - domain_buffer;
    let plot_max_x = max_x + domain_buffer;
    let plot_min_y = min_y - range_buffer;
    let plot_max_y = max_y + range_buffer;

    let root = BitMapBackend::new("test.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).to_makeplot_err("Error")?;
    let mut chart = ChartBuilder::on(&root)
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        // .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
        .build_cartesian_2d(plot_min_x..plot_max_x, plot_min_y..plot_max_y)
        // .build_cartesian_2d(min_x..max_x, min_y..max_y)
        .to_makeplot_err("Error")?;

    chart.configure_mesh().draw().to_makeplot_err("Error")?;

    chart
        .draw_series(LineSeries::new(values.into_iter(), &RED))
        .to_makeplot_err("Error")?;

    root.present().to_makeplot_err("Error")?;

    Ok(())
}
