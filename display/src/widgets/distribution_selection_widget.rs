use std::collections::HashMap;
use distributions::distribution_class::{DistributionClass, HasPayload};
use macroquad::prelude::*;
use distributions::distribution_type::DistributionType;
use crate::widgets::selection_list::SelectionList;
use crate::widgets::slider::{Slider, SliderType};

pub struct DistributionSelectionWidget<'a> {
    pub x: f32,
    pub y: f32,
    pub heading: String,
    pub selection_list: SelectionList<'a, DistributionClass>,
    pub parameter_sliders: Vec<Slider<'a>>,
    pub previous_selected_type: Option<DistributionType>,
    pub slider_values: HashMap<DistributionType, Vec<f32>>,
}

impl<'a> DistributionSelectionWidget<'a> {
    pub fn new(x: f32,
               y: f32,
               box_size: f32,
               contents: Vec<DistributionClass>,
               heading: String,
               option_text_params: &TextParams<'a>) -> Self {
        let selection_list = SelectionList::new(x, y, box_size, contents, heading.clone(), option_text_params.clone());
        let slider_y = y + selection_list.get_height();
        let mut slider_values = HashMap::new();

        for distribution_class in &selection_list.contents {
            let parameter_count = Self::get_parameter_count(&distribution_class.get_payload());
            slider_values.insert(distribution_class.get_payload().clone(), vec![0.0; parameter_count]);
        }

        let parameter_sliders = Self::create_sliders(&selection_list.selected.get_payload(), slider_y, x, option_text_params);

        Self { x, y, heading, selection_list, parameter_sliders, previous_selected_type: None, slider_values }
    }

    fn create_sliders(distribution_type: &DistributionType,
                      slider_y: f32,
                      x: f32,
                      option_text_params: &TextParams<'a>) -> Vec<Slider<'a>> {
        let param_names: Vec<&str> = match distribution_type {
            DistributionType::Gaussian => vec!["mean", "std_dev"],
            DistributionType::Beta => vec!["alpha", "beta", "a", "b"],
            DistributionType::Uniform => vec!["a", "b"],
            DistributionType::Exponent => vec!["labmda"],
            DistributionType::Gamma => vec!["k", "theta"],
            DistributionType::LogNormal => vec!["mu", "sigma"],
            DistributionType::Cauchy => vec!["mean", "gamma"],
        };

        param_names.iter().enumerate().map(|(i, &name)| {
            Slider::new(
                x,
                slider_y + (i * 60) as f32,
                100.0,
                20.0,
                0.0,
                500.0,
                1.0,
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
        let selected_distribution = self.selection_list.selected.get_payload().clone();

        if let Some(slider_values) = self.slider_values.get_mut(&selected_distribution) {
            for (i, slider) in self.parameter_sliders.iter_mut().enumerate() {
                slider.handle_input();
                slider_values[i] = slider.current_value;
            }
        }

        self.selection_list.handle_input();

        let new_selected_distribution = self.selection_list.selected.get_payload().clone();

        if new_selected_distribution != selected_distribution {
            self.parameter_sliders = Self::create_sliders(
                &new_selected_distribution,
                self.y + self.selection_list.get_height(),
                self.x,
                &self.selection_list.text_params
            );

            if let Some(slider_values) = self.slider_values.get(&new_selected_distribution) {
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
