use macroquad::prelude::TextParams;
use distributions::distribution_class::DistributionClass;
use settings::distribution_settings::DistributionSettings;
use crate::elements::{slider::Slider, slider::SliderType, selection_list::SelectionList};
use distributions::distribution_type::DistributionType;

pub struct AppState<'a> {
    pub distribution_settings: DistributionSettings,
    pub x_axis_distribution_list: SelectionList<'a, DistributionClass>,
    pub x_axis_distribution_params: Vec<Slider<'a>>,
    pub y_axis_distribution_list: SelectionList<'a, DistributionClass>,
    pub y_axis_distribution_params: Vec<Slider<'a>>,
    pub user_exit_requested: bool,
}

impl<'a> AppState<'a> {
    pub async fn new(option_text_params: TextParams<'a>) -> Self {
        // distribution classes and lists for both axes
        let mut distributions = Vec::new();
        distributions.push(DistributionClass::new(DistributionType::Beta, "Beta".to_string()));
        distributions.push(DistributionClass::new(DistributionType::Cauchy, "Cauchy".to_string()));
        distributions.push(DistributionClass::new(DistributionType::Exponent, "Exponent".to_string()));
        distributions.push(DistributionClass::new(DistributionType::Gamma, "Gamma".to_string()));
        distributions.push(DistributionClass::new(DistributionType::Gaussian, "Gaussian".to_string()));
        distributions.push(DistributionClass::new(DistributionType::LogNormal, "LogNormal".to_string()));
        distributions.push(DistributionClass::new(DistributionType::Uniform, "Uniform".to_string()));
        let x_axis_distribution_list = SelectionList::new(20.0, 20.0, 20.0, distributions.clone(), "X-axis distribution".to_string(), option_text_params.clone());
        let y_axis_distribution_list = SelectionList::new(180.0, 20.0, 20.0, distributions.clone(), "Y-axis distributions".to_string(), option_text_params.clone());

        // distribution parameter sliders
        let mut x_axis_distribution_params = Vec::new();
        let mut y_axis_distribution_params = Vec::new();
        x_axis_distribution_params.push(Slider::new(25.0, 350.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 1", &option_text_params.clone()));
        x_axis_distribution_params.push(Slider::new(25.0, 410.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 2", &option_text_params.clone()));
        x_axis_distribution_params.push(Slider::new(25.0, 470.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 3", &option_text_params.clone()));
        x_axis_distribution_params.push(Slider::new(25.0, 530.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 4", &option_text_params.clone()));
        y_axis_distribution_params.push(Slider::new(185.0, 350.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 1", &option_text_params.clone()));
        y_axis_distribution_params.push(Slider::new(185.0, 410.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 2", &option_text_params.clone()));
        y_axis_distribution_params.push(Slider::new(185.0, 470.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 3", &option_text_params.clone()));
        y_axis_distribution_params.push(Slider::new(185.0, 530.0, 60.0, 20.0, 0.0, 10.0, 1.0, SliderType::Horizontal, "parameter 4", &option_text_params.clone()));

        Self {
            distribution_settings: DistributionSettings::new(),
            x_axis_distribution_list,
            x_axis_distribution_params,
            y_axis_distribution_list,
            y_axis_distribution_params,
            user_exit_requested: false,
        }
    }
}