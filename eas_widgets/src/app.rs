use crate::component_window::{window_ui, ComponentWindow};
use crate::toggle_switch::toggle;
use std::collections::HashMap;

const N_MAX_WINDOWS: i32 = 1000;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
// #[derive(serde::Deserialize, serde::Serialize)]
// #[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct WidgetTestApp {
    toggled: bool
}

impl Default for WidgetTestApp {
    fn default() -> Self {

        Self {
            toggled: false
        }
    }
}

impl WidgetTestApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for WidgetTestApp {

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            toggled
        }: &mut WidgetTestApp = self;

        egui::CentralPanel::default().show(ctx, |ui| {

            ui.add(toggle(toggled));
            window_ui(ui);
        });
    }
}
