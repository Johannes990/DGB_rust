use std::collections::HashMap;
use distributions::distribution_class::{DistributionClass, HasPayload};
use macroquad::prelude::*;
use distributions::distribution_type::DistributionType;
use crate::widgets::selection_list::SelectionList;
use crate::widgets::slider::{Slider, SliderType};

pub struct DistributionSelectionWidget<'a> {
    pub x: f32,
    pub y: f32,
    pub heading: &'a str,
    pub selection_list: SelectionList<'a, DistributionClass<'a>>,
    pub parameter_sliders: Vec<Slider<'a>>,
    pub previous_selected_type: Option<DistributionType>,
    pub slider_values: HashMap<DistributionType, Vec<f32>>,
    pub default_slider_value: f32,
}

impl<'a> DistributionSelectionWidget<'a> {
    pub fn new(x: f32,
               y: f32,
               box_size: f32,
               contents: Vec<DistributionClass<'a>>,
               heading: &'a str,
               option_text_params: &TextParams<'a>,
               default_slider_value: f32) -> Self {
        let selection_list = SelectionList::new(x, y, box_size, contents, heading, option_text_params.clone());
        let slider_y = y + selection_list.get_height() + 68.0; // 68.0 is to get good visual alignment
        println!("slider_y: {}, self.y: {}, sel_list.get_height: {}", slider_y, y, selection_list.get_height());
        let mut slider_values = HashMap::new();

        for distribution_class in &selection_list.contents {
            let parameter_count = Self::get_parameter_count(&distribution_class.get_payload());
            let mut initial_vals = Vec::new();

            for j in 0..parameter_count {
                initial_vals.push(j as f32 * 0.01 + 0.001); // some distribution parameters have to be above zero and also can't be equal to each other
            }

            slider_values.insert(distribution_class.get_payload().clone(), initial_vals);
        }

        let parameter_sliders = Self::create_sliders(&selection_list.selected, slider_y, x, option_text_params, default_slider_value);

        Self { x, y, heading, selection_list, parameter_sliders, previous_selected_type: None, slider_values, default_slider_value }
    }

    fn create_sliders(distribution_class: &DistributionClass,
                      slider_y: f32,
                      x: f32,
                      option_text_params: &TextParams<'a>,
                      default_slider_value: f32) -> Vec<Slider<'a>> {
        let param_names: Vec<&str> = match distribution_class.get_payload() {
            DistributionType::Gaussian => vec!["mean", "std_dev"],
            DistributionType::Beta => vec!["alpha", "beta", "a", "b"],
            DistributionType::Uniform => vec!["a", "b"],
            DistributionType::Exponent => vec!["lambda"],
            DistributionType::Gamma => vec!["k", "theta"],
            DistributionType::LogNormal => vec!["mu", "sigma"],
            DistributionType::Cauchy => vec!["mean", "gamma"],
        };

        param_names.iter().enumerate().map(|(i, &name)| {
            let knob_size = 20.0;

            Slider::new(
                x + 0.5 * knob_size,
                slider_y + (i * 55) as f32,
                100.0,
                knob_size,
                distribution_class.min_values[i],
                distribution_class.max_values[i],
                default_slider_value,
                SliderType::Horizontal,
                name,
                &option_text_params
            )
        }).collect()
    }

    fn get_parameter_count(distribution_type: &DistributionType) -> usize {
        match distribution_type {
            DistributionType::Gaussian
            | DistributionType::Uniform
            | DistributionType::Gamma
            | DistributionType::LogNormal
            | DistributionType::Cauchy => 2,
            DistributionType::Beta => 4,
            DistributionType::Exponent => 1,
        }
    }

    pub fn draw(&mut self) {
        self.selection_list.draw();

        for slider in &self.parameter_sliders {
            slider.draw();
        }
    }

    pub fn handle_input(&mut self) {
        let selected_distribution = self.selection_list.selected.clone();

        if let Some(slider_values) = self.slider_values.get_mut(&selected_distribution.get_payload()) {
            for (i, slider) in self.parameter_sliders.iter_mut().enumerate() {
                slider.handle_input();
                slider_values[i] = slider.current_value;
            }
        }

        self.selection_list.handle_input();

        let new_selected_distribution = self.selection_list.selected.clone();

        if new_selected_distribution != selected_distribution {
            let slider_y = self.y + self.selection_list.get_height() + 30.0;

            self.parameter_sliders = Self::create_sliders(
                &new_selected_distribution,
                slider_y,
                self.x,
                &self.selection_list.text_params,
                self.default_slider_value
            );

            if let Some(slider_values) = self.slider_values.get(&new_selected_distribution.get_payload()) {
                for (i, slider) in self.parameter_sliders.iter_mut().enumerate() {
                    slider.current_value = slider_values[i];
                }
            }
        }

        for mut slider in &mut self.parameter_sliders {
            slider.handle_input();
        }
    }
}
