use macroquad::prelude::*;
use crate::elements::button::Button;
use crate::elements::palette;

pub async fn show_page(font: &Font) -> Option<u8> {
    let scr_h = screen_height();
    let scr_w = screen_width();
    let button_w = scr_w / 4.0;
    let button_h = 60.0;
    let button_w_pad = scr_w - (2.0 * button_w);
    let text_params = TextParams {
        font: Some(font),
        font_size: 27,
        color: BLACK,
        ..Default::default()
    };

    let back_button = Button::new(button_w_pad, 0.0, button_w, button_h, palette::BACK_PAGE_BUTTON, "BACK TO PREV", BLACK, 20.0);
    let quit_button = Button::new(button_w_pad + button_w, 0.0, button_w, button_h, palette::QUIT_DIALOG_BUTTON, "QUIT", BLACK, 20.0);

    clear_background(palette::GENERAL_BACKGROUND);

    back_button.draw();
    quit_button.draw();

    draw_text_ex(
        "Johannes Jürgenson",
        100.0,
        100.0,
        text_params.clone()
    );

    draw_text_ex(
        "13. juuli 2024",
        100.0,
        150.0,
        text_params.clone()
    );

    if back_button.is_pressed() {
        return Some(1);
    }

    if quit_button.is_pressed() {
        return Some(255);
    }

    None
}
