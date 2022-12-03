use eframe::epaint::PathShape;
use egui::{Vec2, Pos2, Color32, Ui, Stroke, TextEdit, NumExt};
const COMPONENT_SIZE: Vec2 = Vec2{x: 150.0, y: 125.0};
const N_MAX_WINDOWS: i32 = 1000;
const CONNECTION_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE};
const GAP_SIZE: f32 = 10.0;

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct Component{
    name: String,
    pos: Pos2,
    components: Vec<Component>,
    connections: Vec<Connection>,
    active_connection: Option<Connection>,
}

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct Connection{
    // to: Component,
    // from: Component,
    p1: Pos2,
    p2: Pos2,
    connecting: bool,
}

impl Default for Component {
    fn default() -> Self {
        Self {
            name: "Empty".to_string(),
            pos: Pos2{x: 0.0, y: 0.0},
            components: Vec::default(),
            connections: Vec::default(),
            active_connection: None
        }
    }
}

impl Component{

    fn new(name: String) -> Self {
        Self {
            name: name,
            pos: Pos2{x: 0.0, y: 0.0},
            components: Vec::default(),
            connections: Vec::default(),
            active_connection: None
        }
    }

    fn create_window(&mut self, ctx: &egui::Context, parent_ui: &mut Ui) {
        egui::Window::new(self.name.to_string())
            .title_bar(false)
            .fixed_size(COMPONENT_SIZE)
            .default_pos(self.pos)
            .current_pos(self.pos)
            .show(ctx, |ui| {
                
                // Hover response for all
                let hover_response = ui.allocate_response(ui.available_size(), egui::Sense::hover());

                // Drag
                let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                if response.dragged() {
                    self.pos = self.pos + response.drag_delta();
                }

                // Adding arrow
                let rect = ui.min_rect();
                let edge = rect.right();
                let mut min = rect.right_top();
                min.x -= 10.0;
                let mut max = rect.right_bottom();
                max.x += 10.0;

                let right_edge_rect = egui::Rect{min:min, max:max};
                let edge_response = ui.allocate_rect(right_edge_rect,  egui::Sense::click());
                
                if edge_response.hovered(){
                    ui.painter().rect_filled(right_edge_rect, 0.0, egui::Color32::LIGHT_BLUE);
                }
                
                if edge_response.clicked() {

                    if self.active_connection.is_none() {
                        let pos = hover_response.hover_pos().unwrap();
                        self.active_connection = Some(Connection{p1: pos, p2: pos, connecting: true});
                    } else if self.active_connection.is_some() {
                        // self.connections.push(self.active_connection.take().unwrap());
                        
                        self.active_connection = None;
                    }
                }
                
                if self.active_connection.is_some() {
                    let mut connection = self.active_connection.as_mut().unwrap();
                    parent_ui.painter().line_segment([connection.p1, connection.p2], CONNECTION_STROKE);
                    connection.p2 = ctx.pointer_hover_pos().unwrap();
                }


                ui.text_edit_singleline(&mut self.name);
                // if ui.add(egui::Label::new("click me").sense(egui::Sense::click())).double_clicked() {
                //     println!("Double clicked!");
                //     ui.text_edit_singleline(&mut self.name).request_focus();

                // }
                // let mut label = egui::Label::new(*self.name);
                // label.sense(egui::Sense::click());
                // if ui.label(self.name.to_string()).double_clicked(){
                    // println!("Double clicked!");
                    // ui.text_edit_singleline(&mut self.name);
                // }

        });
    }


    fn create_globdule(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        
        let stroke = egui::Stroke{width: 1.0, color: egui::Color32::WHITE};
        let size = COMPONENT_SIZE[0];
        let delta = size * 3.0_f32.powf(0.5) / 3.0;
        let gamma = 30.0_f32.to_radians().tan() * size / 2.0;

        let points = vec![
            Pos2{x: self.pos[0], y: self.pos[1] + delta}, 
            Pos2{x: self.pos[0] + 0.5 * size, y: self.pos[1] - gamma}, 
            Pos2{x: self.pos[0] - 0.5 * size, y: self.pos[1] - gamma}, 
        ];
        ui.painter().add(PathShape::convex_polygon(points, Color32::GRAY, stroke));

        let mut origin = Pos2{x: self.pos[0] + gamma, y: self.pos[1]};
        for i in 1..10 {
            let points = vec![
                Pos2{x: origin[0], y: origin[1] + delta}, 
                Pos2{x: origin[0] + 0.5 * size, y: origin[1] - gamma}, 
                Pos2{x: origin[0] - 0.5 * size, y: origin[1] - gamma}, 
            ];
            ui.painter().add(PathShape::convex_polygon(points, Color32::GRAY, stroke));

            if i % 2 == 0 {
                origin[0] += gamma;
            }else{
                origin[1] += gamma;
            }
        }

        // ui.painter().
        let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                
        if response.dragged() {
            self.pos = self.pos + response.drag_delta();
        }
        

    }

}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    components: Vec<Component>,
    line_state: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {

        let mut c1 = Component{name: "6DoF".to_string(), pos: Pos2{x: 600.0, y: 200.0}, ..Default::default()};
        let mut c2 = Component{name: "Thermal".to_string(), pos: Pos2{x: 800.0, y: 200.0}, ..Default::default()};
        c1.components.push(c2);
        println!("{:?}", c1.components[0]);
        Self {
            components: vec![c1],
            line_state: 0.0
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

fn draw_grid(ui: &mut Ui, stroke: Stroke, line_state: &mut f32) {
                
    // Create grid lines because it's COOL and we're in the FUTURE
    let height = ui.available_height();
    let width = ui.available_width();
    let min_dim = height.min(width);
    let spacing = min_dim * 0.05;
    let margin = min_dim * 0.02;
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

    // if *line_state == 0 { 
    //     *line_state = self.time
    // }
    // let time = ui.input().unstable_dt.at_most(1.0 / 30.0) as f64;

    let time = ui.input().stable_dt.min(0.1);
    *line_state += time / 15.0;
    if *line_state >= 1.0{
        *line_state = 0.0;
    }
    // Horizontal lines
    for i in 1..n_horizontal_lines {

        // Segment 1
        let w1 = (width - margin) * *line_state - (i as f32) * 10.0;
        let w2 = w1 + GAP_SIZE;
        ui.painter().line_segment([
            Pos2{y: (i as f32 * spacing) as f32, x: margin} + offset.to_vec2(),
            Pos2{y: (i as f32 * spacing) as f32, x: w1} + offset.to_vec2()
        ],
        stroke);

        // Segment 2
        ui.painter().line_segment([
            Pos2{y: (i as f32 * spacing) as f32, x: w2} + offset.to_vec2(),
            Pos2{y: (i as f32 * spacing) as f32, x: width - margin} + offset.to_vec2()
        ],
        stroke);

    }

}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {components, line_state} = self;
        
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("Engineering Analysis Studio");

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.label("powered by ");
                    ui.hyperlink_to("egui", "https://github.com/emilk/egui");
                    ui.label(" and ");
                    ui.hyperlink_to(
                        "eframe",
                        "https://github.com/emilk/egui/tree/master/crates/eframe",
                    );
                    ui.label(".");
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Studio Floor");
            egui::warn_if_debug_build(ui);
            draw_grid(ui, egui::Stroke{width: 1.0, color: egui::Color32::from_gray(60)}, line_state);

            let parent_rect = ui.min_rect().right_bottom();
            let pos = parent_rect - Pos2{x:100.0, y:100.0}.to_vec2();
            let text = egui::RichText::new("Add Component").font(egui::FontId::proportional(40.0));
            ctx.request_repaint();
            egui::Area::new("add_button_area")
                .anchor(egui::Align2::RIGHT_BOTTOM, Vec2{x:-100.0, y:-100.0})
                .show(ctx, |ui| {
                    if ui.button(text).clicked() {
                        
                        // Create unique name
                        let names = Vec::from_iter(components.iter().map(|c| c.name.to_string()));
                        let mut default_name = "Empty".to_string();
                        for counter in 1..N_MAX_WINDOWS{
                            if !names.contains(&default_name){ break; }
                            default_name = format!("Empty_{}", counter);
                        }
                        components.push(Component::new(default_name));
                    }
                }
            );

            // ui_connected_windows(ui, ctx, components);
            for component in components{
                
                // Force component to be in bounds
                component.pos = component.pos.max(ui.min_rect().left_top());
                component.create_window(ctx, ui);

                for child_component in &mut component.components{
                    child_component.create_window(ctx, ui);
                    
                    // Create connection
                    ui.painter().line_segment(
                        [
                            component.pos + COMPONENT_SIZE / 2.0,
                            child_component.pos + COMPONENT_SIZE / 2.0],
                        CONNECTION_STROKE
                    );   

                }

            }
            


        });
        
        

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
