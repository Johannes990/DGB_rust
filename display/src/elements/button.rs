use macroquad::prelude::*;

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub background_color: Color,
    pub label: String,
    pub text_color: Color,
    pub font_size: f32
}

impl Button {
    pub fn new(x: f32, y: f32, width: f32, height: f32, background_color: Color, label: &str, text_color: Color, font_size: f32) -> Self {
        Self {
            x,
            y,
            width,
            height,
            background_color,
            text_color,
            label: label.to_string(),
            font_size,
        }
    }

    pub fn draw(&self) {
        let text_dims = measure_text(&self.label, None, self.font_size as u16, 1.0);
        let text_x = self.x +(self.width / 2.0) - (text_dims.width / 2.0);
        let text_y = self.y + (self.height / 2.0) + (text_dims.height / 2.0);

        draw_rectangle(self.x, self.y, self.width, self.height, self.background_color);
        draw_text(&self.label, text_x, text_y, self.font_size, self.text_color);
    }

    pub fn is_pressed(&self) -> bool {
        let (mouse_x, mouse_y) = mouse_position();

        is_mouse_button_pressed(MouseButton::Left)
            && mouse_x >= self.x
            && mouse_x < self.x + self.width
            && mouse_y >= self.y
            && mouse_y < self.y + self.height
    }
}
