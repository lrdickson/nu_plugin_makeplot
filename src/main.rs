use nu_plugin::{serve_plugin, EvaluatedCall, JsonSerializer, LabeledError, Plugin};
use nu_protocol::{PluginSignature, Type, Value};

use nu_plugin_makeplot::make_plot;

struct Plot;

impl Plot {
    fn new() -> Self {
        Self
    }
}

impl Plugin for Plot {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![
            PluginSignature::build("makeplot").usage("creates a plot"), // .input_output_type(Type::List(Type::Int), Type::Nothing)
                                                                        // .input_output_type(Type::Table(), Type::Nothing)
        ]
    }

    fn run(
        &mut self,
        name: &str,
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        // assert_eq!(name, "makeplot");

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

        match make_plot(values) {
            Ok(val) => Ok(Value::binary(val, call.head)),
            Err(e) => Err(LabeledError {
                msg: format!("{}", e),
                label: e.label,
                span: Some(call.head),
            }),
        }
    }
}

fn main() {
    serve_plugin(&mut Plot::new(), JsonSerializer)
}
