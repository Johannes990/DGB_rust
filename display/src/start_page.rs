use macroquad::prelude::*;
use crate::elements::button::Button;


pub async fn show_page() -> Option<u8> {
    let scr_h = screen_height();
    let scr_w = screen_width();
    let button_w = scr_w / 2.0;
    let button_w_pad = scr_w / 4.0;
    let button_h = 100.0;
    let button_h_pad = (scr_h - 3.0 * button_h) / 2.0;

    let page_1_button = Button::new(button_w_pad, button_h_pad, button_w, button_h, MAGENTA, "PAGE 1", BLACK, 20.0);
    let page_2_button = Button::new(button_w_pad, button_h_pad + 2.0 * button_h, button_w, button_h, MAGENTA, "PAGE 2", BLACK, 20.0);

    clear_background(WHITE);

    page_1_button.draw();
    page_2_button.draw();

    if page_1_button.is_pressed() {
        return Some(1);
    }

    if page_2_button.is_pressed() {
        return Some(2);
    }

    None
}