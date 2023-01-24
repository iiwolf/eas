extern crate eas;

#[cfg(test)]
mod tests{
    use std::{collections::HashMap};
    use cpython::{Python, PyResult, PyDict, PyObject, ToPyObject, PyFloat, ObjectProtocol, PyErr};
    use eas::{python_component::PythonProcess, component::Value};
    
    fn call_python(code: &str, variables: HashMap<String, Value>) -> HashMap<String, Value>{
        let gil = Python::acquire_gil();
        let py = gil.python();
        let locals = PyDict::new(py);
        for (key, value) in variables.iter() {
            match value {
                Value::Float(f) => locals.set_item(py, key, f).unwrap(),
                Value::Vectorf32(v) => locals.set_item(py, key, v).unwrap(),
            }
        }
        
        py.run(code, None, Some(&locals)).unwrap();
    
        let mut output_variables = HashMap::new();
        for (key, result) in locals.items(py).iter() {
            let key: String = key.extract(py).unwrap();
            match result.get_type(py).name(py).as_ref() {
                "float" => output_variables.insert(key, Value::Float(result.extract(py).unwrap())),
                "int" => todo!(),
                "str" => todo!(),
                "list" => output_variables.insert(key, Value::Vectorf32(result.extract(py).unwrap())),
                "dict" => todo!(),
                _ => {
                    println!("Unsupported variable \"{}\" of type \"{}\"", key, result.get_type(py).name(py).as_ref());
                    None
                },
            };
        }
    
        output_variables
    
    }
    
    #[test]
    fn test_simulate(){

        let eval_expression = r#"
def multiply_two(x: float) -> float:
    return x * 2

if __name__ == "__main__":
    y = multiply_two(x)
"#;
        let output_hash = call_python(eval_expression, HashMap::from([('x'.to_string(), Value::Float(2.0))]));
        assert_eq!(Some(&Value::Float(2.0)), output_hash.get("x"));
        assert_eq!(Some(&Value::Float(4.0)), output_hash.get("y"));

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



            // value.name(py);
            // if v.is_ok() {
                // let test = v.unwrap();
                // match result.extract(py).unwrap() {
                //     Ok(Value::Float(_v)) => println!("float"),
                //     Ok(Value::Vectorf32(_v)) => println!("floatvec"),
                //     Err(e) => println!("Error")
                // }
            // }
            // match result.get_type(py) {
            //     cpython::PyType::Float => println!("Float")
            // }
            // match result {
            //     value => {
            //         match value.get_type(py) {
            //             cpython::PyType: => {
            //                 match value.extract::<f32>() {
            //                     Ok(f) => println!("The value is a float: {}", f),
            //                     _ => println!("Failed to extract float"),
            //                 }
            //             },
            //         }
            //     }
            // }
            // match value {
            //     Ok(v) => {
            //         match value.extract::<f32>() {
            //             Ok(f) => println!("The value is a float: {}", f),
            //             _ => println!("The value is not a float")
            //         }
            //     },
            //     Err(error) => {
            //         println!("An error occurred: {:?}", error);
            //     }
            // }
            // match value {
            //     Ok(value) =>

            //     Ok(value.extract::<f32>()) => { output_variables.insert(key, Value::Float(f)); },
            //     Ok(v) => { output_variables.insert(key, Value::Vectorf32(v)); },
            //     _ => {}
            // }