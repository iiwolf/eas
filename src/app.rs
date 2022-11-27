use eframe::epaint::PathShape;
use egui::{Vec2, Pos2, Color32, Ui, Stroke};

#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct Component{
    name: String,
    pos: Pos2,
    size: Vec2,
    components: Vec<Component>
}

impl Component{
    fn create_window(&mut self, ctx: &egui::Context) {
        egui::Window::new(self.name.to_string())
            .fixed_size(self.size)
            .default_pos(self.pos)
            .current_pos(self.pos)
            .show(ctx, |ui| {
                
                let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                
                if response.dragged() {
                    self.pos = self.pos + response.drag_delta();
                }
                
                ui.label(self.name.to_string());
                
        });
    }

    fn create_child_window(&mut self, ctx: &egui::Context, ui: &mut Ui) {

        self.create_window(ctx);

    }

    fn create_globdule(&mut self, ctx: &egui::Context, ui: &mut Ui) {
        
        let stroke = egui::Stroke{width: 1.0, color: egui::Color32::WHITE};
        // let mesh = egui::Mesh{
        //     // indices: vec![0, 1, 2],
        //     // vertices: vec![
        //     //     egui::epaint::Vertex{pos: self.pos, ..Default::default()},
        //     //     egui::epaint::Vertex{pos: self.pos + self.size / 2.0, ..Default::default()},
        //     //     egui::epaint::Vertex{pos: self.pos - self.size / 2.0, ..Default::default()},
        //     // ], 
        //     // texture_id: ctx
        //     ..Default::default()
        // };
        // let size = 100.0;
        // ui.painter().line_segment([self.pos, Pos2{x: self.pos[0] + size, y: self.pos[1]}], egui::Stroke{width: 1.0, color: egui::Color32::WHITE});
        // ui.painter().line_segment([self.pos, Pos2{x: self.pos[0], y: self.pos[1]  + size}], egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE});
        // ui.painter().line_segment([Pos2{x: self.pos[0] + size, y: self.pos[1]}, Pos2{x: self.pos[0], y: self.pos[1]  + size}], egui::Stroke{width: 1.0, color: egui::Color32::RED});
        // ui.painter().line_segment([self.pos, self.pos - self.size / 2.0], egui::Stroke{width: 1.0, color: egui::Color32::RED});
        // ui.painter().line_segment([self.pos + self.size / 2.0, self.pos - self.size / 2.0], egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE});
        let size = self.size[0];
        let delta = size * 3.0_f32.powf(0.5) / 3.0;
        let gamma = 30.0_f32.to_radians().tan() * size / 2.0;

        let points = vec![
            Pos2{x: self.pos[0], y: self.pos[1] + delta}, 
            Pos2{x: self.pos[0] + 0.5 * size, y: self.pos[1] - gamma}, 
            Pos2{x: self.pos[0] - 0.5 * size, y: self.pos[1] - gamma}, 
        ];
        ui.painter().add(PathShape::convex_polygon(points, Color32::GRAY, stroke));

        // for component in &mut self.components{

        //     // let origin = self.pos + Pos2 { x: gamma, y: gamma }.to_vec2();
        //     let points = vec![
        //         Pos2{x: component.pos[0], y: component.pos[1] + delta}, 
        //         Pos2{x: component.pos[0] + 0.5 * size, y: component.pos[1] - gamma}, 
        //         Pos2{x: component.pos[0] - 0.5 * size, y: component.pos[1] - gamma}, 
        //     ];
        // ui.painter().add(PathShape::convex_polygon(points, Color32::GRAY, stroke));

        // }

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
}

impl Default for TemplateApp {
    fn default() -> Self {

        let mut c1 = Component{name: "6DoF".to_string(), pos: Pos2{x: 600.0, y: 200.0}, size: Vec2{x: 100.0, y: 100.0}, ..Default::default()};
        let mut c2 = Component{name: "Thermal".to_string(), pos: Pos2{x: 800.0, y: 200.0}, size: Vec2{x: 100.0, y: 100.0}, ..Default::default()};
        c1.components.push(c2);
        println!("{:?}", c1.components[0]);
        Self {
            components: vec![c1],
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

fn draw_grid(ui: &mut Ui, stroke: Stroke) {
                
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

    // Horizontal lines
    for i in 1..n_horizontal_lines {
        ui.painter().line_segment([
            Pos2{y: (i as f32 * spacing) as f32, x: margin} + offset.to_vec2(),
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
        let Self {components} = self;
        
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
            draw_grid(ui, egui::Stroke{width: 1.0, color: egui::Color32::from_gray(60)});
            // ui.painter().line_segment([margin, (size - margin).to_pos2()], grid_stroke);
            // ui_connected_windows(ui, ctx, components);
            for component in components{
                // component.create_window(ctx);
                component.create_globdule(ctx, ui);
                for child_component in &mut component.components{
                    child_component.create_child_window(ctx, ui);

                    // Create connection
                    let line_stroke = egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE};
                    ui.painter().line_segment(
                        [
                            component.pos + component.size / 2.0,
                            child_component.pos + component.size / 2.0],
                        line_stroke
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
