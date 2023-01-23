extern crate eas;

#[cfg(test)]
mod tests{
    use std::{collections::HashMap};
    use cpython::{Python, PyResult, PyDict, PyObject, ToPyObject, PyFloat, ObjectProtocol};
    use eas::{python_component::PythonProcess, component::Value};

    #[test]
    fn test_python_call(){
        use cpython::{PyDict, PyResult, Python};
        let gil = Python::acquire_gil();
        let py = gil.python();

        let sys = py.import("sys").unwrap();
        let version: String = sys.get(py, "version").unwrap().extract(py).unwrap();

        let locals = PyDict::new(py);
        locals.set_item(py, "os", py.import("os").unwrap()).unwrap();
        let user: String = py
            .eval(
                "os.getenv('USER') or os.getenv('USERNAME')",
                None,
                Some(&locals),
            ).unwrap()
            .extract(py).unwrap();

        println!("Hello {}, I'm Python {}", user, version);
        // Ok(())
    }
    
    // fn call_python(code: String, variables: HashMap<String, Value>) {
    //     use cpython::{PyDict, PyResult, Python};

    //     // Setup python interpreter
    //     let gil = Python::acquire_gil();
    //     let py = gil.python();

    //     // Assign HashMap to python dict
    //     let locals = PyDict::new(py);

    //     locals.set_item(py, 
    //         "os", py.import("os").unwrap()
    //     ).unwrap();

    //     let user: String = py
    //         .eval(
    //             code.to_string(),
    //             None,
    //             Some(&locals),
    //         ).unwrap()
    //         .extract(py).unwrap();

    // }

    fn call_python_function_simple() -> PyResult<()> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        // Define the function
        py.eval("def my_function(x): return x + 1", None, None)?;

        // Get the function object
        let function = py.eval("my_function", None, None)?;
        // function.call(py, args, kwargs)
        // Call the function
        let args = (1,);
        let result = function.call(py, args, None)?;

        // Print the result
        println!("{:?}", result);
        Ok(())
    }

    // fn extract_function_name(signature: &str) -> Option<&str> {
    //     let re = Regex::new(r"^def\s+(\w+)\s*\(").unwrap();
    //     re.captures(signature).and_then(|captures| captures.get(1)).map(|m| m.as_str())
    // }

    fn extract_function_name(signature: &str) -> Option<&str> {
        let def_index = signature.find("def ")?;
        let open_bracket_index = signature.find("(")?;
        signature.get(def_index+4..open_bracket_index)
    }
    
    fn call_python_function(code: String, input_variables: HashMap<String, Value>) -> PyResult<HashMap<String, Value>> {
        let gil = Python::acquire_gil();
        let py = gil.python();

        // Assign input variables to python dict
        let locals = PyDict::new(py);
        for (key, value) in input_variables.iter() {
            let key = key.to_py_object(py);
            match value {
                Value::Float(f) => locals.set_item(py, key, f.to_py_object(py)).unwrap(),
                Value::Vectorf32(v) => locals.set_item(py, key, v.to_py_object(py)).unwrap(),
            }
        }

        // Define function
        py.eval(code.as_str(), None, None);
            
        // Get function as object
        let function_name = extract_function_name(code.as_str()).unwrap();
        let function = py.eval(function_name, None, None)?;
        // let result = function.call(py, None, Some(&locals))?;
        let result = function.call(py, (3,),  Some(&locals))?;

        // Collect the output variables
        let mut output_variables = HashMap::new();
        for (key, result) in locals.items(py).iter() {
            let key: String = key.extract(py)?;
            match result.get_type(py).name(py).as_ref() {
                "float" => output_variables.insert(key, Value::Float(result.extract(py).unwrap())),
                "int" => todo!(),
                "str" => todo!(),
                "list" => output_variables.insert(key, Value::Vectorf32(result.extract(py).unwrap())),
                "dict" => todo!(),
                _ => todo!(),
            };
        }

        Ok(output_variables)
    }
            
    fn call_python(code: String, variables: HashMap<String, Value>) {
        use cpython::{PyDict, PyResult, Python, PyObject, PyFloat, PyErr};

        // Setup python interpreter
        let gil = Python::acquire_gil();
        let py = gil.python();

        // Assign HashMap to python dict
        let locals = PyDict::new(py);
        for (key, value) in variables.iter() {
            match value {
                Value::Float(f) => locals.set_item(py, key, f).unwrap(),
                Value::Vectorf32(v) => locals.set_item(py, key, v).unwrap(),
            }
        }
        
        // Execute code
        let result: PyResult<PyObject> = py.eval(code.as_str(), None, Some(&locals));
        let output = result.unwrap();
        let extraction: Result<PyFloat, PyErr> = output.extract(py);
        if extraction.is_ok(){
            for (val)in extraction.iter(){
                println!("{:?}", val.value(py));
            }
        }
    }

    #[test]
    fn test_simulate(){

        let eval_expression = "def multiply_two(x):\n\treturn x * 2".to_string();
        let pp = PythonProcess{eval_expression: 
            "def multiply_two(x):
                return x * 2".to_string()
        };
        let inputs = HashMap::from([
            ("x".to_string(), Value::Float(2.0))
        ]);
        let result = call_python_function(eval_expression, inputs);
        println!()
        // assert_eq!(Has, result);

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