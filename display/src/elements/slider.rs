use std::fmt::Debug;
use macroquad::prelude::*;


pub struct Slider<'a> {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub min_value: f32,
    pub max_value: f32,
    pub current_value: f32,
    pub is_dragging: bool,
    pub slider_type: SliderType,
    pub label: String,
    pub text_params: TextParams<'a>
}


pub enum SliderType {
    Vertical,
    Horizontal,
}

impl<'a> Slider<'a> {
    pub fn new(x: f32, y: f32, width: f32, height: f32, min_value: f32, max_value: f32, initial_value: f32, slider_type: SliderType, label: &str, text_params: &TextParams<'a>) -> Self {
        Self { x, y, width, height, min_value, max_value, current_value: initial_value, is_dragging: false, slider_type, label: label.to_string(), text_params: text_params.clone() }
    }

    pub fn draw(&self) {
        match self.slider_type {
            SliderType::Horizontal => {
                //track
                draw_rectangle(self.x, self.y + self.height / 2.0 - 2.0, self.width, 4.0, DARKGRAY);
                // Draw the slider knob
                let knob_x = self.x + (self.current_value - self.min_value) / (self.max_value - self.min_value) * self.width;
                draw_rectangle(knob_x - self.height / 2.0, self.y, self.height, self.height, BLUE);
                draw_text_ex(&self.label, self.x, self.y - self.height / 2.0, self.text_params.clone());
            },
            SliderType::Vertical => {
                todo!()
            }
        }
    }

    pub fn handle_input(&mut self) {
        let (mouse_x, mouse_y) = mouse_position();
        let mouse_rect = Rect::new(mouse_x, mouse_y, 1.0, 1.0);
        let knob_x = self.x + (self.current_value - self.min_value) / (self.max_value - self.min_value) * self.width;
        let knob_rect = Rect::new(knob_x - self.height / 2.0, self.y, self.height, self.height);

        if is_mouse_button_pressed(MouseButton::Left) && mouse_rect.overlaps(&knob_rect) {
            self.is_dragging = true;
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.is_dragging = false;
        }

        if self.is_dragging {
            let new_value = self.min_value + (mouse_x - self.x) / self.width * (self.max_value - self.min_value);
            self.current_value = new_value.clamp(self.min_value, self.max_value);
        }
    }
}