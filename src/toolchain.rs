use crate::component::{Component, Value};
use std::{collections::HashMap, ops::Deref};

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

            if !results.is_empty() {
                
                // Next input is combination of what's defined in component and what
                // other components have produced
                // let next_input = component.input.clone();
                
                // Take last results, and assign
                // let last_results = results.last().unwrap().clone();
                // for key in next_input.keys() {
                for key in results.last().unwrap().keys() {
                    if component.input.contains_key(key) {
                        *component.input.get_mut(key).unwrap() = results.last().unwrap()[key].clone();
                        // *component.input.get_mut(key).unwrap() = results.last().unwrap()[key].clone(); //last_results[key].clone();
                    }
                }
                // Current input consists of running data + input
                running_data_map = component.input.clone();
                // running_data_map.extend();
            }

            // Simulate and append to results
            let result = component.simulate(&running_data_map);
            
            // Technically result could have just a filesystem effect and no output - not sure
            // if I want this to be a secondary component yet
            if result.is_some() {
                
                let result = result.unwrap();
                // Update running data with output
                running_data_map.extend(result.clone());

                // Push into vec
                results.push(result);
            }

        }
        results
    }
}