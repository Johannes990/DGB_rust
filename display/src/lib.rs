use macroquad::prelude::*;


pub async fn initialize() {
    let y_half = screen_height() / 2.0;

    clear_background(LIGHTGRAY);
    draw_line(0.0, y_half, screen_width(), y_half, 1.0, BLACK);
}

pub async fn draw_circles_from_vec_values(positions: &Vec<f32>) {
    for x_val in positions {
        draw_circle(*x_val, 250.0, 7.0, BLACK);
    }
}
