use std::collections::HashMap;

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
pub trait Component {
    // Revisit exactly why you can't do this
    // fn new(name: String) -> Self {
    //     Self {
    //         name: name,
    //         eval_expression: String::new(),
    //         input: HashMap::new(),
    //         output: HashMap::new(),
    //     }
    // }

    // The fact that I write this here then in each child annoys me...
    //  I understand the point but I still feel like there has to be 
    //  a way to circumvent this boilerplate in Rust
    fn get_mut_eval_expression(&mut self) -> String;
    fn get_name(&self) -> &String;
    fn get_input(&self) -> &HashMap<String, Value>;
    fn get_output(&self) -> &HashMap<String, Value>;
    fn set_input(&mut self, key: &String, value: Value);
    fn get_input_clone(&self) -> HashMap<String, Value>;
    fn get_output_clone(&self) -> HashMap<String, Value>;

    fn contains_input(&mut self, key: &String) -> bool{
        self.get_input().contains_key(key)
    }
    fn simulate(&mut self, input: &HashMap<String, Value>) -> Option<HashMap<String, Value>> {
        None
    }

}
