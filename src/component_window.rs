use egui::{Pos2, Vec2, Color32, Ui, Stroke, TextEdit, NumExt};
use crate::component::Component;
use crate::connection::{Connection, CONNECTION_STROKE};

const MINIMIZED_COMPONENT_SIZE: Vec2 = Vec2{x: 150.0, y: 125.0};
const EXPANDED_COMPONENT_SIZE: Vec2 = Vec2{x: 400.0, y: 350.0};

pub struct ComponentWindow {
    pub pos: Pos2,
    pub size: Vec2,
    pub highlight_rec: egui::Rect,
    pub expanded: bool,
    texture: Option<egui::TextureHandle>
}

impl ComponentWindow {

    // pub fn name(&self) -> String { self.component.name.clone() }

    pub fn new(pos: Pos2) -> Self {
        let maximize_texture: &egui::TextureHandle = self.maximize_texture.get_or_insert_with(|| {
            // Load the texture only once.
            ui.ctx().load_texture(
                "expanded",
                egui::ColorImage::example(),
                Default::default()
            )
        });

        let maximize_texture: &egui::TextureHandle = self.maximize_texture.get_or_insert_with(|| {
            // Load the texture only once.
        });

        ComponentWindow { 
            pos: pos, 
            size: MINIMIZED_COMPONENT_SIZE.clone(),
            highlight_rec: egui::Rect { 
                min: Pos2{x:0.0, y:0.0}, 
                max: Pos2{x:10.0, y:10.0} 
            },
            expanded: false,
            maximize_texture: ui.ctx().load_texture(
                "maximize",
                egui::ColorImage::example(),
                Default::default()
            ),
            minimize_texture: ui.ctx().load_texture(
                "minimize",
                egui::ColorImage::example(),
                Default::default()
            ),
        } 
    }

    pub fn center(&self) -> Pos2 {
        Pos2{
            x: self.pos.x + self.size.x / 2.0,
            y: self.pos.y + self.size.y / 2.0,
        }
    }

    pub fn create_window(&mut self, ctx: &egui::Context, component: &Component) {
        egui::Window::new(component.name.to_string())
            .title_bar(false)
            .fixed_size(if self.expanded { EXPANDED_COMPONENT_SIZE } else { MINIMIZED_COMPONENT_SIZE } )
            .default_pos(self.pos)
            .current_pos(self.pos)
            .show(ctx, |ui| {
                
                let maximize_texture: &egui::TextureHandle = self.maximize_texture.get_or_insert_with(|| {
                    // Load the texture only once.
                    ui.ctx().load_texture(
                        "expanded",
                        egui::ColorImage::example(),
                        Default::default()
                    )
                });

                // Load exapnded button texture
                // let texture: &egui::TextureHandle = ui.ctx().load_texture("icon-256.png", egui::ColorImage::example(), Default::default());

                let img_size = 16.0 * texture.size_vec2() / texture.size_vec2().y;

                if ui.add(egui::ImageButton::new(texture, img_size)).clicked() {
                    self.expanded = !self.expanded;
                }

                // Drag
                let response = ui.allocate_response(ui.available_size(), egui::Sense::click_and_drag());
                if response.dragged() {
                    self.pos = self.pos + response.drag_delta();
                }
                ui.text_edit_singleline(&mut component.name.to_string());
                
                // Set hightlight rectangle
                let rect = ui.min_rect();
                let mut min = rect.right_top();
                min.x -= 20.0;
                let mut max = rect.right_bottom();
                max.x += 20.0;

                self.highlight_rec = egui::Rect{min:min, max:max};

            });
    }
}