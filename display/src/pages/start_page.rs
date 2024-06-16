use macroquad::prelude::*;
use crate::values_and_constants::context_button_display_params::ContextButtonDisplayParams;
use crate::widgets::button::Button;
use crate::values_and_constants::palette;

pub async fn show_page<'a>(button_display_params: &ContextButtonDisplayParams, button_text_params: &'a TextParams<'a>) -> Option<u8> {
    let page_1_button = Button::new(button_display_params.start_x,
                                    button_display_params.start_page_1_y,
                                    button_display_params.context_button_width,
                                    button_display_params.context_button_height,
                                    palette::START_PAGE_BUTTON,
                                    "PAGE 1",
                                    button_text_params);
    let options_page_button = Button::new(button_display_params.start_x,
                                          button_display_params.start_options_y,
                                          button_display_params.context_button_width,
                                          button_display_params.context_button_height,
                                          palette::START_PAGE_BUTTON,
                                          "OPTIONS",
                                          button_text_params);
    let quit_button = Button::new(button_display_params.context_quit_x,
                                  button_display_params.context_quit_y,
                                  button_display_params.context_button_width,
                                  button_display_params.context_button_height,
                                  palette::QUIT_DIALOG_BUTTON,
                                  "QUIT",
                                  button_text_params);

    clear_background(palette::GENERAL_BACKGROUND);

    page_1_button.draw();
    options_page_button.draw();
    quit_button.draw();

    if page_1_button.is_pressed() {
        return Some(1);
    }

    if options_page_button.is_pressed() {
        return Some(2);
    }

    if quit_button.is_pressed() {
        return Some(255)
    }

    None
}