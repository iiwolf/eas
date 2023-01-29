use std::collections::HashMap;
use egui::{Response, Ui};

use crate::component::Value;

pub trait ExecutionProcess {
    fn simulate(&mut self, input: &HashMap<String, Value>) -> Option<HashMap<String, Value>>;
    fn get_eval_expression(&mut self) -> &String;
    fn set_eval_expression(&mut self, eval_expression: String);
    // fn display_execution_process(&mut self, ui: &mut Ui);
    fn display_widget(&mut self, ui: &mut Ui) -> Response;

}
