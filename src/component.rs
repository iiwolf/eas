use std::collections::HashMap;

use crate::execution_process::{ExecutionProcess, self};

// #[derive(serde::Deserialize, serde::Serialize, Debug)]
#[derive(Debug, Clone, PartialEq)]
pub enum Value{
    // Integer(i32),
    Float(f32),
    // String(&'static str),
    // Vectori32(Vec<f32>),
    Vectorf32(Vec<f32>),
    // VectorString(Vec<f32>),
}

impl From<evalexpr::Value> for Value {
    fn from(value: evalexpr::Value) -> Self {
        Value::Float(value.as_float().unwrap() as f32)
    }
}

pub struct Component {
    pub name: String,
    pub execution_process: Box<dyn ExecutionProcess>,
    pub input: HashMap<String, Value>,
    pub output: HashMap<String, Value>,
}

impl Component {
    // Revisit exactly why you can't do this
    pub fn new(name: String, execution_process: Box<dyn ExecutionProcess>) -> Self {
        Self {
            name: name,
            execution_process: execution_process,
            input: HashMap::new(),
            output: HashMap::new(),
        }
    }

    pub fn simulate(&mut self, input: &HashMap<String, Value>) -> Option<HashMap<String, Value>> {
        self.execution_process.simulate(&input)
    }

}
