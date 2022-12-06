use std::collections::HashMap;
use evalexpr::*;

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

// #[derive(serde::Deserialize, serde::Serialize, Debug)]
#[derive(Debug)]
pub struct Component{
    pub name: String,
    pub eval_expression: String,
    pub required_input: HashMap<String, Value>,
    pub required_output: HashMap<String, Value>,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            name: "Empty".to_string(),
            eval_expression: String::new(),
            required_input: HashMap::new(),
            required_output: HashMap::new(),
        }
    }
}

impl Component{

    pub fn new(name: String) -> Self {
        Self {
            name: name,
            eval_expression: String::new(),
            required_input: HashMap::new(),
            required_output: HashMap::new(),
        }
    }

    pub fn simulate(&self, input: &HashMap<String, Value>) -> HashMap<String, Value> {
        
        // Create mutable clone
        let mut eval_string = self.eval_expression.clone();
        
        // Debug
        println!("Evaluating expression:    {:?}", eval_string);

        // Process RHS, replacing each input variable with value
        for (variable, value) in &self.required_input {

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
                return HashMap::new()
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
                
                if self.required_output.contains_key(&variable) {
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

        // Return output has either way
        output_hash
    }

}
