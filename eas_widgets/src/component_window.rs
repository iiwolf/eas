use egui::{Pos2, Rect, Ui, Vec2, Stroke};
use egui_extras::RetainedImage;
use std::collections::HashMap;
const MINIMIZED_COMPONENT_SIZE: Vec2 = Vec2 { x: 100.0, y: 100.0 };
const EXPANDED_COMPONENT_SIZE: Vec2 = Vec2 { x: 400.0, y: 400.0 };
const MAX_ICON_SIZE: Vec2 = Vec2 { x: 24.0, y: 24.0 };
const HIGHLIGHT_STROKE: egui::Stroke = egui::Stroke {
    width: 1.0,
    color: egui::Color32::LIGHT_BLUE,
};
const DEFAULT_STROKE: egui::Stroke = egui::Stroke {
    width: 1.0,
    color: egui::Color32::DARK_GRAY,
};
const MAJOR_GRID_STROKE: egui::Stroke = egui::Stroke {
    width: 1.0,
    color: egui::Color32::DARK_GRAY
};
const BACKGROUND_COLOR: egui::Color32 = egui::Color32::from_rgb(27, 27, 27);
const PADDING: f32 = 10.0;

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
    rust_logo: RetainedImage,
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

pub fn buffer_rect(rect: egui::Rect, margin: f32) -> egui::Rect {
    let border = Vec2 { x: margin, y: margin };
    egui::Rect {
        min: rect.min + border,
        max: rect.max - border,
    }
}

impl ComponentWindow {
    // pub fn name(&self) -> String { self.component.name.clone() }

    pub fn new(pos: Pos2) -> Self {
        ComponentWindow {
            pos: pos,
            rect: Rect {
                min: pos,
                max: pos + EXPANDED_COMPONENT_SIZE,
            },
            size: EXPANDED_COMPONENT_SIZE,
            highlight_rec: egui::Rect {
                min: Pos2 { x: 0.0, y: 0.0 },
                max: Pos2 { x: 10.0, y: 10.0 },
            },
            execution_string: "None".to_string(),
            expanded: true,
            maximize_image: RetainedImage::from_image_bytes(
                "maximize.png",
                include_bytes!("../../assets/top_right_purple.png"),
            )
            .unwrap(),
            minimize_image: RetainedImage::from_image_bytes(
                "minimize.png",
                include_bytes!("../../assets/bottom_left_purple.png"),
            )
            .unwrap(),
            run_image: RetainedImage::from_image_bytes(
                "triangle.png",
                include_bytes!("../../assets/triangle.png"),
            )
            .unwrap(),
            rust_logo: RetainedImage::from_image_bytes(
                "rust_logo.svg",
                include_bytes!("../../assets/rust_logo.svg"),
            )
            .unwrap(),

        }
    }

    pub fn center(&self) -> Pos2 {
        Pos2 {
            x: self.pos.x + self.size.x / 2.0,
            y: self.pos.y + self.size.y / 2.0,
        }
    }

    // pub fn create_window(&mut self, ctx: &egui::Context, component: &mut Component) {

    pub fn display(&mut self, ctx: &egui::Context, ui: &mut egui::Ui) {
        draw_grid(ui, MAJOR_GRID_STROKE);
        // Widget code can be broken up in four steps:
        //  1. Decide a size for the widget
        //  2. Allocate space for it
        //  3. Handle interactions with the widget (if any)
        //  4. Paint the widget
        let frame = egui::Frame::none()
            .fill(BACKGROUND_COLOR)
            // .outer_margin(10.0)
            .inner_margin(10.0)
            .stroke(MAJOR_GRID_STROKE)
            .rounding(10.0);
            
        egui::Window::new("my_area")
            .default_pos(egui::pos2(0.0, 0.0))
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .frame(frame)
            .show(ctx, |ui| {
                ui.set_width(self.size.x);
                ui.set_height(self.size.y);
                
                // ui.style_mut().spacing.button_padding = Vec2::new(0.0,0.0);
                // ui.set_style(style)
                self.rect = ui.min_rect();
                
                // Title Bar
                let title_bar_size = Vec2 {
                    x: self.size.x,
                    y: self.size.y * if self.expanded {0.10} else {0.20},
                };
                let icon_size = Vec2 {
                    x: title_bar_size.y.min(MAX_ICON_SIZE.x),
                    y: title_bar_size.y.min(MAX_ICON_SIZE.y),
                };
    
                let button_padding = ui.style().spacing.button_padding;

                // ui.add_space(PADDING);
                egui::TopBottomPanel::top("Component Title")
                .frame(egui::Frame::none())
                .show_inside(
                    ui,|ui| {
                        ui.with_layout(
                            egui::Layout::left_to_right(egui::Align::Center),
                    |ui| {
                        
                        // If run clicked
                        if ui
                            .add(
                            egui::ImageButton::new(
                                    self.run_image.texture_id(ctx),
                                    icon_size,
                                ).frame(false)
                            )
                            .clicked()
                        {
                            println!("Simulate!");
                            // let input = &component.required_input;
                            // component.required_output = component.simulate(input);
                        }

                        // Manually size component label to take up space - icon_size
                        let rect = ui.available_rect_before_wrap();
                        let text_rect = egui::Rect::from_min_max(
                            rect.min + Vec2::new(icon_size.x, 0.0),
                            rect.max - Vec2::new(icon_size.x, 0.0)
                        );

                        let mut label = egui::RichText::new("Component 1");
                        if self.expanded {
                            label = label.heading().strong();
                        } 
                        // else {
                            // label = label;
                        // }
                        ui.put(text_rect, egui::Label::new(label));
                        
                        // If expanded, add entry boxes
                        if self.expanded {
                            // Minimize button + click
                            if ui
                                // .put(widget_rect,
                                .add(
                                    egui::ImageButton::new(
                                    self.minimize_image.texture_id(ctx),
                                    icon_size,
                                ).frame(false))
                                .clicked()
                            {
                                self.expanded = false;
                                self.size = MINIMIZED_COMPONENT_SIZE;
                            }
        
                            // rows_from_hash(ui, &mut component.required_input);
                            // rows_from_hash(ui, &mut component.required_output);
                        } else {
                            // Maximize button + click
                            if ui
                                // .put(widget_rect, 
                                .add(
                                    egui::ImageButton::new(
                                    self.maximize_image.texture_id(ctx),
                                    icon_size,
                                ).frame(false))
                                .clicked()
                            {
                                self.expanded = true;
                                self.size = EXPANDED_COMPONENT_SIZE;
                            }
                        }
                    });
                });
                ui.separator();

                egui::CentralPanel::default().show_inside(ui, |ui| {

                    if self.expanded {

                        // Code edit
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut self.execution_string)
                                    .font(egui::TextStyle::Monospace) // for cursor height
                                    .code_editor()
                                    .desired_rows(23)
                                    .lock_focus(true)
                                    .desired_width(ui.available_width())
                            );
                        });

                    } else {

                        // Else just logo
                        let rect = buffer_rect(self.rect, 0.15 * self.size.x);
                        ui
                        // .put(rect, 
                            .add(                            egui::ImageButton::new(
                            self.rust_logo.texture_id(ctx),
                            // rect.max - rect.min,
                            ui.available_size()
                        ).frame(false));
                    }

                });

                // Set hightlight rectangle
                let rect = ui.min_rect();
                let mut min = rect.right_top();
                min.x -= 20.0;
                let mut max = rect.right_bottom();
                max.x += 20.0;

                // ui.text_edit_singleline(&mut component.name.to_string());
                self.highlight_rec = egui::Rect { min: min, max: max };
            });

        // let mut string = self.execution_string.clone();
        // ui.add_sized(ui.available_size(), egui::TextEdit::multiline(&string));
        // Load exapnded button texture
        // let img_size = 16.0 * minimize_texture.size_vec2() / minimize_texture.size_vec2().y;

        // response
    }
}


pub fn draw_grid(ui: &mut Ui, stroke: Stroke) {
                
    // Create grid lines because it's COOL and we're in the FUTURE
    let height = ui.available_height();
    let width = ui.available_width();
    let min_dim = height.min(width);
    let spacing = min_dim * 0.05;
    let margin = min_dim * 0.0;
    let n_vertical_lines = (width / spacing).round() as i32;  
    let n_horizontal_lines = (height / spacing).round() as i32;  
    let offset = ui.min_rect().left_top();
    
    // Vertical lines
    for i in 1..n_vertical_lines {
        ui.painter().line_segment([
            Pos2{x: (i as f32 * spacing) as f32, y: margin} + offset.to_vec2(),
            Pos2{x: (i as f32 * spacing) as f32, y: height - margin} + offset.to_vec2()
        ],
        stroke);
    }

    // Horizontal lines
    for i in 1..n_horizontal_lines {
        ui.painter().line_segment([
            Pos2{y: (i as f32 * spacing) as f32, x: margin} + offset.to_vec2(),
            Pos2{y: (i as f32 * spacing) as f32, x: width - margin} + offset.to_vec2()
        ],
        stroke);
    }
    
}
