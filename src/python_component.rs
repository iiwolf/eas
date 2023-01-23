use std::collections::HashMap;
use evalexpr::*;
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

    fn new(eval_expression: String) -> Self {
        Self {
            eval_expression: eval_expression,
        }
    }
}

impl ExecutionProcess for PythonProcess {
    
    fn simulate(&mut self, input: &HashMap<String, Value>) -> Option<HashMap<String, Value>> {
        None
        // // Create mutable clone
        // let mut eval_string = self.eval_expression.clone();
        
        // // Debug
        // println!("Evaluating expression:    {:?}", eval_string);
        
        // // Open Python instance
        // let gil = Python::acquire_gil();
        // let py = gil.python();
        // let locals = PyDict::new(py);
        
        // // Process RHS, replacing each input variable with value
        // let locals = PyDict::new(py);
        // for (key, value) in input.iter() {
        //     match value {
        //         Value::Float(f) => locals.set_item(py, key, f).unwrap(),
        //         Value::Vectorf32(v) => locals.set_item(py, key, v).unwrap(),
        //     }
        // }
        
        // // Debug
        // println!("                          {:?}", eval_string);

        // // Create HashMapContext to get multiple assignments
        // let mut context = HashMapContext::new();

        // // Run code
        // // let result = py.eval(&self.eval_expression, None, Some(&locals))?.extract(py)?;
        // let output: PyResult<PyObject> = py.eval(&self.eval_expression, None, Some(&locals));.unwrap(); //.extract(py).unwrap();

        // let result = match output.is_ok() {
        //     True => output.unwrap().extract(py),
        //     False => {
        //         println!("Failed to run properly!\n\t{:?}", output.unwrap_err());
        //         return None;
        //     },
        // };

        // if output.is_err() {
        //     println!("Failed to run properly!\n\t{:?}", output.unwrap_err());
        //     return None;
        // }

        // let result = output.unwrap().extract(py);
        
        // // Create new hashmap to store variables
        // let mut output_hash: HashMap<String, Value> = HashMap::new();
        
        // // evalexpr ran properly
        // if result.is_ok() {
            
        //     // For now, assume output is always dict
        //     let output_dict: PyDict = result.unwrap();

        //     // Assign variables from calculation
        //     for (variable, value) in output_dict.items(py) {

        //         // if self.output.contains_key(&variable.to_string()) {
        //             let float: f32 = value.cast_into::<PyFloat>(py).unwrap().value(py) as f32;
        //             output_hash.insert(variable.to_string(), Value::Float(float));

        //         // } else {
        //         //     println!("Warning: unused output variable {:?}", variable);
        //         // }
                
        //     }

        // // evalexpr failed
        // }else{
        //     // println!("Failed to run properly!\n\t{:?}", result.unwrap_err());
        //     println!("Failed to run properly and I don't know how to output the python errors so rip");
        // }

        // // Save as most recent output
        // // self.output = output_hash.clone();
        
        // // Return output has either way
        // Some(output_hash)
    }

}
