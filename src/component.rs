use std::{collections::HashMap, iter::Enumerate};
use evalexpr::*;
use egui::{Vec2, Pos2, Color32, Ui, Stroke, TextEdit, NumExt};

const COMPONENT_SIZE: Vec2 = Vec2{x: 150.0, y: 125.0};


// #[derive(serde::Deserialize, serde::Serialize, Debug)]
#[derive(Debug, Clone, PartialEq)]
pub enum Value{
    // Integer(i32),
    Float(f32),
    // String(&'static str),
    // Vectori32(Vec<f32>),
    Vectorf32(Vec<f32>),
    // VectorString(Vec<f32>),
}

impl From<evalexpr::Value> for Value {
    fn from(value: evalexpr::Value) -> Self {
        Value::Float(value.as_float().unwrap() as f32)
    }
}

// #[derive(serde::Deserialize, serde::Serialize, Debug)]
#[derive(Debug)]
pub struct Component{
    pub name: String,
    pub eval_expression: String,
    pub required_input: HashMap<String, Value>,
    pub required_output: HashMap<String, Value>,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            name: "Empty".to_string(),
            eval_expression: String::new(),
            required_input: HashMap::new(),
            required_output: HashMap::new(),
        }
    }
}

impl Component{

    pub fn new(name: String) -> Self {
        Self {
            name: name,
            eval_expression: String::new(),
            required_input: HashMap::new(),
            required_output: HashMap::new(),
        }
    }

    pub fn simulate(&self, input: &HashMap<String, Value>) -> HashMap<String, Value> {
        
        // Create mutable clone
        let mut eval_string = self.eval_expression.clone();
        
        // Debug
        println!("Evaluating expression:    {:?}", eval_string);

        // Process RHS, replacing each input variable with value
        for (variable, value) in &self.required_input {

            if input.contains_key(variable) {
                eval_string = match value {
                    Value::Float(val) => eval_string.replace(variable, &format!("{:?}", val)),
                    Value::Vectorf32(vec32) => {
                        for (i, value) in vec32.iter().enumerate() {
                            eval_string = eval_string.replace(
                                &format!("{}{:?}", variable, i), 
                                &format!("{:?}", value)
                            );
                        }
                        eval_string
                    }
                }
            } else {
                println!("Invalid input: missing input {:?}", variable);
                return HashMap::new()
            }
            
        }

        // Debug
        println!("                          {:?}", eval_string);

        // Create HashMapContext to get multiple assignments
        let mut context = HashMapContext::new();
        
        // Run code
        let result = eval_with_context_mut(&eval_string, &mut context);

        // Create new hashmap to store variables
        let mut output_hash = HashMap::new();
        
        // evalexpr ran properly
        if result.is_ok() {

            // Assign variables from calculation
            for (variable, value) in context.iter_variables() {
                
                if self.required_output.contains_key(&variable) {
                    // Convert evalexpr::Value to eas::Value
                    output_hash.insert(variable, Value::from(value));
                } else {
                    println!("Warning: unused output variable {:?}", variable);
                }
            }

        // evalexpr failed
        }else{
            println!("Failed to run properly!\n\t{:?}", result.unwrap_err());
        }

        // Return output has either way
        output_hash
    }

    // pub fn create_window(&mut self, ctx: &egui::Context, parent_ui: &mut Ui, active_connection: &mut Option<Connection>) {
    //     egui::Window::new(self.name.to_string())
    //         .title_bar(false)
    //         .fixed_size(COMPONENT_SIZE)
    //         .default_pos(self.pos)
    //         .current_pos(self.pos)
    //         .show(ctx, |ui| {
                

    //             // Drag
    //             let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
    //             if response.dragged() {
    //                 self.pos = self.pos + response.drag_delta();
    //             }
                
    //             // Adding arrow
    //             let rect = ui.min_rect();
    //             let mut min = rect.right_top();
    //             min.x -= 10.0;
    //             let mut max = rect.right_bottom();
    //             max.x += 10.0;

    //             let right_edge_rect = egui::Rect{min:min, max:max};
    //             let edge_response = ui.allocate_rect(right_edge_rect,  egui::Sense::click());
                
    //             // Highlight affect on edge hover
    //             if edge_response.hovered(){
    //                 ui.painter().rect_filled(right_edge_rect, 0.0, egui::Color32::LIGHT_BLUE);
    //             }
                
    //             if edge_response.clicked() {

    //                 // Start arrow
    //                 if active_connection.is_none() {
    //                     let pos = edge_response.hover_pos().unwrap();
    //                     *active_connection = Some(Connection{p1: pos, p2: pos, connecting: true});

    //                 // Finish connection
    //                 } else if active_connection.is_some() {
    //                     self.connections.push(active_connection.take().unwrap());
    //                     *active_connection = None;
    //                 }
    //             }
                
    //             if active_connection.is_some() {
    //                 let mut connection = active_connection.as_mut().unwrap();
    //                 parent_ui.painter().line_segment([connection.p1, connection.p2], CONNECTION_STROKE);
    //                 connection.p2 = ctx.pointer_hover_pos().unwrap();
    //             }


    //             ui.text_edit_singleline(&mut self.name);


    //     });
    // }

}
