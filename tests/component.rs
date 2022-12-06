extern crate eas;

#[cfg(test)]
mod tests{
    use std::{collections::HashMap};


    #[test]
    fn test_simulate(){
        use eas::component::{Component, Value};
        let mut c1 = Component{
            name: "square".to_string(),
            eval_expression: String::from("y = x ^ 2"), 
            required_input: HashMap::from([
                ("x".to_string(), Value::Float(3.0))
            ]),
            required_output: HashMap::from([
                ("y".to_string(), Value::Float(0.0)),
            ])
        };

        let mut c2 = Component{
            name: "addition".to_string(),
            eval_expression: String::from("z = x + 100.0 / (y0 + y1)"), 
            required_input: HashMap::from([
                ("x".to_string(), Value::Float(9.0)),
                ("y".to_string(), Value::Vectorf32(vec![5.0, 2.0])),
            ]),
            required_output: HashMap::from([
                ("z".to_string(), Value::Float(0.0)),
            ])
            
        };

        let input = HashMap::from([("x".to_string(), Value::Float(3.0))]);
        let answer = HashMap::from([("y".to_string(), Value::Float(9.0))]);
        assert_eq!(c1.simulate(&input), answer);
        // assert_eq!(
        //     c2.simulate(&HashMap::from([
        //         ("x".to_string(), Value::Float(9.0)),
        //         ("y".to_string(), Value::Vectorf32(vec![5.0, 2.0])),
        //     ])), 
        //     HashMap::from([("y", 19.0)])
        // );

    }
}