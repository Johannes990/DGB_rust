use macroquad::prelude::TextParams;
use distributions::distribution_class::DistributionClass;
use distributions::distribution_type::DistributionType;
use crate::widgets::distribution_selection_widget::DistributionSelectionWidget;

pub struct AppState<'a> {
    pub x_axis_distribution: DistributionSelectionWidget<'a>,
    pub y_axis_distribution: DistributionSelectionWidget<'a>,
    pub user_exit_requested: bool,
}

impl<'a> AppState<'a> {
    pub async fn new(option_text_params: TextParams<'a>) -> Self {
        // distributions
        let mut distributions = Vec::new();
        distributions.push(DistributionClass::new(DistributionType::Beta, "Beta".to_string()));
        distributions.push(DistributionClass::new(DistributionType::Cauchy, "Cauchy".to_string()));
        distributions.push(DistributionClass::new(DistributionType::Exponent, "Exponent".to_string()));
        distributions.push(DistributionClass::new(DistributionType::Gamma, "Gamma".to_string()));
        distributions.push(DistributionClass::new(DistributionType::Gaussian, "Gaussian".to_string()));
        distributions.push(DistributionClass::new(DistributionType::LogNormal, "LogNormal".to_string()));
        distributions.push(DistributionClass::new(DistributionType::Uniform, "Uniform".to_string()));

        let x_axis_distribution = DistributionSelectionWidget::new(20.0, 20.0, 20.0, distributions.clone(), "X axis".to_string(), &option_text_params);
        let y_axis_distribution = DistributionSelectionWidget::new(200.0, 20.0, 20.0, distributions, "Y axis".to_string(), &option_text_params);

        Self {
            x_axis_distribution,
            y_axis_distribution,
            user_exit_requested: false,
        }
    }
}