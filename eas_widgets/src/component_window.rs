use egui::{Pos2, Vec2, Ui, Rect};
use egui_extras::RetainedImage;

const MINIMIZED_COMPONENT_SIZE: Vec2 = Vec2{x: 150.0, y: 125.0};
const EXPANDED_COMPONENT_SIZE: Vec2 = Vec2{x: 400.0, y: 350.0};
const DEFAULT_ICON_SIZE: Vec2 = Vec2{x: 32.0, y: 32.0};
const HIGHLIGHT_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::LIGHT_BLUE};
const DEFAULT_STROKE: egui::Stroke = egui::Stroke{width: 1.0, color: egui::Color32::DARK_GRAY};

pub struct ComponentWindow {
    pub pos: Pos2,
    pub size: Vec2,
    pub highlight_rec: egui::Rect,
    pub expanded: bool,
    maximize_image: RetainedImage,
    minimize_image: RetainedImage,
    run_image: RetainedImage,
}

pub fn window_ui(ui: &mut egui::Ui) -> egui::Response {
    // Widget code can be broken up in four steps:
    //  1. Decide a size for the widget
    //  2. Allocate space for it
    //  3. Handle interactions with the widget (if any)
    //  4. Paint the widget

    // 2. Allocating space:
    // This is where we get a region of the screen assigned.
    // We also tell the Ui to sense clicks in the allocated region.
    let (rect, mut response): (Rect, egui::Response) = ui.allocate_exact_size(MINIMIZED_COMPONENT_SIZE, egui::Sense::click());

    // 4. Paint!
    // Make sure we need to paint:
    if ui.is_rect_visible(rect) {

        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let radius = 5.0;
        let border = Vec2{x:10.0, y:10.0};

        // Outer rectangle
        ui.painter()
            .rect(rect, radius, egui::Color32::default(), DEFAULT_STROKE);
        
        let inner_rect = egui::Rect{
            min: rect.min + border,
            max: rect.max - border,
        };

        // Inner rectangle
        ui.painter()
            .rect(inner_rect, radius, egui::Color32::DARK_GRAY, egui::Stroke::default());
        
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(window(&mut my_bool))`
// iOS-style window switch.
// pub fn window_() -> impl egui::Widget + '_ {
//     move |ui: &mut egui::Ui| window_ui(ui)
// }

