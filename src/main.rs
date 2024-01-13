use nu_plugin::{serve_plugin, EvaluatedCall, JsonSerializer, LabeledError, Plugin};
use nu_protocol::{PluginSignature, Span, Type, Value};

use nu_plugin_makeplot::make_plot;

struct Plot;

impl Plot {
    fn new() -> Self {
        Self
    }
}

enum InputParse {
    Start,
    List,
    Table,
}

fn get_number(v: &Value, span: Option<Span>) -> Result<f32, LabeledError> {
    match v {
        Value::Int { .. } => Ok(v.as_int()? as f32),
        Value::Float { .. } => Ok(v.as_float()? as f32),
        _ => Err(LabeledError {
            label: "Error".into(),
            msg: "Error".into(),
            span,
        }),
    }
}

impl Plugin for Plot {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("makeplot")
            .usage("Creates a plot")
            .input_output_type(Type::List(Box::new(Type::Int)), Type::Binary)
            .input_output_type(Type::List(Box::new(Type::Float)), Type::Binary)]
    }

    fn run(
        &mut self,
        _: &str, // name
        call: &EvaluatedCall,
        input: &Value,
    ) -> Result<Value, LabeledError> {
        // assert_eq!(name, "makeplot");

        let mut input_parse = InputParse::Start;

        // Collect the values from the input
        let values: Result<Vec<(f32, f32)>, LabeledError> = match input {
            Value::List {
                vals,
                internal_span: _,
            } => vals
                .iter()
                .enumerate()
                .map(|(i, v)| match v {
                    Value::Int { .. } | Value::Float { .. } => {
                        match input_parse {
                            InputParse::Start => input_parse = InputParse::List,
                            InputParse::Table => {
                                return Err(LabeledError {
                                    label: "Input contains a mix of numbers and records".into(),
                                    msg: "Input contains a mix of numbers and records".into(),
                                    span: Some(call.head),
                                });
                            }
                            _ => (),
                        }
                        let y = get_number(v, Some(call.head))?;
                        Ok((i as f32, y))
                    }
                    Value::Record { .. } => {
                        match input_parse {
                            InputParse::Start => input_parse = InputParse::Table,
                            InputParse::List => {
                                return Err(LabeledError {
                                    label: "Input contains a mix of numbers and records".into(),
                                    msg: "Input contains a mix of numbers and records".into(),
                                    span: Some(call.head),
                                });
                            }
                            _ => (),
                        }

                        // TODO: take column headers other than x and y
                        let record = v.as_record()?;
                        let x = record.get("x").unwrap();
                        let x = get_number(x, Some(call.head))?;
                        let y = record.get("y").unwrap();
                        let y = get_number(y, Some(call.head))?;

                        Ok((x, y))
                    }
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
            Ok(out) => return Ok(Value::binary(out, call.head)),
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
