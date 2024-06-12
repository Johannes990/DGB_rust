use macroquad::prelude::*;


pub async fn show_page() -> Option<u8> {
    let scr_h = screen_height();
    let scr_w = screen_width();
    let button_w = scr_w / 5.0;
    let button_w_pad = scr_w / 10.0;
    let button_h = scr_h / 5.0;
    let button_h_pad = scr_h / 10.0;
    let button_1_x = scr_w - button_w_pad - 1.0 * button_w;
    let button_1_y = scr_h - button_h_pad - 4.0 * button_h;

    let mouse_x = mouse_position().0;
    let mouse_y = mouse_position().1;

    clear_background(WHITE);

    draw_rectangle(button_1_x, button_1_y, button_w, button_h, BLACK);
    draw_text("BACK", button_1_x, button_1_y + 35.0, 25.0, WHITE);

    if is_mouse_button_pressed(MouseButton::Left) && (mouse_x >= button_1_x && mouse_x < button_1_x + button_w)
        && (mouse_y >= button_1_y && mouse_y < button_1_y + button_h) {
        return Some(1);
    }

    None
}
