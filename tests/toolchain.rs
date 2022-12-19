extern crate eas;

#[cfg(test)]
mod tests{

    #[test]
    fn test_data_transfer(){
        use eas::component::{Component, Value};
        use eas::toolchain::Toolchain;
        use std::collections::HashMap;
        let mut c1 = Component{
            name: "square".to_string(),
            eval_expression: String::from("x = y ^ 2"), 
            required_input: HashMap::from([
                ("y".to_string(), Value::Float(3.0))
            ]),
            required_output: HashMap::from([
                ("x".to_string(), Value::Float(0.0)),
            ])
        };

        let mut c2 = Component{
            name: "addition".to_string(),
            eval_expression: String::from("z = x + 100.0 / (a0 + a1)"), 
            required_input: HashMap::from([
                ("x".to_string(), Value::Float(9.0)),
                ("a".to_string(), Value::Vectorf32(vec![5.0, 5.0])),
            ]),
            required_output: HashMap::from([
                ("z".to_string(), Value::Float(0.0)),
            ])
            
        };

        // let mut tc = Toolchain{components: vec![c1, c2]};
        // let input = HashMap::from([
        //     ("y".to_string(), Value::Float(3.0)),
        //     ("a".to_string(), Value::Vectorf32(vec![5.0, 5.0])),
        // ]);
        // let answer = vec![
        //     HashMap::from([("x".to_string(), Value::Float(9.0))]),
        //     HashMap::from([("z".to_string(), Value::Float(19.0))])
        // ];
        // assert_eq!(tc.simulate(&input), answer);

    }
}