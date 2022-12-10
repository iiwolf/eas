use egui::{Pos2};

use crate::component_window::ComponentWindow;
pub const CONNECTION_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE};

// #[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone, Copy)]
#[derive(Default)]
pub struct Connection{
    pub tc_from: Option<usize>,
    pub c_from: Option<usize>,
    pub tc_to: Option<usize>,
    pub c_to: Option<usize>,
    pub windex: Option<usize>
}

impl Connection {
    pub fn reset(&mut self) {
        self.tc_from = None;
        self.c_from = None;
        self.tc_to = None;
        self.c_to = None;
        self.windex = None;
    }
    // pub fn new(p1: Pos2, p2: Pos2) -> Self {}
}
