use nu_plugin::{serve_plugin, EvaluatedCall, JsonSerializer, LabeledError, Plugin};
use nu_protocol::{PluginExample, PluginSignature, Span, SyntaxShape, Type, Value};

use nu_plugin_makeplot::{make_plot, PlotOptions};

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
            label: "Incorrect type".into(),
            msg: format!("{:?} is not the correct type", v).into(),
            span,
        }),
    }
}

fn parse_call_opts(call: &EvaluatedCall) -> Result<PlotOptions, LabeledError> {
    let mut options = PlotOptions::new();

    let width: Option<i64> = call.get_flag("width")?;
    match width {
        Some(w) => options.width = w as u32,
        None => (),
    }

    let height: Option<i64> = call.get_flag("height")?;
    match height {
        Some(h) => options.height = h as u32,
        None => (),
    }

    let title: Option<String> = call.get_flag("title")?;
    match title {
        Some(t) => options.title = t,
        None => (),
    }

    Ok(options)
}

impl Plugin for Plot {
    fn signature(&self) -> Vec<PluginSignature> {
        vec![PluginSignature::build("makeplot")
            .usage("Creates a plot in png format")
            .input_output_types(vec![
                (Type::List(Box::new(Type::Number)), Type::Binary),
                // (Type::Table(vec![(String::from("x"), Type::Number), (String::from("y"), Type::Number)]), Type::Binary),
                (Type::Table(vec![]), Type::Binary),
            ])
            .named(
                "width",
                SyntaxShape::Int,
                "The width of the plot in pixels.",
                None,
            )
            .named(
                "height",
                SyntaxShape::Int,
                "The height of the plot in pixels.",
                None,
            )
            .named(
                "title",
                SyntaxShape::String,
                "The title of the plot.",
                Some('t'),
            )
            .plugin_examples(vec![
                PluginExample{
                    example: "seq 0 0.1 6.4 | each {|x| {x: $x, y: ($x | math sin)}} | makeplot | save sine.png".into(),
                    description: "Create a plot of a sine wave from a table of values".into(),
                    result: None,
                },
                PluginExample{
                    example: "seq 0 10 | math sqrt | makeplot | save sqrt.png".into(),
                    description: "Create a plot of the square root of a from a list of values".into(),
                    result: None,
                },
            ]),
        ]
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
                .map(|(i, value)| match value {
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
                        let y = get_number(value, Some(call.head))?;
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

                        // Get the x value
                        let record = value.as_record()?;
                        let x = match record.get("x") {
                            Some(v) => v,
                            None => {
                                return Err(LabeledError {
                                    label: "Missing x value".into(),
                                    msg: format!("Missing x value from record {}", i).into(),
                                    span: Some(call.head),
                                });
                            }
                        };
                        let x = get_number(x, Some(call.head))?;

                        // Get the y value
                        let y = match record.get("y") {
                            Some(v) => v,
                            None => {
                                return Err(LabeledError {
                                    label: "Missing y value".into(),
                                    msg: format!("Missing y value from record {}", i).into(),
                                    span: Some(call.head),
                                });
                            }
                        };
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

        let options = parse_call_opts(call)?;
        match make_plot(values, &options) {
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
