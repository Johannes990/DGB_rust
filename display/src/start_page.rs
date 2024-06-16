use macroquad::prelude::*;
use crate::widgets::button::Button;
use crate::widgets::palette;

pub async fn show_page<'a>(button_text_params: &'a TextParams<'a>) -> Option<u8> {
    let scr_h = screen_height();
    let scr_w = screen_width();
    let button_w = scr_w / 4.0;
    let button_h = 60.0;
    let button_w_pad = 1.5 * scr_w / 4.0;
    let button_h_pad = (scr_h / 2.0) - 2.5 * button_h;

    let page_1_button = Button::new(button_w_pad, button_h_pad, button_w, button_h, palette::START_PAGE_BUTTON, "PAGE 1", button_text_params);
    let options_page_button = Button::new(button_w_pad, button_h_pad + 2.0 * button_h, button_w, button_h, palette::START_PAGE_BUTTON, "OPTIONS", button_text_params);
    let quit_button = Button::new(button_w_pad, button_h_pad + 4.0 * button_h, button_w, button_h, palette::QUIT_DIALOG_BUTTON, "QUIT", button_text_params);

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