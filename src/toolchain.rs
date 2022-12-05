use crate::component::{Component, Value};
use std::{collections::HashMap};

pub struct Toolchain {
    pub components: Vec<Component>
}

impl Toolchain {
    pub fn simulate(&self, input_data: &HashMap<String, Value>) -> Vec<HashMap<String, Value>>{

        let mut results: Vec<HashMap<String, Value>> = Vec::new();
        for (i, component) in self.components.iter().enumerate() {
            
            // Simulate and append to results
            let result = match i {
                0 => component.simulate(input_data),
                _ => component.simulate(&results[results.len()-1]),
            };

            // Push into vec
            results.push(result);

        }
        results
    }
}