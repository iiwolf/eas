use std::collections::HashMap;
use egui::{Ui, Widget, Response};
use cpython::{Python, PyDict, PyFloat, PyResult, PyObject};
use crate::{component::Value, execution_process::ExecutionProcess};

// #[derive(serde::Deserialize, serde::Serialize, Debug)]
#[derive(Debug)]
pub struct PythonProcess {
    pub eval_expression: String,
}

impl Default for PythonProcess {
    fn default() -> Self {
        Self {
            eval_expression: String::new(),
        }
    }
}

impl PythonProcess {

    pub fn new(eval_expression: String) -> Self {
        Self {
            eval_expression: eval_expression,
        }
    }
}

impl ExecutionProcess for PythonProcess {
    
    fn get_eval_expression(&mut self) -> &String { &self.eval_expression }
    fn set_eval_expression(&mut self, eval_expression: String) { self.eval_expression = eval_expression; }

    fn display_widget(&mut self, ui: &mut Ui) -> Response {
        ui.add(
            egui::TextEdit::multiline(&mut self.eval_expression)
                .font(egui::TextStyle::Monospace) // for cursor height
                .code_editor()
                .desired_rows(23)
                .lock_focus(true)
                // .desired_width(self.size.x * 0.5)
        )
    }

    fn simulate(&mut self, input: &HashMap<String, Value>) -> Option<HashMap<String, Value>> {
        
        // Debug
        println!("Evaluating expression:    {:?}", &self.eval_expression);
        
        // Open Python instance
        let gil = Python::acquire_gil();
        let py = gil.python();
        let locals = PyDict::new(py);
        
        // Add variables to local scope
        for (key, value) in input.iter() {
            match value {
                Value::Float(f) => locals.set_item(py, key, f).unwrap(),
                Value::Vectorf32(v) => locals.set_item(py, key, v).unwrap(),
            }
        }
        
        // Run code
        let result = py.run(&self.eval_expression, None, Some(&locals));

        if result.is_err() {
            println!("Failed to run properly!\n\t{:?}", result.unwrap_err());
            return None;
        }

        // Create new hashmap to store output variables
        let mut output_hash: HashMap<String, Value> = HashMap::new();
        for (key, result) in locals.items(py).iter() {
            let key: String = key.extract(py).unwrap();

            // Inputs SHOULD NOT be added to output has despite being available locals
            if input.contains_key(&key) { continue };
            
            // Insert to output hash based on types
            match result.get_type(py).name(py).as_ref() {
                "float" => output_hash.insert(key, Value::Float(result.extract(py).unwrap())),
                "int" => todo!(),
                "str" => todo!(),
                "list" => output_hash.insert(key, Value::Vectorf32(result.extract(py).unwrap())),
                "dict" => todo!(),
                _ => {
                    println!("Unsupported variable \"{}\" of type \"{}\"", key, result.get_type(py).name(py).as_ref());
                    None
                },
            };
        }

        // Return output has either way
        Some(output_hash)
    }

}
