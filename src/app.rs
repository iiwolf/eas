use egui::{Vec2, Pos2, Color32};

#[derive(serde::Deserialize, serde::Serialize, Default, Debug)]
pub struct Component{
    name: String,
    pos: Pos2,
    size: Vec2,
    components: Vec<Component>
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

            // ui_connected_windows(ui, ctx, components);
            for component in components{
                egui::Window::new(component.name.to_string())
                    .current_pos(component.pos)
                    .fixed_size(component.size)
                    .default_pos(component.pos)
                    .show(ctx, |ui| {
                        
                        let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                        
                        if response.dragged() {
                            component.pos = component.pos + response.drag_delta();
                        }
                        
                        ui.label(component.name.to_string());
                        
                });
                
                for reference_component in &mut component.components{

                    // Create component
                    egui::Window::new(reference_component.name.to_string())
                        .current_pos(reference_component.pos)
                        .default_size(reference_component.size)
                        .show(ctx, |ui| {
                        ui.label(reference_component.name.to_string());
                    });

                    // Create connection
                    let line_stroke = egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE};
                    ui.painter().line_segment(
                        [
                            component.pos + component.size / 2.0,
                            reference_component.pos + component.size / 2.0],
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
