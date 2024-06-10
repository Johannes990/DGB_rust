use macroquad::color::{BLACK, LIGHTGRAY};
use macroquad::shapes::{draw_line, draw_circle};
use macroquad::window::{clear_background, next_frame, screen_height, screen_width};
use display;


fn window_conf() -> macroquad::prelude::Conf {
    macroquad::prelude::Conf {
        window_title: "Dynamic Graph Builder".to_owned(),
        window_height: 500,
        window_width: 500,
        high_dpi: false,
        window_resizable: true,
        fullscreen: false,
        icon: None,
        sample_count: 0,
        platform: Default::default(),
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let x_half = screen_width() / 2.0;
    let y_half = screen_height() / 2.0;

    loop {
        clear_background(LIGHTGRAY);

        draw_line(0.0, y_half, screen_width(), y_half, 1.0, BLACK);
        draw_circle(x_half, y_half, 7.0, BLACK);

        next_frame().await
    }
}
