use macroquad::prelude::*;
use crate::elements::button::Button;
use crate::elements::palette;

pub async fn show_page<'a>(button_text_params: &'a TextParams<'a>) -> Option<u8> {
    let scr_h = screen_height();
    let scr_w = screen_width();
    let button_w = scr_w / 4.0;
    let button_h = 60.0;
    let button_w_pad = scr_w - (2.0 * button_w);

    let back_button = Button::new(button_w_pad, 0.0, button_w, button_h, palette::BACK_PAGE_BUTTON, "BACK", button_text_params);
    let quit_button = Button::new(button_w_pad + button_w, 0.0, button_w, button_h, palette::QUIT_DIALOG_BUTTON, "QUIT", button_text_params);

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
