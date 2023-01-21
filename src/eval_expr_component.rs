use std::collections::HashMap;
use evalexpr::*;
use crate::component::{Value, Component};
use crate::execution_process::ExecutionProcess;

// #[derive(serde::Deserialize, serde::Serialize, Debug)]
#[derive(Debug)]
pub struct EvalExprProcess{
    pub eval_expression: String,
}

impl Default for EvalExprProcess {
    fn default() -> Self {
        Self {
            eval_expression: String::new(),
        }
    }
}



impl EvalExprProcess {

    pub fn new(eval_expression: String) -> Self {
        Self {
            eval_expression: eval_expression,
        }
    }
}

impl ExecutionProcess for EvalExprProcess {

    fn simulate(&mut self, input: &HashMap<String, Value>) -> Option<HashMap<String, Value>> {
        
        // Create mutable clone
        let mut eval_string = self.eval_expression.clone();
        
        // Debug
        println!("Evaluating expression:    {:?}", eval_string);

        // Process RHS, replacing each input variable with value
        for (variable, value) in input {

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
                
                output_hash.insert(variable, Value::from(value));

                // if self.output.contains_key(&variable) {
                //     // Convert evalexpr::Value to eas::Value
                //     output_hash.insert(variable, Value::from(value));
                // } else {
                //     println!("Warning: unused output variable {:?}", variable);
                // }
            }

        // evalexpr failed
        }else{
            println!("Failed to run properly!\n\t{:?}", result.unwrap_err());
        }
       
        // Return output has either way
        Some(output_hash)
    }

}
