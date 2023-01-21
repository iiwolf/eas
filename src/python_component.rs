use std::collections::HashMap;
use evalexpr::*;
use cpython::{Python, PyDict, PyFloat};
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
        
        // Create mutable clone
        let mut eval_string = self.eval_expression.clone();
        
        // Debug
        println!("Evaluating expression:    {:?}", eval_string);

        // Process RHS, replacing each input variable with value
        // for (variable, value) in &self.input {

        //     if input.contains_key(variable) {
        //         eval_string = match value {
        //             Value::Float(val) => eval_string.replace(variable, &format!("{:?}", val)),
        //             Value::Vectorf32(vec32) => {
        //                 for (i, value) in vec32.iter().enumerate() {
        //                     eval_string = eval_string.replace(
        //                         &format!("{}{:?}", variable, i), 
        //                         &format!("{:?}", value)
        //                     );
        //                 }
        //                 eval_string
        //             }
        //         }
        //     } else {
        //         println!("Invalid input: missing input {:?}", variable);
        //         return None
        //     }
            
        // }

        // Debug
        println!("                          {:?}", eval_string);

        // Create HashMapContext to get multiple assignments
        let mut context = HashMapContext::new();
        
        // Open Python instance
        let gil = Python::acquire_gil();
        let py = gil.python();
        let locals = PyDict::new(py);
        
        for (variable, value) in input {
            match value {
                Value::Float(val) => locals.set_item(py, variable, val),
                Value::Vectorf32(vec32) => locals.set_item(py, variable, vec32)
            };
        }

        // Run code
        // let result = py.eval(&self.eval_expression, None, Some(&locals))?.extract(py)?;
        let output = py.eval("os.getenv('USER') or os.getenv('USERNAME')", None, Some(&locals)); //.unwrap().extract(py).unwrap();

        // let result = match output.is_ok() {
        //     True => output.unwrap().extract(py),
        //     False => {
        //         println!("Failed to run properly!\n\t{:?}", output.unwrap_err());
        //         return None;
        //     },
        // };

        if output.is_err() {
            println!("Failed to run properly!\n\t{:?}", output.unwrap_err());
            return None;
        }

        let result = output.unwrap().extract(py);
        
        // Create new hashmap to store variables
        let mut output_hash: HashMap<String, Value> = HashMap::new();
        
        // evalexpr ran properly
        if result.is_ok() {
            
            // For now, assume output is always dict
            let output_dict: PyDict = result.unwrap();

            // Assign variables from calculation
            for (variable, value) in output_dict.items(py) {

                // if self.output.contains_key(&variable.to_string()) {
                    let float: f32 = value.cast_into::<PyFloat>(py).unwrap().value(py) as f32;
                    output_hash.insert(variable.to_string(), Value::Float(float));

                // } else {
                //     println!("Warning: unused output variable {:?}", variable);
                // }
                
            }

        // evalexpr failed
        }else{
            // println!("Failed to run properly!\n\t{:?}", result.unwrap_err());
            println!("Failed to run properly and I don't know how to output the python errors so rip");
        }

        // Save as most recent output
        // self.output = output_hash.clone();
        
        // Return output has either way
        Some(output_hash)
    }

}
