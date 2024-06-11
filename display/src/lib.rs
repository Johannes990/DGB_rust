pub mod histogram;
use macroquad::prelude::*;


pub async fn initialize() {
    let y_half = screen_height() / 2.0;
    let x_half = screen_width() / 2.0;

    clear_background(LIGHTGRAY);
    draw_line(0.0, y_half, screen_width(), y_half, 1.0, BLACK);
    draw_line(x_half, 225.0, x_half, 275.0, 1.0, BLACK);
}

pub async fn draw_circles_from_vec_values(positions: &Vec<f32>) {
    for x_val in positions {
        draw_circle(*x_val, 250.0, 3.0, BLACK);
        draw_circle(*x_val, 250.0, 1.0, RED);
    }
}
