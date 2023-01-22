use std::collections::HashMap;
use crate::component::Value;

pub trait ExecutionProcess {
    fn simulate(&mut self, input: &HashMap<String, Value>) -> Option<HashMap<String, Value>>;
}
