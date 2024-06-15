use macroquad::math::{bool, f32};
use macroquad::prelude::*;
use crate::elements::button::Button;
use crate::elements::palette;
use settings::distribution_settings::DistributionSettings;
use distributions::distribution_class::{DistributionClass, HasName};
use crate::elements::slider::Slider;
use crate::elements::selection_list::SelectionList;

pub async fn show_page<'a>(button_text_params: &TextParams<'a>,
                           settings_text_params: &TextParams<'a>,
                           settings_state: &mut DistributionSettings,
                           slider_x1: &mut Slider<'a>,
                           slider_y1: &mut Slider<'a>,
                           slider_x2: &mut Slider<'a>,
                           slider_y2: &mut Slider<'a>,
                           slider_x3: &mut Slider<'a>,
                           slider_y3: &mut Slider<'a>,
                           slider_x4: &mut Slider<'a>,
                           slider_y4: &mut Slider<'a>,
                           checklist: &mut SelectionList<'a, DistributionClass>) -> Option<u8> {
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

    let selected_dist_class_option = checklist.selected.clone();

    if let Some(selected_dist_class) = selected_dist_class_option {
        draw_text_ex(selected_dist_class.get_name(), 180.0, 90.0, settings_text_params.clone());
    }

    slider_x1.draw();
    slider_y1.draw();
    slider_x2.draw();
    slider_y2.draw();
    slider_x3.draw();
    slider_y3.draw();
    slider_x4.draw();
    slider_y4.draw();
    checklist.draw();
    slider_x1.handle_input();
    slider_y1.handle_input();
    slider_x2.handle_input();
    slider_y2.handle_input();
    slider_x3.handle_input();
    slider_y3.handle_input();
    slider_x4.handle_input();
    slider_y4.handle_input();
    checklist.handle_input();

    if back_button.is_pressed() {
        return Some(1);
    }

    if quit_button.is_pressed() {
        return Some(255);
    }

    None
}

fn draw_checkbox<'a>(selected: &bool, label: &String, axis: &str, x: f32, y: f32, text_params: TextParams) -> bool {
    let size = 20.0;
    let (mouse_x, mouse_y) = mouse_position();
    let clicked = is_mouse_button_pressed(MouseButton::Left);

    draw_text_ex(&*label, x + size + 10.0, y + size - 5.0, text_params.clone());
    draw_rectangle(x, y, size, size, WHITE);
    if *selected {
        draw_rectangle(x + 3.0, y + 3.0, size - 6.0, size - 6.0, RED);
    }

    if clicked && mouse_x >= x && mouse_x <= x + size && mouse_y >= y && mouse_y <= y + size {
        println!("{}-axis distribution selected: {}", axis, label);
        println!("selected was {}", selected.to_string());
        selected != selected;
        println!("selected now is {}", selected.to_string());
    }

    *selected
}