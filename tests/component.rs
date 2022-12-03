extern crate eas;

#[cfg(test)]
mod tests{

    #[test]
    fn test_data_transfer(){
        use eas::component::Component;
        struct Connection {
            from: Component,
            to: Component,
        }

        let mut c1 = Component{name: "square".to_string(), eval_expression: String::from("x ^ 2"), ..Default::default()};
        let mut c2 = Component{name: "addition".to_string(), eval_expression: String::from("x + 100.0"), ..Default::default()};
        let mut connection: Connection = Connection {from: c1, to: c2 };

        assert_eq!(c1.simulate(3.0), 9.0);
        assert_eq!(c2.simulate(9.0), 109.0);
        assert_eq!(c2.simulate(c1.simulate(3.0)), 109.0);
    }
}