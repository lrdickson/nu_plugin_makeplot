use nu_plugin::{serve_plugin, EvaluatedCall, JsonSerializer, LabeledError, Plugin};
use nu_protocol::{PluginSignature, Type, Value};
use plotters::drawing::DrawingAreaErrorKind;
use plotters::prelude::*;

use std::error::Error;

struct Plot;

trait ResultExt<T> {
    fn to_labeled_err(self, call: &EvaluatedCall) -> Result<T, LabeledError>;
}

impl<T, U: Error + Send + Sync> ResultExt<T> for Result<T, DrawingAreaErrorKind<U>> {
    fn to_labeled_err(self, call: &EvaluatedCall) -> Result<T, LabeledError> {
        match self {
            Ok(x) => Ok(x),
            Err(e) => Err(LabeledError {
                label: "Drawing Area Error".into(),
                msg: format!("{}", e).into(),
                span: Some(call.head),
            }),
        }
    }
}

impl Plot {
    fn new() -> Self {
        Self
    }

    fn make_plot(&mut self, call: &EvaluatedCall, input: &Value) -> Result<(), LabeledError> {
        let values: Result<Vec<(f32, f32)>, LabeledError> = match input {
            Value::List {
                vals,
                internal_span: _,
            } => vals
                .iter()
                .enumerate()
                .map(|(i, v)| match v {
                    Value::Int { .. } => Ok((i as f32, v.as_int()? as f32)),
                    Value::Float { .. } => Ok((i as f32, v.as_float()? as f32)),
                    _ => Err(LabeledError {
                        label: "Incorrect input type".into(),
                        msg: "Incorrect input type".into(),
                        span: Some(call.head),
                    }),
                })
                .collect(),
            _ => {
                return Err(LabeledError {
                    label: "Incorrect input type".into(),
                    msg: "Incorrect input type".into(),
                    span: Some(call.head),
                })
            }
        };
        let values = values?;

        let root = BitMapBackend::new("test.png", (640, 480)).into_drawing_area();
        root.fill(&WHITE).to_labeled_err(call)?;
        let mut chart = ChartBuilder::on(&root)
            .margin(5)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(-1f32..1f32, -0.1f32..1f32)
            .to_labeled_err(call)?;

        chart.configure_mesh().draw().to_labeled_err(call)?;

        chart
            .draw_series(LineSeries::new(values.into_iter(), &RED))
            .to_labeled_err(call)?;

        root.present().to_labeled_err(call)?;

        Ok(())
    }
}

impl Plugin for Plot {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("plotgen").usage("creates a plot"), // .input_output_type(Type::List(Type::Int), Type::Nothing)
                                                                       // .input_output_type(Type::Table(), Type::Nothing)
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        assert_eq!(name, "plotgen");
        self.make_plot(call, input)?;
        Ok(Value::nothing(call.head))
    }
}

fn main() {
    serve_plugin(&mut Plot::new(), JsonSerializer)
}
