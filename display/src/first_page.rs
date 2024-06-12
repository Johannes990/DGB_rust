use macroquad::prelude::*;
use crate::elements::button::Button;


pub async fn show_page() -> Option<u8> {
    let scr_h = screen_height();
    let scr_w = screen_width();
    let button_w = scr_w / 5.0;
    let button_h = scr_h / 5.0;

    let back_button = Button::new(scr_w - button_w, 0.0, button_w, button_h, BLACK, "BACK TO START", YELLOW, 15.0);

    clear_background(WHITE);
    back_button.draw();

    if back_button.is_pressed() {
        return Some(1);
    }

    None
}
