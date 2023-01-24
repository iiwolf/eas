extern crate eas;

#[cfg(test)]
mod tests{
    use std::{collections::HashMap};
    use eas::{python_process::PythonProcess, component::Value, execution_process::ExecutionProcess};
    
    #[test]
    fn test_simulate(){

        let eval_expression = r#"
def multiply_two(x: float) -> float:
    return x * 2What are

if __name__ == "__main__":
    y = multiply_two(x)
"#;
        let mut process = PythonProcess::new(eval_expression.to_string());
        let input = HashMap::from([('x'.to_string(), Value::Float(2.0))]);
        let output_hash = process.simulate(&input);
        assert_eq!(Some(&Value::Float(2.0)), output_hash.as_ref().unwrap().get("x"));
        assert_eq!(Some(&Value::Float(4.0)), output_hash.as_ref().unwrap().get("y"));

    }

    // #[test]
    // fn test_simulate(){
    //     use eas::component::{Component, Value};
    //     let mut c1 = Component{
    //         name: "square".to_string(),
    //         eval_expression: String::from("y = x ^ 2"), 
    //         required_input: HashMap::from([
    //             ("x".to_string(), Value::Float(3.0))
    //         ]),
    //         required_output: HashMap::from([
    //             ("y".to_string(), Value::Float(0.0)),
    //         ])
    //     };

    //     let input = HashMap::from([("x".to_string(), Value::Float(3.0))]);
    //     let answer = HashMap::from([("y".to_string(), Value::Float(9.0))]);
    //     assert_eq!(c1.simulate(&input), answer);
    //     // assert_eq!(
    //     //     c2.simulate(&HashMap::from([
    //     //         ("x".to_string(), Value::Float(9.0)),
    //     //         ("y".to_string(), Value::Vectorf32(vec![5.0, 2.0])),
    //     //     ])), 
    //     //     HashMap::from([("y", 19.0)])
    //     // );

    // }
}