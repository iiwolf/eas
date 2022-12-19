use std::collections::HashMap;
use egui::{Pos2, Vec2, Ui, Rect};
use egui_extras::RetainedImage;
use egui::Window;
const MINIMIZED_COMPONENT_SIZE: Vec2 = Vec2{x: 150.0, y: 150.0};
const EXPANDED_COMPONENT_SIZE: Vec2 = Vec2{x: 400.0, y: 350.0};
const DEFAULT_ICON_SIZE: Vec2 = Vec2{x: 32.0, y: 32.0};
const HIGHLIGHT_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE};
const DEFAULT_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::DARK_GRAY};
const BACKGROUND_COLOR: egui::Color32 = egui::Color32::from_rgb(27, 27, 27);
const PADDING: f32 = 5.0;

pub struct ComponentWindow {
    pub pos: Pos2,
    pub rect: Rect,
    pub size: Vec2,
    pub highlight_rec: Rect,
    pub expanded: bool,
    pub execution_string: String,
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
            execution_string: "None".to_string(),
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
    
    pub fn display(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        // Widget code can be broken up in four steps:
        //  1. Decide a size for the widget
        //  2. Allocate space for it
        //  3. Handle interactions with the widget (if any)
        //  4. Paint the widget
        egui::Area::new("my_area")
            .movable(true)
            .default_pos(egui::pos2(32.0, 32.0))
            .show(ctx, |ui| {

                ui.set_width(self.size.x);
                ui.set_height(self.size.y);

                
                let title_bar_size = Vec2{x:self.size.x, y: self.size.y * 0.10};
                let icon_size = Vec2{x:title_bar_size.y, y: title_bar_size.y};
                ui.with_layout(egui::Layout::top_down(egui::Align::RIGHT), |ui| {
                    ui.allocate_ui_with_layout(
                            title_bar_size,
                            egui::Layout::left_to_right(egui::Align::Min).with_main_justify(true), 
                            // ui.horizontal(
                            |ui| {

                            // ui.set_width(0.10 * MINIMIZED_COMPONENT_SIZE.x);
                            // ui.set_height(0.10 * MINIMIZED_COMPONENT_SIZE.y);
                            ui.add_space(PADDING);
                            ui.label("Component 1");
                            // ui.add_space(
                                // title_bar_size.x - icon_size.x - PADDING - 
                            // )
                            // If expanded, add entry boxes
                            if self.expanded {

                                // Minimize button + click
                                if ui.add(egui::ImageButton::new(
                                    self.minimize_image.texture_id(ctx),
                                    icon_size
                                )).clicked(){
                                    self.expanded = false;
                                    self.size = MINIMIZED_COMPONENT_SIZE;
                                }

                                // rows_from_hash(ui, &mut component.required_input);
                                // rows_from_hash(ui, &mut component.required_output);
                            } else {


                                // Maximize button + click
                                if ui.add(egui::ImageButton::new(
                                    self.maximize_image.texture_id(ctx),
                                    icon_size
                                )).clicked(){
                                    self.expanded = true;
                                    self.size = EXPANDED_COMPONENT_SIZE;
                                }
                            }

                        }
                    // )
                    );

                        
                    // If run clicked
                    if ui.add(egui::ImageButton::new(
                        self.run_image.texture_id(ctx),
                        DEFAULT_ICON_SIZE
                    )).clicked(){
                        println!("Simulate!");
                        // let input = &component.required_input;
                        // component.required_output = component.simulate(input);
                    }

            });
              

                // 4. Paint!
                // Make sure we need to paint:
                if ui.is_rect_visible(self.rect) {

                    // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
                    let radius = 5.0;
                    let border = Vec2{x:10.0, y:10.0};
                    self.rect = ui.min_rect();
                    
                    // Outer rectangle
                    ui.painter()
                        .rect(self.rect, radius, egui::Color32::default(), DEFAULT_STROKE);
                    
                    let inner_rect = egui::Rect{
                        min: self.rect.min + border,
                        max: self.rect.max - border,
                    };
            
                    // Inner rectangle
                    // ui.painter()
                    //     .rect(inner_rect, radius, BACKGROUND_COLOR, egui::Stroke::default());
                    
                }
                
                // Set hightlight rectangle
                let rect = ui.min_rect();
                let mut min = rect.right_top();
                min.x -= 20.0;
                let mut max = rect.right_bottom();
                max.x += 20.0;

                // ui.text_edit_singleline(&mut component.name.to_string());
                self.highlight_rec = egui::Rect{min:min, max:max};

            });    
            

        // let mut string = self.execution_string.clone();
        // ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&string));
        // Load exapnded button texture
        // let img_size = 16.0 * minimize_texture.size_vec2() / minimize_texture.size_vec2().y;
        


        // response
    }
}