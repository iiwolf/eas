use egui::{Vec2, Pos2, Color32, Ui, Stroke, TextEdit, NumExt};
use crate::component::Component;
use crate::connection::{Connection, CONNECTION_STROKE};

const COMPONENT_SIZE: Vec2 = Vec2{x: 150.0, y: 125.0};

pub struct ComponentWindow {
    pub pos: Pos2,
    pub highlight_rec: egui::Rect,
}

impl ComponentWindow {

    // pub fn name(&self) -> String { self.component.name.clone() }

    pub fn new(pos: Pos2) -> Self {
        ComponentWindow { 
            pos: pos, 
            highlight_rec: egui::Rect { 
                min: Pos2{x:0.0, y:0.0}, 
                max: Pos2{x:10.0, y:10.0} 
            }
        } 
    }
    pub fn create_window(&mut self, ctx: &egui::Context, parent_ui: &mut Ui, component: &Component) {
        egui::Window::new(component.name.to_string())
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
                ui.text_edit_singleline(&mut component.name.to_string());
                
                // Set hightlight rectangle
                let rect = ui.min_rect();
                let mut min = rect.right_top();
                min.x -= 10.0;
                let mut max = rect.right_bottom();
                max.x += 10.0;

                self.highlight_rec = egui::Rect{min:min, max:max};

            });
    }
}