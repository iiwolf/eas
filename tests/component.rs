extern crate eas;

#[cfg(test)]
mod tests{
    use std::collections::HashMap;


    #[test]
    fn test_simulate(){
        use eas::component::{Component, Value};
        let mut c1 = Component{
            name: "square".to_string(),
            eval_expression: String::from("x ^ 2"), 
            input_data: HashMap::from([
                ("x".to_string(), Value::Float(3.0))
            ])
        };

        let mut c2 = Component{
            name: "addition".to_string(),
            eval_expression: String::from("x + 100.0 / (y0 + y1)"), 
            input_data: HashMap::from([
                ("x".to_string(), Value::Float(9.0)),
                ("y".to_string(), Value::Vectorf32(vec![5.0, 2.0])),
            ])
            
        };

        assert_eq!(c1.simulate(), 9.0);
        assert_eq!(c2.simulate(), 19.0);
    }
}