use macroquad::prelude::*;
pub async fn show_page() -> Option<u8> {
    let scr_h = screen_height();
    let scr_w = screen_width();
    let button_w = scr_w / 2.0;
    let button_w_pad = scr_w / 4.0;
    let button_h = 100.0;
    let button_h_pad = (scr_h - 3.0 * button_h) / 2.0;
    let button_1_x = button_w_pad;
    let button_1_y = button_h_pad;
    let button_2_x = button_w_pad;
    let button_2_y = button_1_y + 2.0 * button_h;
    let mouse_x = mouse_position().0;
    let mouse_y = mouse_position().1;

    clear_background(WHITE);

    draw_rectangle(button_1_x, button_1_y, button_w, button_h, LIME);
    draw_rectangle(button_2_x, button_2_y, button_w, button_h, YELLOW);
    draw_text("BUTTON 1", button_w, button_1_y + 35.0, 25.0, BLACK);
    draw_text("BUTTON 2", button_w, button_2_y + 35.0, 25.0, BLACK);

    if is_mouse_button_pressed(MouseButton::Left) && (mouse_x >= button_1_x && mouse_x < button_1_x + button_w)
        && (mouse_y >= button_1_y && mouse_y < button_1_y + button_h) {
        return Some(1);
    }

    if is_mouse_button_pressed(MouseButton::Left) && (mouse_x >= button_2_x && mouse_x < button_2_x + button_w)
        && (mouse_y >= button_2_y && mouse_y < button_2_y + button_h) {
        return Some(2);
    }

    None
}