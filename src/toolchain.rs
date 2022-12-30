use crate::component::{Component, Value};
use std::{collections::HashMap};

pub struct Toolchain {
    pub components: Vec<Component>,
    pub is_active: bool,
}

impl Toolchain {
    
    pub fn new(components: Vec<Component>) -> Self {
        Toolchain { components: components, is_active: false }
    }

    pub fn simulate(&mut self, input_data: &HashMap<String, Value>) -> Vec<HashMap<String, Value>>{

        let mut results: Vec<HashMap<String, Value>> = Vec::new();
        let mut running_data_map = input_data.clone();
        for component in self.components.iter_mut() {
            
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