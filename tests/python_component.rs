extern crate eas;

#[cfg(test)]
mod tests{
    use std::{collections::HashMap};

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