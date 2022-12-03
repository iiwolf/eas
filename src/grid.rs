const GAP_SIZE: f32 = 10.0;
use egui::{Vec2, Pos2, Color32, Ui, Stroke};
use rand::prelude::*;

pub fn draw_grid(ui: &mut Ui, stroke: Stroke, line_state: &mut f32) {
                
    // Create grid lines because it's COOL and we're in the FUTURE
    let height = ui.available_height();
    let width = ui.available_width();
    let min_dim = height.min(width);
    let spacing = min_dim * 0.05;
    let margin = min_dim * 0.02;
    let n_vertical_lines = (width / spacing).round() as i32;  
    let n_horizontal_lines = (height / spacing).round() as i32;  
    let offset = ui.min_rect().left_top();
    
    // Vertical lines
    for i in 1..n_vertical_lines {
        ui.painter().line_segment([
            Pos2{x: (i as f32 * spacing) as f32, y: margin} + offset.to_vec2(),
            Pos2{x: (i as f32 * spacing) as f32, y: height - margin} + offset.to_vec2()
        ],
        stroke);
    }

    let cur_time = ui.input().time as f32;
    let do_flicker = rand::random::<f32>() < 0.05 && (cur_time - *line_state) > 5.0;

    // Horizontal lines
    for i in 1..n_horizontal_lines {

        if  do_flicker { 
            // Segment 1
            let mut w1 = 0.0;
            let step_size = width / 100.0;
            for i in 0..100 {

                if rand::random::<f32>() < 0.5 { 
                    ui.painter().line_segment([
                        Pos2{y: (i as f32 * spacing) as f32, x: w1} + offset.to_vec2(),
                        Pos2{y: (i as f32 * spacing) as f32, x: w1 + (i as f32) * step_size} + offset.to_vec2()
                    ],
                    stroke);
                }
                w1 += (i as f32) * step_size;
            }

        }else{
            ui.painter().line_segment([
                Pos2{y: (i as f32 * spacing) as f32, x: margin} + offset.to_vec2(),
                Pos2{y: (i as f32 * spacing) as f32, x: width - margin} + offset.to_vec2()
            ],
            stroke);
        }

    }
    
    if do_flicker {
        *line_state = cur_time;
    }

}
