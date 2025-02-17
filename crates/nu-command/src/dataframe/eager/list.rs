use nu_protocol::{
    ast::Call,
    engine::{Command, EngineState, Stack},
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Value,
};

use crate::dataframe::values::NuDataFrame;

#[derive(Clone)]
pub struct ListDF;

impl Command for ListDF {
    fn name(&self) -> &str {
        "dfr ls"
    }

    fn usage(&self) -> &str {
        "Lists stored dataframes"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name()).category(Category::Custom("dataframe".into()))
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            description: "Creates a new dataframe and shows it in the dataframe list",
            example: r#"let test = ([[a b];[1 2] [3 4]] | dfr into-df);
    ls"#,
            result: None,
        }]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        let mut vals: Vec<(String, Value)> = vec![];

        for overlay_frame in engine_state.active_overlays(&[]) {
            for var in &overlay_frame.vars {
                if let Ok(value) = stack.get_var(*var.1, call.head) {
                    let name = String::from_utf8_lossy(var.0).to_string();
                    vals.push((name, value));
                }
            }
        }

        let vals = vals
            .into_iter()
            .filter_map(|(name, value)| {
                NuDataFrame::try_from_value(value).ok().map(|df| (name, df))
            })
            .map(|(name, df)| {
                let name = Value::String {
                    val: name,
                    span: call.head,
                };

                let columns = Value::int(df.as_ref().width() as i64, call.head);

                let rows = Value::int(df.as_ref().height() as i64, call.head);

                let cols = vec![
                    "name".to_string(),
                    "columns".to_string(),
                    "rows".to_string(),
                ];
                let vals = vec![name, columns, rows];

                Value::Record {
                    cols,
                    vals,
                    span: call.head,
                }
            })
            .collect::<Vec<Value>>();

        let list = Value::List {
            vals,
            span: call.head,
        };

        Ok(list.into_pipeline_data())
    }
}
