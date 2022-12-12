use std::collections::HashMap;
use egui::{Pos2, Vec2, Ui};
use egui_extras::RetainedImage;

use crate::component::{Component, Value};

const MINIMIZED_COMPONENT_SIZE: Vec2 = Vec2{x: 150.0, y: 125.0};
const EXPANDED_COMPONENT_SIZE: Vec2 = Vec2{x: 400.0, y: 350.0};
const DEFAULT_ICON_SIZE: Vec2 = Vec2{x: 32.0, y: 32.0};

pub struct ComponentWindow {
    pub pos: Pos2,
    pub size: Vec2,
    pub highlight_rec: egui::Rect,
    pub expanded: bool,
    maximize_image: RetainedImage,
    minimize_image: RetainedImage,
    run_image: RetainedImage,
}

fn rows_from_hash(ui: &mut Ui, variables: &mut HashMap<String, Value>) {
    for (name, value) in variables {

        match value {
            Value::Float(val) => {
                ui.label(name.to_string());
                ui.add(egui::DragValue::new(val).speed(1.0));
                ui.end_row();                
            },
            Value::Vectorf32(values) => {
                ui.label(name.to_string());
                for val in values{
                    ui.add(egui::DragValue::new(val).speed(1.0));
                    ui.end_row();     
                }           

            }
        }
    }
}
impl ComponentWindow {

    // pub fn name(&self) -> String { self.component.name.clone() }

    pub fn new(pos: Pos2) -> Self {

        ComponentWindow { 
            pos: pos, 
            size: MINIMIZED_COMPONENT_SIZE.clone(),
            highlight_rec: egui::Rect { 
                min: Pos2{x:0.0, y:0.0}, 
                max: Pos2{x:10.0, y:10.0} 
            },
            expanded: false,
            maximize_image: RetainedImage::from_image_bytes("maximize.png", include_bytes!("../assets/maximize.png")).unwrap(),
            minimize_image: RetainedImage::from_image_bytes("maximize.png", include_bytes!("../assets/minimize.png")).unwrap(),
            run_image: RetainedImage::from_image_bytes("maximize.png", include_bytes!("../assets/triangle.png")).unwrap(),
        } 
    }

    pub fn center(&self) -> Pos2 {
        Pos2{
            x: self.pos.x + self.size.x / 2.0,
            y: self.pos.y + self.size.y / 2.0,
        }
    }

    pub fn create_window(&mut self, ctx: &egui::Context, component: &mut Component) {
        egui::Window::new(component.name.to_string())
            .title_bar(false)
            .fixed_size(if self.expanded { EXPANDED_COMPONENT_SIZE } else { MINIMIZED_COMPONENT_SIZE } )
            .default_pos(self.pos)
            .current_pos(self.pos)
            .show(ctx, |ui| {

                // Drag
                let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                if response.dragged() {
                    self.pos = self.pos + response.drag_delta();
                }

                // Load exapnded button texture
                // let img_size = 16.0 * minimize_texture.size_vec2() / minimize_texture.size_vec2().y;
                
                // If run clicked
                // if ui.add(egui::ImageButton::new(
                //     self.minimize_image.texture_id(ctx),
                //     Vec2::new(64.0, 64.0)
                // )).clicked(){
                //     let input = &component.required_input;
                //     component.required_output = component.simulate(input);
                // }

                // If expanded, add entry boxes
                if self.expanded {
                    
                    // Minimize button + click
                    if ui.add(egui::ImageButton::new(
                        self.minimize_image.texture_id(ctx),
                        DEFAULT_ICON_SIZE
                    )).clicked(){
                        self.expanded = false;
                    }

                    rows_from_hash(ui, &mut component.required_input);
                    rows_from_hash(ui, &mut component.required_output);
                } else {

                    // Maximize button + click
                    if ui.add(egui::ImageButton::new(
                        self.maximize_image.texture_id(ctx),
                        DEFAULT_ICON_SIZE
                    )).clicked(){
                        self.expanded = true;
                    }
                }

                // Set hightlight rectangle
                let rect = ui.min_rect();
                let mut min = rect.right_top();
                min.x -= 20.0;
                let mut max = rect.right_bottom();
                max.x += 20.0;

                ui.text_edit_singleline(&mut component.name.to_string());

                self.highlight_rec = egui::Rect{min:min, max:max};

            });
    }
}