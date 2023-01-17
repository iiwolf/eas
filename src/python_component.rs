use std::collections::HashMap;
use evalexpr::*;
use cpython::{Python, PyDict, PyFloat};
use crate::component::{Value, Component};

// #[derive(serde::Deserialize, serde::Serialize, Debug)]
#[derive(Debug)]
pub struct PythonComponent {
    pub name: String,
    pub eval_expression: String,
    pub input: HashMap<String, Value>,
    pub output: HashMap<String, Value>,
}

impl Default for PythonComponent {
    fn default() -> Self {
        Self {
            name: "PythonComponent".to_string(),
            eval_expression: String::new(),
            input: HashMap::new(),
            output: HashMap::new(),
        }
    }
}

impl PythonComponent {

    fn new(name: String) -> Self {
        Self {
            name: name,
            eval_expression: String::new(),
            input: HashMap::new(),
            output: HashMap::new(),
        }
    }
}

impl Component for PythonComponent {
    
    fn get_name(&self) -> &String { &self.name }
    fn get_mut_eval_expression(&mut self) -> String { self.eval_expression }
    fn get_input(&self) -> &HashMap<String, Value>{ &self.input }
    fn get_output(&self) -> &HashMap<String, Value>{ &self.output }
    fn set_input(&mut self, key: &String, value: Value) {
        *self.input.get_mut(key).unwrap() = value;
    }    

    fn get_input_clone(&self) -> HashMap<String, Value>{ self.input.clone() }
    fn get_output_clone(&self) -> HashMap<String, Value>{ self.output.clone() }

    fn simulate(&mut self, input: &HashMap<String, Value>) -> Option<HashMap<String, Value>> {
        
        // Save as most recent input
        self.input = input.clone();

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
        
        for (variable, value) in &self.input {
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

                if self.output.contains_key(&variable.to_string()) {
                    let float: f32 = value.cast_into::<PyFloat>(py).unwrap().value(py) as f32;
                    output_hash.insert(variable.to_string(), Value::Float(float));

                } else {
                    println!("Warning: unused output variable {:?}", variable);
                }
                
            }

        // evalexpr failed
        }else{
            // println!("Failed to run properly!\n\t{:?}", result.unwrap_err());
            println!("Failed to run properly and I don't know how to output the python errors so rip");
        }

        // Save as most recent output
        self.output = output_hash.clone();
        
        // Return output has either way
        Some(output_hash)
    }

}
