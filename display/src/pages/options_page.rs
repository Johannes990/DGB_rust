use macroquad::prelude::*;
use crate::widgets::button::Button;
use crate::constants_and_values::palette;
use crate::app_state::AppState;

pub async fn show_page<'a>(app_state: &mut AppState<'a>, button_text_params: &'a TextParams<'a>) -> Option<u8> {
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

    &app_state.x_axis_distribution_list.draw();
    &app_state.y_axis_distribution_list.draw();
    &app_state.x_axis_distribution_list.handle_input();
    &app_state.y_axis_distribution_list.handle_input();



    /*
    let selected_dist_class_option = checklist.selected.clone();

    if let Some(selected_dist_class) = selected_dist_class_option {
        draw_text_ex(selected_dist_class.get_name(), 180.0, 90.0, settings_text_params.clone());
    }
     */

    if back_button.is_pressed() {
        return Some(1);
    }

    if quit_button.is_pressed() {
        return Some(255);
    }

    None
}
