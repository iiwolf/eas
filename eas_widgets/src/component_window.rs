use egui::{Pos2, Vec2, Ui};
use egui_extras::RetainedImage;

const MINIMIZED_COMPONENT_SIZE: Vec2 = Vec2{x: 150.0, y: 125.0};
const EXPANDED_COMPONENT_SIZE: Vec2 = Vec2{x: 400.0, y: 350.0};
const DEFAULT_ICON_SIZE: Vec2 = Vec2{x: 32.0, y: 32.0};

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
    let (rect, mut response) = ui.allocate_exact_size(MINIMIZED_COMPONENT_SIZE, egui::Sense::click());

    // 4. Paint!
    // Make sure we need to paint:
    if ui.is_rect_visible(rect) {

        // We will follow the current style by asking
        // "how should something that is being interacted with be painted?".
        // This will, for instance, give us different colors when the widget is hovered or clicked.
        let visuals = ui.style().interact_selectable(&response);

        // All coordinates are in absolute screen coordinates so we use `rect` to place the elements.
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(window(&mut my_bool))`
/// iOS-style window switch.
// pub fn window() -> impl egui::Widget + '_ {
//     move |ui: &mut egui::Ui| window_ui(ui)
// }

pub fn url_to_file_source_code() -> String {
    format!("https://github.com/emilk/egui/blob/master/{}", file!())
}