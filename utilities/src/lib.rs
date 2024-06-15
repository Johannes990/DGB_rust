use std::time::{SystemTime, UNIX_EPOCH};
use macroquad::input::{is_mouse_button_pressed, MouseButton};
use macroquad::prelude::mouse_position;

pub fn generate_seed() -> [u64; 2] {
    let start = SystemTime::now();
    let since_epoch = start.duration_since(UNIX_EPOCH).expect("Time error");
    let nanos = since_epoch.as_nanos();

    [(nanos & 0xFFFFFFFFFFFFFFFF) as u64, (nanos >> 64) as u64]
}

pub fn shift_f32_to_range(
    sample: f32,
    initial_range_start: u32,
    initial_range_end: u32,
    final_range_start: u32,
    final_range_end: u32,
) -> Result<f32, String> {
    if initial_range_start >= initial_range_end {
        return Err("Initial range start value must be lower than initial range end value.".to_string())
    }

    let initial_range_start = initial_range_start as f32;
    let initial_range_end = initial_range_end as f32;
    let final_range_start = final_range_start as f32;
    let final_range_end = final_range_end as f32;

    if sample < initial_range_start {
        return Ok(final_range_start);
    } else if sample > initial_range_end {
        return Ok(final_range_end);
    }

    let initial_range = initial_range_end - initial_range_start;
    let final_range = final_range_end - final_range_start;
    let rel_pos = (sample - initial_range_start) / initial_range;
    let shifted_value = final_range_start + rel_pos * final_range;
    println!("Shifted {} to a new value of {}", sample, shifted_value);

    Ok(shifted_value)
}

pub fn left_mouse_click_in_area(x_start: f32, y_start: f32, x_end: f32, y_end: f32) -> bool {
    mouse_click_in_area(MouseButton::Left, x_start, y_start, x_end, y_end)
}

pub fn right_mouse_click_in_area(x_start: f32, y_start: f32, x_end: f32, y_end: f32) -> bool {
    mouse_click_in_area(MouseButton::Right, x_start, y_start, x_end, y_end)
}

fn mouse_click_in_area(button: MouseButton, x_start: f32, y_start: f32, x_end: f32, y_end: f32) -> bool {
    let clicked = is_mouse_button_pressed(button);
    let (mouse_x, mouse_y) = mouse_position();

    clicked && mouse_x >= x_start && mouse_y >= y_start && mouse_x <= x_end && mouse_y <= y_end
}