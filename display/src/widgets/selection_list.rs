use macroquad::math::f32;
use macroquad::prelude::*;
use distributions::distribution_class::{HasName, HasPayload};
use utilities::left_mouse_click_in_area;
use crate::values_and_constants::palette::{SELECTION_LIST_BACKGROUND, SELECTION_LIST_CHECK};

///
/// Custom checklist widget
/// here contents must be of type T where T implements
/// the HasName and HasPayload traits
///
pub struct SelectionList<'a, T: HasPayload + HasName + PartialEq + Clone> {
    pub x: f32,
    pub y: f32,
    pub box_size: f32,
    pub contents: Vec<T>,
    pub selected: T,
    pub heading: String,
    pub text_params: TextParams<'a>,
    y_spacing: f32,
    text_y: f32,
}

impl<'a, T: HasPayload + HasName + PartialEq + Clone> SelectionList<'a, T> {
    pub fn new(x: f32,
               y: f32,
               box_size: f32,
               contents: Vec<T>,
               heading: String,
               text_params: TextParams<'a>) -> Self {
        Self { x, y, box_size, contents: contents.clone(), selected: contents.get(0).unwrap().clone(), heading, text_params, y_spacing: 30.0 , text_y: 0.0 }
    }

    pub fn draw(&mut self) {
        let font_height = measure_text(&*self.heading, self.text_params.font, self.text_params.font_size, self.text_params.font_scale).height;
        self.text_y = self.y + font_height + 3.0;
        draw_text_ex(&self.heading, self.x, self.text_y, self.text_params.clone());

        for (i, content_line) in self.contents.iter().enumerate() {
            let row_spacing = self.y_spacing * (i as f32 + 1.0);
            let checkbox_y = self.y + row_spacing;
            let text_x = self.x + self.box_size + 5.0;
            let text_y_row = self.text_y + row_spacing;
            let selected_pad = 2.0;
            draw_rectangle(self.x, checkbox_y, self.box_size, self.box_size, SELECTION_LIST_BACKGROUND);

            match &self.selected {
                selected if selected.get_name() == content_line.get_name() => {
                    draw_rectangle(self.x + selected_pad,
                                   checkbox_y + selected_pad,
                                   self.box_size - 2.0 * selected_pad,
                                   self.box_size - 2.0 * selected_pad,
                                   SELECTION_LIST_CHECK);
                }
                _ => {
                    //println!("Something is selected but match does not know what!");
                },
            }

            draw_text_ex(content_line.get_name(), text_x, text_y_row, self.text_params.clone());
        }
    }

    pub fn handle_input(&mut self) {
        let y_spacing = 30.0;

        for (i, content_line) in self.contents.iter().enumerate() {
            let row_spacing = y_spacing * (i as f32 + 1.0);
            let x_start = self.x;
            let y_start = self.y + row_spacing;
            if left_mouse_click_in_area(x_start, y_start, x_start + self.box_size, y_start + self.box_size) {
                self.selected = content_line.clone();
            }
        }
    }

    pub fn get_height(&self) -> f32 {
        self.text_y + (self.contents.len() as f32) * self.box_size
    }
}
