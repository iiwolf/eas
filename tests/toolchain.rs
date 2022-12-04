extern crate eas;

#[cfg(test)]
mod tests{

    #[test]
    fn test_data_transfer(){
        use eas::component::Component;
        use eas::toolchain::Toolchain;

        let mut c1 = Component{name: "square".to_string(), eval_expression: String::from("x ^ 2"), ..Default::default()};
        let mut c2 = Component{name: "addition".to_string(), eval_expression: String::from("x + 100.0"), ..Default::default()};
        let mut tc = Toolchain{components: vec![c1, c2]};
        assert_eq!(tc.simulate(3.0), 109.0);

    }
}