use std::collections::HashMap;
use egui::{Pos2, Vec2, Ui, Rect};
use egui_extras::RetainedImage;

const MINIMIZED_COMPONENT_SIZE: Vec2 = Vec2{x: 150.0, y: 125.0};
const EXPANDED_COMPONENT_SIZE: Vec2 = Vec2{x: 400.0, y: 350.0};
const DEFAULT_ICON_SIZE: Vec2 = Vec2{x: 32.0, y: 32.0};
const HIGHLIGHT_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE};
const DEFAULT_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::DARK_GRAY};

pub struct ComponentWindow {
    pub pos: Pos2,
    pub rect: Rect,
    pub size: Vec2,
    pub highlight_rec: Rect,
    pub expanded: bool,
    maximize_image: RetainedImage,
    minimize_image: RetainedImage,
    run_image: RetainedImage,
}

// fn rows_from_hash(ui: &mut Ui, variables: &mut HashMap<String, Value>) {
//     for (name, value) in variables {

//         match value {
//             Value::Float(val) => {
//                 ui.label(name.to_string());
//                 ui.add(egui::DragValue::new(val).speed(1.0));
//                 ui.end_row();                
//             },
//             Value::Vectorf32(values) => {
//                 ui.label(name.to_string());
//                 for val in values{
//                     ui.add(egui::DragValue::new(val).speed(1.0));
//                     ui.end_row();     
//                 }           

//             }
//         }
//     }
// }
impl ComponentWindow {

    // pub fn name(&self) -> String { self.component.name.clone() }

    pub fn new(pos: Pos2) -> Self {

        ComponentWindow { 
            pos: pos, 
            rect: Rect{min: pos, max: pos + MINIMIZED_COMPONENT_SIZE},
            size: MINIMIZED_COMPONENT_SIZE,
            highlight_rec: egui::Rect { 
                min: Pos2{x:0.0, y:0.0}, 
                max: Pos2{x:10.0, y:10.0} 
            },
            expanded: false,
            maximize_image: RetainedImage::from_image_bytes("maximize.png", include_bytes!("../../assets/maximize.png")).unwrap(),
            minimize_image: RetainedImage::from_image_bytes("maximize.png", include_bytes!("../../assets/minimize.png")).unwrap(),
            run_image: RetainedImage::from_image_bytes("maximize.png", include_bytes!("../../assets/triangle.png")).unwrap(),
        } 
    }

    pub fn center(&self) -> Pos2 {
        Pos2{
            x: self.pos.x + self.size.x / 2.0,
            y: self.pos.y + self.size.y / 2.0,
        }
    }

    // pub fn create_window(&mut self, ctx: &egui::Context, component: &mut Component) {
    
    pub fn display(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) -> egui::Response {
        // Widget code can be broken up in four steps:
        //  1. Decide a size for the widget
        //  2. Allocate space for it
        //  3. Handle interactions with the widget (if any)
        //  4. Paint the widget
    
        // 2. Allocating space:
        // This is where we get a region of the screen assigned.
        // We also tell the Ui to sense clicks in the allocated region.
        let response = ui.allocate_rect(self.rect, egui::Sense::click_and_drag());

        // Drag
        // let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
        if response.dragged() {
            // self.pos = self.pos + response.drag_delta();
            self.rect = self.rect.translate(response.drag_delta());
            // println!("{:?}", self.pos);
        }

        // Move to location
        // let rect = rect;

        // 4. Paint!
        // Make sure we need to paint:
        if ui.is_rect_visible(self.rect) {
    
            // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
            let radius = 5.0;
            let border = Vec2{x:10.0, y:10.0};
    
            // Outer rectangle
            ui.painter()
                .rect(self.rect, radius, egui::Color32::default(), DEFAULT_STROKE);
            
            let inner_rect = egui::Rect{
                min: self.rect.min + border,
                max: self.rect.max - border,
            };
    
            // Inner rectangle
            ui.painter()
                .rect(inner_rect, radius, egui::Color32::DARK_GRAY, egui::Stroke::default());
            
                
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

            // rows_from_hash(ui, &mut component.required_input);
            // rows_from_hash(ui, &mut component.required_output);
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

        // ui.text_edit_singleline(&mut component.name.to_string());
        self.highlight_rec = egui::Rect{min:min, max:max};

        response
    }
}