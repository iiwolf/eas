use egui::{Vec2, Pos2};
use crate::grid::draw_grid;
use crate::component_window::ComponentWindow;
use crate::connection::{Connection, CONNECTION_STROKE};
use eas::component::{Component, Value};
use eas::toolchain::Toolchain;
use std::collections::HashMap;

const N_MAX_WINDOWS: i32 = 1000;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    components: Vec<ComponentWindow>,
    line_state: f32,
    active_connection: Option<Connection>
}

impl Default for TemplateApp {
    fn default() -> Self {

        let mut c1 = Component{
            name: "square".to_string(),
            eval_expression: String::from("x = y ^ 2"), 
            required_input: HashMap::from([
                ("y".to_string(), Value::Float(3.0))
            ]),
            required_output: HashMap::from([
                ("x".to_string(), Value::Float(0.0)),
            ])
        };

        let mut c2 = Component{
            name: "addition".to_string(),
            eval_expression: String::from("z = x + 100.0 / (a0 + a1)"), 
            required_input: HashMap::from([
                ("x".to_string(), Value::Float(9.0)),
                ("a".to_string(), Value::Vectorf32(vec![5.0, 5.0])),
            ]),
            required_output: HashMap::from([
                ("z".to_string(), Value::Float(0.0)),
            ])
            
        };
        // Trying to decide how I want the toolchain-component-component-window relationship 
        //  to work... ultimately I know it's best to separate business and UI logic.
        //  But what's the cleanest way to map a component *or* toolchain click to the 
        //  actual process? In C++ this would be pointers everywhere. I'm thinking right now
        //  just a hash/process ID. That seems clunky to me for some reason...
        //  Also this should be runable without the GUI at all. Through Rhai, Rust, Python, or 
        //  otherwise... So I guess just nail down the processes first?
        let mut tc = Toolchain{components: vec![c1, c2]};
        let mut cw1 = ComponentWindow{component: c1, pos: Pos2{x: 400.0, y:400.0}};
        let mut cw2 = ComponentWindow{component: c2, pos: Pos2{x: 400.0, y:800.0}};
        Self {
            components: vec![cw1, cw2],
            line_state: 0.0,
            active_connection: None,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    // fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //     eframe::set_value(storage, eframe::APP_KEY, self);
    // }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {components, line_state, active_connection} = self;
        
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
            for component in components {
                
                // // Force component to be in bounds
                // component.pos = component.pos.max(ui.min_rect().left_top());
                // component.create_window(ctx, ui, active_connection);
                
                
                // // If new component connected
                // if component.active_connection.is_some() {

                // }

                // // Create connection
                // ui.painter().line_segment(
                //     [
                //         component.pos + COMPONENT_SIZE / 2.0,
                //         child_component.pos + COMPONENT_SIZE / 2.0],
                //     CONNECTION_STROKE
                // );   


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
