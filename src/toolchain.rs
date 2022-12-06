use crate::component::{Component, Value};
use std::{collections::HashMap};

pub struct Toolchain {
    pub components: Vec<Component>
}

impl Toolchain {
    pub fn simulate(&self, input_data: &HashMap<String, Value>) -> Vec<HashMap<String, Value>>{

        let mut results: Vec<HashMap<String, Value>> = Vec::new();
        let mut running_data_map = input_data.clone();
        for component in &self.components {
            
            // Simulate and append to results
            let result = component.simulate(&running_data_map);
            
            // Update running data with output
            running_data_map.extend(result.clone());

            // Push into vec
            results.push(result);

        }
        results
    }
}