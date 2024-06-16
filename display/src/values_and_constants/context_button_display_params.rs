use macroquad::prelude::{screen_width, screen_height};

pub struct ContextButtonDisplayParams {
    pub context_button_width: f32,
    pub context_button_height: f32,
    pub context_back_x: f32,
    pub context_back_y: f32,
    pub context_quit_x: f32,
    pub context_quit_y: f32,
    pub quit_yes_x: f32,
    pub quit_yes_y: f32,
    pub quit_no_x: f32,
    pub quit_no_y: f32,
    pub start_x: f32,
    pub start_page_1_y: f32,
    pub start_options_y: f32,
}

impl ContextButtonDisplayParams {
    pub fn new() -> Self {
        let context_button_width = 120.0;
        let context_button_height = 50.0;
        let context_back_x = screen_width() - 2.0 * context_button_width;
        let context_back_y = 0.0;
        let context_quit_x = screen_width() - context_button_width;
        let context_quit_y = 0.0;
        let quit_yes_x = screen_width() * 0.5 - 100.0 - context_button_width;
        let quit_no_x = screen_width() * 0.5 + 100.0;
        let quit_yes_y = (screen_height() - context_button_height) * 0.5;
        let quit_no_y = (screen_height() - context_button_height) * 0.5;
        let start_x = (screen_width() - context_button_width) * 0.5;
        let start_page_1_y = (screen_height() - 2.0 * context_button_height) * 0.5;
        let start_options_y = (screen_height() + context_button_height) * 0.5;

        Self {
            context_button_width,
            context_button_height,
            context_back_x,
            context_back_y,
            context_quit_x,
            context_quit_y,
            quit_yes_x,
            quit_yes_y,
            quit_no_x,
            quit_no_y,
            start_x,
            start_page_1_y,
            start_options_y
        }
    }
}