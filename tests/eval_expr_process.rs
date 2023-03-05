extern crate eas;

#[cfg(test)]
mod tests{
    use std::{collections::HashMap};
    use eas::{eval_expr_process::EvalExprProcess, component::Value, execution_process::ExecutionProcess};


    #[test]
    fn test_simulate(){
        
        let mut process = EvalExprProcess::new("y = x ^ 2".to_string());
        let input = HashMap::from([('x'.to_string(), Value::Float(2.0))]);
        let output_hash = process.simulate(&input);
        assert_eq!(Some(&Value::Float(4.0)), output_hash.as_ref().unwrap().get("y"));
    }
}