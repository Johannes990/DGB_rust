use macroquad::prelude::*;
use crate::app_state::AppState;
use crate::values_and_constants::context_button_display_params::ContextButtonDisplayParams;
use crate::widgets::button::Button;
use crate::values_and_constants::palette;

pub async fn show_page<'a>(app_state: &mut AppState<'a>, button_display_params: &ContextButtonDisplayParams, button_text_params: &'a TextParams<'a>) -> Option<u8> {
    let quit_yes_button = Button::new(button_display_params.quit_yes_x,
                                      button_display_params.quit_yes_y,
                                      button_display_params.context_button_width,
                                      button_display_params.context_button_height,
                                      palette::QUIT_YES_BUTTON,
                                      "YES",
                                      button_text_params);
    let quit_no_button = Button::new(button_display_params.quit_no_x,
                                     button_display_params.quit_no_y,
                                     button_display_params.context_button_width,
                                     button_display_params.context_button_height,
                                     palette::QUIT_NO_BUTTON,
                                     "NO",
                                     button_text_params);

    clear_background(palette::QUIT_DIALOG_BACKGROUND);

    quit_yes_button.draw();
    quit_no_button.draw();

    if quit_yes_button.is_pressed() {
        app_state.user_exit_requested = true;
        return Some(0);
    }

    if quit_no_button.is_pressed() {
        return Some(254);
    }

    None
}