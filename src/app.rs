use egui::{Vec2, Pos2};
use crate::grid::draw_grid;
use crate::component_window::ComponentWindow;
use crate::connection::{Connection, CONNECTION_STROKE};
use crate::component::{Component, Value};
use crate::toolchain::Toolchain;
use std::collections::HashMap;

const N_MAX_WINDOWS: i32 = 1000;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    toolchains: Vec<Toolchain>,
    component_windows: Vec<ComponentWindow>,
    tc_from: Option<usize>,
    c_from: Option<usize>,
    tc_to: Option<usize>,
    c_to: Option<usize>,
    line_state: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {

        // TC1 
        let tc1: Toolchain = Toolchain::new(vec![
            Component{
                name: "square".to_string(),
                eval_expression: String::from("x = y ^ 2"), 
                required_input: HashMap::from([
                    ("y".to_string(), Value::Float(3.0))
                ]),
                required_output: HashMap::from([
                    ("x".to_string(), Value::Float(0.0)),
                ])
        }]);
        
        // TC2
        let tc2: Toolchain = Toolchain::new(vec![
            Component{
                name: "addition".to_string(),
                eval_expression: String::from("z = x + 100.0 / (a0 + a1)"), 
                required_input: HashMap::from([
                    ("x".to_string(), Value::Float(9.0)),
                    ("a".to_string(), Value::Vectorf32(vec![5.0, 5.0])),
                ]),
                required_output: HashMap::from([
                    ("z".to_string(), Value::Float(0.0)),
                ])
        }]);
        
        // Trying to decide how I want the toolchain-component-component-window relationship 
        //  to work... ultimately I know it's best to separate business and UI logic.
        //  But what's the cleanest way to map a component *or* toolchain click to the 
        //  actual process? In C++ this would be pointers everywhere. I'm thinking right now
        //  just a hash/process ID. That seems clunky to me for some reason...
        //  Also this should be runable without the GUI at all. Through Rhai, Rust, Python, or 
        //  otherwise... So I guess just nail down the processes first?
        // let mut tc = Toolchain{components: vec![c1, c2]};
        let mut cw1 = ComponentWindow::new(Pos2{x: 400.0, y:400.0});
        let mut cw2 = ComponentWindow::new(Pos2{x: 400.0, y:800.0});
        Self {
            toolchains: vec![tc1, tc2],
            component_windows: vec![cw1, cw2],
            tc_from: None,
            c_from: None,
            tc_to: None,
            c_to: None,
            line_state: 0.0,
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
        let Self {
            toolchains,
            component_windows, 
            tc_from,
            c_from,
            tc_to,
            c_to,
            line_state, 
        }: &mut TemplateApp = self;
        
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui: &mut egui::Ui| {
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
                        let names = Vec::from_iter(
                            toolchains.iter().map(
                                |tc| tc.components.iter().map(
                                    |c| c.name.to_string()
                               ).collect()
                            )
                        );
                        let mut default_name = "Empty".to_string();
                        for counter in 1..N_MAX_WINDOWS{
                            if !names.contains(&default_name){ break; }
                            default_name = format!("Empty_{}", counter);
                        }
                        // components.push(ComponentWindow::new(default_name));
                    }
                }
            );

            for (i, tc) in toolchains.iter().enumerate() {
                for (j, component) in tc.components.iter().enumerate() {
                    let window_index = i * (toolchains.len() - 1) + j;
                    let component_window = &mut component_windows[window_index];

                    // Force component to be in bounds
                    component_window.pos = component_window.pos.max(ui.min_rect().left_top());
                    component_window.create_window(ctx, ui, component);
                    
                    // Adding arrow
                    let edge_response = ui.allocate_rect(component_window.highlight_rec,  egui::Sense::click());
                    
                    // Highlight affect on edge hover
                    if edge_response.hovered(){
                        ui.painter().rect_filled(component_window.highlight_rec, 0.0, egui::Color32::LIGHT_BLUE);
                    }
                    
                    if edge_response.clicked() {

                        // Start arrow
                        if tc_from.is_none() {
                            *tc_from = Some(i);
                            *c_from = Some(j);
                        } 

                        // Finish connection
                        else if tc_from.is_some() {
                            *tc_to = Some(i);
                            *c_to = Some(j);
                        }
                    }

                }
            }

            // toolchains[0].components.insert(active_index, element)
            if tc_from.is_some() && tc_to.is_some() {

                // Take from other toolchain and place into new one
                let component_to_take = toolchains[tc_to.unwrap()].components.swap_remove(c_to.unwrap());
                toolchains[tc_from.unwrap()].components.push(component_to_take);

                // Clear connection
                *tc_from = None;
                *c_from = None;
                *tc_to = None;
                *c_to = None;
            }
            else if tc_from.is_some() {
                // let mut connection = tc_from.as_mut().unwrap();
                let window_index = tc_from.unwrap() * toolchains.len() + c_from.unwrap();
                let p1 = component_windows[window_index].pos;
                let p2 = ctx.pointer_hover_pos().unwrap();
                ui.painter().line_segment([p1, p2], CONNECTION_STROKE);
            }

        });
    }
}
