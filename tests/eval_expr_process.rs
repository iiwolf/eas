extern crate eas;

#[cfg(test)]
mod tests{
    use std::{collections::HashMap};

    use eas::eval_expr_process::EvalExprProcess;


    #[test]
    fn test_simulate(){
        use eas::component::{Component, Value};
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
        // assert_eq!(
        //     c2.simulate(&HashMap::from([
        //         ("x".to_string(), Value::Float(9.0)),
        //         ("y".to_string(), Value::Vectorf32(vec![5.0, 2.0])),
        //     ])), 
        //     HashMap::from([("y", 19.0)])
        // );

    }
}