use macroquad::math::{bool, f32};
use macroquad::prelude::*;
use crate::elements::button::Button;
use crate::elements::palette;
use settings::distribution_settings::{DistributionSettings, DistributionType};
use crate::elements::slider::Slider;

pub async fn show_page(settings_state: &mut DistributionSettings, slider_x: &mut Slider, slider_y: &mut Slider) -> Option<u8> {
    let scr_h = screen_height();
    let scr_w = screen_width();
    let button_w = scr_w / 4.0;
    let button_h = 60.0;
    let button_w_pad = scr_w - (2.0 * button_w);

    let back_button = Button::new(button_w_pad, 0.0, button_w, button_h, palette::BACK_PAGE_BUTTON, "BACK TO MENU", BLACK, 25.0);
    let quit_button = Button::new(button_w_pad + button_w, 0.0, button_w, button_h, palette::QUIT_DIALOG_BUTTON, "QUIT", BLACK, 25.0);

    clear_background(palette::GENERAL_BACKGROUND);

    back_button.draw();
    quit_button.draw();

    // settings menu display here
    draw_text("X-axis", 20.0, 90.0, 30.0, WHITE);
    let mut y1 = 110.0;
    let distributions = [
        (DistributionType::Gaussian, "Gaussian"),
        (DistributionType::Beta, "Beta"),
        (DistributionType::Uniform, "Uniform"),
        (DistributionType::Exponent, "Exponent"),
        (DistributionType::Gamma, "Gamma"),
        (DistributionType::LogNormal, "LogNormal"),
        (DistributionType::Cauchy, "Cauchy"),
    ];

    for (distribution_type, name) in distributions.iter() {
        let selected = settings_state.x_axis_distribution == *distribution_type;
        if draw_checkbox(selected, name, "X", 20.0, y1) {
            settings_state.x_axis_distribution = distribution_type.clone();
        }
        y1 += 30.0;
    }

    y1 = 110.0;
    draw_text("Y-axis", 180.0, 90.0, 30.0, WHITE);
    for (distribution_type, name) in distributions.iter() {
        let selected = settings_state.y_axis_distribution == *distribution_type;
        if draw_checkbox(selected, name, "Y", 180.0, y1) {
            settings_state.y_axis_distribution = distribution_type.clone();
        }
        y1 += 30.0;
    }

    slider_x.draw();
    slider_y.draw();

    slider_x.handle_input();
    slider_y.handle_input();

    if back_button.is_pressed() {
        return Some(1);
    }

    if quit_button.is_pressed() {
        return Some(255);
    }

    None
}

fn draw_checkbox(selected: bool, label: &str, axis: &str, x: f32, y: f32) -> bool {
    let size = 20.0;
    let (mouse_x, mouse_y) = mouse_position();
    let clicked = is_mouse_button_pressed(MouseButton::Left);
    let mut is_selected = selected;

    draw_rectangle(x, y, size, size, WHITE);
    if is_selected {
        draw_rectangle(x + 3.0, y + 3.0, size - 6.0, size - 6.0, RED);
    }

    draw_text(label, x + size + 10.0, y + size - 5.0, 30.0, WHITE);

    if clicked && mouse_x >= x && mouse_x <= x + size && mouse_y >= y && mouse_y <= y + size {
        is_selected = !is_selected;
        println!("{}-axis distribution selected: {}", axis, label);
    }

    is_selected
}