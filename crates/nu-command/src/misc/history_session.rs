use nu_protocol::ast::Call;
use nu_protocol::engine::{Command, EngineState, Stack};
use nu_protocol::{
    Category, Example, IntoPipelineData, PipelineData, ShellError, Signature, Type, Value,
};

#[derive(Clone)]
pub struct HistorySession;

impl Command for HistorySession {
    fn name(&self) -> &str {
        "history session"
    }

    fn usage(&self) -> &str {
        "Get the command history session"
    }

    fn signature(&self) -> nu_protocol::Signature {
        Signature::build("history session")
            .category(Category::Misc)
            .input_output_types(vec![(Type::Nothing, Type::Int)])
    }

    fn examples(&self) -> Vec<Example> {
        vec![Example {
            example: "history session",
            description: "Get current history session",
            result: None,
        }]
    }

    fn run(
        &self,
        engine_state: &EngineState,
        _stack: &mut Stack,
        call: &Call,
        _input: PipelineData,
    ) -> Result<PipelineData, ShellError> {
        Ok(Value::int(engine_state.history_session_id, call.head).into_pipeline_data())
    }
}
