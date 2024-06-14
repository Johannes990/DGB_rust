use macroquad::prelude::*;
use crate::elements::button::Button;
use crate::elements::palette;

pub async fn show_page<'a>(text_params: &TextParams<'a>, mut user_exit_requested: &bool) -> Option<u8> {
    let scr_h = screen_height();
    let scr_w = screen_width();
    let button_w = scr_w / 4.0;
    let button_w_pad = (scr_w / 2.0) - 1.5 * button_w;
    let button_h = 60.0;
    let button_h_pad = (scr_h / 2.0) - 0.5 * button_h;

    let quit_yes_button = Button::new(button_w_pad, button_h_pad, button_w, button_h, palette::QUIT_YES_BUTTON, "YES", text_params);
    let quit_no_button = Button::new(button_w_pad + 2.0 * button_w, button_h_pad, button_w, button_h, palette::QUIT_NO_BUTTON, "NO", text_params);

    clear_background(palette::QUIT_DIALOG_BACKGROUND);

    quit_yes_button.draw();
    quit_no_button.draw();

    if quit_yes_button.is_pressed() {
        user_exit_requested = &true;
        return Some(0);
    }

    if quit_no_button.is_pressed() {
        return Some(254);
    }

    None
}