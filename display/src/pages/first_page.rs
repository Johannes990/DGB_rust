use macroquad::prelude::*;
use crate::values_and_constants::context_button_display_params::ContextButtonDisplayParams;
use crate::widgets::button::Button;
use crate::values_and_constants::palette;

pub async fn show_page<'a>(button_display_params: &ContextButtonDisplayParams, button_text_params: &'a TextParams<'a>) -> Option<u8> {
    let back_button = Button::new(button_display_params.context_back_x,
                                  button_display_params.context_back_y,
                                  button_display_params.context_button_width,
                                  button_display_params.context_button_height,
                                  palette::BACK_PAGE_BUTTON,
                                  "BACK",
                                  button_text_params);
    let quit_button = Button::new(button_display_params.context_quit_x,
                                  button_display_params.context_quit_y,
                                  button_display_params.context_button_width,
                                  button_display_params.context_button_height,
                                  palette::QUIT_DIALOG_BUTTON,
                                  "QUIT",
                                  button_text_params);

    clear_background(palette::GENERAL_BACKGROUND);

    back_button.draw();
    quit_button.draw();

    if back_button.is_pressed() {
        return Some(1);
    }

    if quit_button.is_pressed() {
        return Some(255);
    }

    None
}
