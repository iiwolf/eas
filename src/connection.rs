use egui::{Pos2};

#[derive(serde::Deserialize, serde::Serialize, Debug, Default)]
pub struct Connection{
    // to: Option<&mut Component>,
    // from: Option<&mut Component>,
    pub p1: Pos2,
    pub p2: Pos2,
    pub connecting: bool,
}
