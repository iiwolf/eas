use evalexpr::*;
use egui::{Vec2, Pos2, Color32, Ui, Stroke, TextEdit, NumExt};
use crate::connection::Connection;
pub const CONNECTION_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE};
const COMPONENT_SIZE: Vec2 = Vec2{x: 150.0, y: 125.0};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Component{
    pub name: String,
    pub pos: Pos2,
    pub connections: Vec<Connection>,
    pub active_connection: Option<Connection>,
    pub eval_expression: String,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            name: "Empty".to_string(),
            pos: Pos2{x: 0.0, y: 0.0},
            connections: Vec::default(),
            active_connection: None,
            eval_expression: String::new()
        }
    }
}

impl Component{

    pub fn new(name: String) -> Self {
        Self {
            name: name,
            pos: Pos2{x: 0.0, y: 0.0},
            connections: Vec::default(),
            active_connection: None,
            eval_expression: String::new(),
        }
    }

    pub fn simulate(&mut self, input_val: f32) -> f32 {
        let input_val = &format!("{:?}", input_val);
        println!("{}", input_val);
        let eval_string = &self.eval_expression.replace("x", input_val);
        println!("Evaluating string: \n\t\"{}\"", eval_string);
        let result = eval(&eval_string);
        if result.is_ok() {
            result.unwrap().as_float().unwrap() as f32
        }else{
            println!("Failed to run properly!\n\t{:?}", result.unwrap_err());
            0.0
        }
        // .unwrap().as_float()
    }

    pub fn create_window(&mut self, ctx: &egui::Context, parent_ui: &mut Ui, active_connection: &mut Option<Connection>) {
        egui::Window::new(self.name.to_string())
            .title_bar(false)
            .fixed_size(COMPONENT_SIZE)
            .default_pos(self.pos)
            .current_pos(self.pos)
            .show(ctx, |ui| {
                

                // Drag
                let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                if response.dragged() {
                    self.pos = self.pos + response.drag_delta();
                }
                
                // Adding arrow
                let rect = ui.min_rect();
                let mut min = rect.right_top();
                min.x -= 10.0;
                let mut max = rect.right_bottom();
                max.x += 10.0;

                let right_edge_rect = egui::Rect{min:min, max:max};
                let edge_response = ui.allocate_rect(right_edge_rect,  egui::Sense::click());
                
                // Highlight affect on edge hover
                if edge_response.hovered(){
                    ui.painter().rect_filled(right_edge_rect, 0.0, egui::Color32::LIGHT_BLUE);
                }
                
                if edge_response.clicked() {

                    // Start arrow
                    if active_connection.is_none() {
                        let pos = edge_response.hover_pos().unwrap();
                        *active_connection = Some(Connection{p1: pos, p2: pos, connecting: true});

                    // Finish connection
                    } else if active_connection.is_some() {
                        self.connections.push(active_connection.take().unwrap());
                        *active_connection = None;
                    }
                }
                
                if active_connection.is_some() {
                    let mut connection = active_connection.as_mut().unwrap();
                    parent_ui.painter().line_segment([connection.p1, connection.p2], CONNECTION_STROKE);
                    connection.p2 = ctx.pointer_hover_pos().unwrap();
                }


                ui.text_edit_singleline(&mut self.name);


        });
    }

}
