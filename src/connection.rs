use egui::{Pos2};
pub const CONNECTION_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE};

// #[derive(serde::Deserialize, serde::Serialize, Debug, Default, Clone, Copy)]
pub struct Connection{
    // to: Option<&mut Component>,
    // from: Option<&mut Component>,
    pub p1: Pos2,
    pub p2: Pos2,
    pub connecting: bool,
}
