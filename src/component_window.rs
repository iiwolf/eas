use egui::{Pos2, Rect, Ui, Vec2, Stroke};
use egui_extras::RetainedImage;
use std::collections::HashMap;
use crate::component::{Component, Value};

// Style constants
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

    // Component information
    // pub component: Component,
    // pub execution_string: String,

    // State tracking
    pos: Pos2,
    default_pos: Pos2,
    rect: Rect,
    size: Vec2,
    pub highlight_rec: Rect,
    expanded: bool,

    // Buttons
    maximize_image: RetainedImage,
    minimize_image: RetainedImage,
    run_image: RetainedImage,
    rust_logo: RetainedImage,

    // Input/output
    pub input: HashMap<String, Value>,
    pub output: HashMap<String, Value>,

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

            // Position, size, etc.
            default_pos: pos,
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
            expanded: true,

            // Buttons
            maximize_image: RetainedImage::from_image_bytes(
                "maximize.png",
                include_bytes!("../assets/top_right_purple.png"),
            )
            .unwrap(),
            minimize_image: RetainedImage::from_image_bytes(
                "minimize.png",
                include_bytes!("../assets/bottom_left_purple.png"),
            )
            .unwrap(),
            run_image: RetainedImage::from_image_bytes(
                "triangle.png",
                include_bytes!("../assets/triangle.png"),
            )
            .unwrap(),
            rust_logo: RetainedImage::from_image_bytes(
                "rust_logo.svg",
                include_bytes!("../assets/rust_logo.svg"),
            )
            .unwrap(),

            // Data
            input: HashMap::new(),
            output: HashMap::new(),

        }
    }

    pub fn center(&self) -> Pos2 {
        Pos2 {
            x: self.pos.x + self.size.x / 2.0,
            y: self.pos.y + self.size.y / 2.0,
        }
    }

    // pub fn create_window(&mut self, ctx: &egui::Context, component: &mut Component) {

    pub fn display(&mut self, ctx: &egui::Context, ui: &mut egui::Ui, component: &mut Component) {

        let frame = egui::Frame::none()
            .fill(BACKGROUND_COLOR)
            // .outer_margin(10.0)
            .inner_margin(10.0)
            .stroke(MAJOR_GRID_STROKE)
            .rounding(10.0);

        egui::Window::new(&component.name)
            .default_pos(self.default_pos)
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .frame(frame)
            .show(ctx, |ui| {

                // Fix height and width based on minimized or expanded
                ui.set_width(self.size.x);
                ui.set_height(self.size.y);
                self.rect = ui.min_rect();

                // Title Bar
                let title_bar_size = Vec2 {
                    x: self.size.x,
                    y: self.size.y * if self.expanded {0.20} else {0.10},
                };
                let icon_size = Vec2 {
                    x: title_bar_size.y.min(MAX_ICON_SIZE.x),
                    y: title_bar_size.y.min(MAX_ICON_SIZE.y),
                };
                

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
                                self.output = component.simulate(&self.input);
                            }

                            // Manually size component label to take up space - icon_size
                            let rect = ui.available_rect_before_wrap();
                            let text_rect = egui::Rect::from_min_max(
                                rect.min + Vec2::new(icon_size.x, 0.0),
                                rect.max - Vec2::new(icon_size.x, 0.0)
                            );

                            let mut label = egui::RichText::new(&component.name);
                            if self.expanded {
                                label = label.heading().strong();
                            } else {
                                label = label.small();
                            }

                            ui.put(text_rect, egui::Label::new(label));
                            
                            // If expanded, add entry boxes
                            if self.expanded {

                                // Minimize button + click
                                if ui
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
                
                if self.expanded {
                    let side_panel_size = self.size.x * 0.25;

                    // Input side panel
                    egui::SidePanel::left("Inputs")
                        .frame(egui::Frame::default())
                        .resizable(true)
                        .default_width(side_panel_size)
                        .show_inside(ui, |ui| {
                            ui.label("Inputs");
                            rows_from_hash(ui, &mut self.input);
                        });

                    // Output side panel
                    egui::SidePanel::right("Outputs")
                        .frame(egui::Frame::default())
                        .resizable(true)
                        .default_width(side_panel_size)
                        .show_inside(ui, |ui| {
                            ui.label("Outputs");
                            rows_from_hash(ui, &mut self.output);
                        });
                }

                // Core app 
                egui::CentralPanel::default().show_inside(ui, |ui| {

                    if self.expanded {

                        // Code edit
                        egui::ScrollArea::vertical().show(ui, |ui| {
                            ui.add(
                                egui::TextEdit::multiline(&mut component.eval_expression)
                                    .font(egui::TextStyle::Monospace) // for cursor height
                                    .code_editor()
                                    .desired_rows(23)
                                    .lock_focus(true)
                                    .desired_width(self.size.x * 0.5)
                            );
                        });

                    } else {

                        // Compute square from remanining available size
                        let size = Vec2::new(ui.available_height(), ui.available_height());

                        let rust_logo = egui::ImageButton::new(self.rust_logo.texture_id(ctx),size).frame(false);
                        ui.with_layout(egui::Layout::top_down(egui::Align::Center),
                         |ui| {
                            ui.add_sized(size, rust_logo);
                        });

                    }

                });

                self.pos = ui.min_rect().left_top();

                // Set hightlight rectangle
                let rect = ui.min_rect();
                let mut min = rect.right_top();
                min.x -= 20.0;
                let mut max = rect.right_bottom();
                max.x += 20.0;

                // ui.text_edit_singleline(&mut component.name.to_string());
                self.highlight_rec = egui::Rect { min: min, max: max };
            });
        
        // self.pos = window.unwrap().inner.unwrap().
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
