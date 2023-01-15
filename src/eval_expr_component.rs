use std::collections::HashMap;
use evalexpr::*;
use crate::component::{Value, Component};

// #[derive(serde::Deserialize, serde::Serialize, Debug)]
#[derive(Debug)]
pub struct EvalExprComponent{
    pub name: String,
    pub eval_expression: String,
    pub input: HashMap<String, Value>,
    pub output: HashMap<String, Value>,
}

impl Default for EvalExprComponent {
    fn default() -> Self {
        Self {
            name: "EvalExprComponent".to_string(),
            eval_expression: String::new(),
            input: HashMap::new(),
            output: HashMap::new(),
        }
    }
}



impl EvalExprComponent {

    fn new(name: String) -> Self {
        Self {
            name: name,
            eval_expression: String::new(),
            input: HashMap::new(),
            output: HashMap::new(),
        }
    }
}

impl Component for EvalExprComponent {

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
        for (variable, value) in &self.input {

            if input.contains_key(variable) {
                eval_string = match value {
                    Value::Float(val) => eval_string.replace(variable, &format!("{:?}", val)),
                    Value::Vectorf32(vec32) => {
                        for (i, value) in vec32.iter().enumerate() {
                            eval_string = eval_string.replace(
                                &format!("{}{:?}", variable, i), 
                                &format!("{:?}", value)
                            );
                        }
                        eval_string
                    }
                }
            } else {
                println!("Invalid input: missing input {:?}", variable);
                return None
            }
            
        }

        // Debug
        println!("                          {:?}", eval_string);

        // Create HashMapContext to get multiple assignments
        let mut context = HashMapContext::new();
        
        // Run code
        let result = eval_with_context_mut(&eval_string, &mut context);

        // Create new hashmap to store variables
        let mut output_hash = HashMap::new();
        
        // evalexpr ran properly
        if result.is_ok() {

            // Assign variables from calculation
            for (variable, value) in context.iter_variables() {
                
                if self.output.contains_key(&variable) {
                    // Convert evalexpr::Value to eas::Value
                    output_hash.insert(variable, Value::from(value));
                } else {
                    println!("Warning: unused output variable {:?}", variable);
                }
            }

        // evalexpr failed
        }else{
            println!("Failed to run properly!\n\t{:?}", result.unwrap_err());
        }

        // Save as most recent output
        self.output = output_hash.clone();
        
        // Return output has either way
        Some(output_hash)
    }

}
