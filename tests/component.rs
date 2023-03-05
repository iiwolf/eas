extern crate eas;

#[cfg(test)]
mod tests{
    use std::{collections::HashMap};
    use eas::component::{Component, Value};

    #[test]
    fn test_simulate_eval_expr(){
        use eas::eval_expr_process::EvalExprProcess;
        let mut c1 = Component{
            name: "square".to_string(),
            execution_process: Box::new(EvalExprProcess::new("y = x ^ 2".to_string())), 
            input: HashMap::from([
                ("x".to_string(), Value::Float(3.0))
            ]),
            output: HashMap::from([
                ("y".to_string(), Value::Float(0.0)),
            ])
        };

        let input = HashMap::from([("x".to_string(), Value::Float(3.0))]);
        let answer = HashMap::from([("y".to_string(), Value::Float(9.0))]);
        assert_eq!(c1.simulate(&input).unwrap(), answer);

    }

    #[test]
    fn test_simulate_python(){
        use eas::python_process::PythonProcess;
        let mut c1 = Component{
            name: "square".to_string(),
            execution_process: Box::new(PythonProcess::new("y = x ** 2".to_string())), 
            input: HashMap::from([
                ("x".to_string(), Value::Float(3.0))
            ]),
            output: HashMap::from([
                ("y".to_string(), Value::Float(0.0)),
            ])
        };

        let input = HashMap::from([("x".to_string(), Value::Float(3.0))]);
        let answer = HashMap::from([("y".to_string(), Value::Float(9.0))]);
        assert_eq!(c1.simulate(&input).unwrap(), answer);

    }
}