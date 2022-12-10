use egui::{Pos2};

use crate::component_window::ComponentWindow;
pub const CONNECTION_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE};

// #[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone, Copy)]
pub struct Connection{
    pub p1: Pos2,
    pub p2: Pos2,
    pub connecting: bool,
    pub from_ref: Option<String>,
    pub to_ref: Option<String>,
}

impl Connection {
    pub fn new(p1: Pos2, p2: Pos2) -> Self {
        Connection { p1: p1, p2: p2, connecting: true, from_ref: None, to_ref: None }
    }
}
