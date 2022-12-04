use crate::component::Component;
pub struct Toolchain {
    pub components: Vec<Component>
}

impl Toolchain {
    pub fn simulate(&self, mut input_val: f32) -> f32{

        let mut results = Vec::new();
        for component in &self.components {
            
            // Simulate and append to results
            let result = component.simulate();
            results.push(result);

            // Input val for next iteration
            input_val = result;
        }
        input_val
    }
}