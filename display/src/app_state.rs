use macroquad::prelude::TextParams;
use distributions::distribution_class::DistributionClass;
use distributions::distribution_type::DistributionType;
use crate::widgets::distribution_selection_widget::DistributionSelectionWidget;

pub struct AppState<'a> {
    pub x_axis_distribution_selector: DistributionSelectionWidget<'a>,
    pub y_axis_distribution_selector: DistributionSelectionWidget<'a>,
    pub user_exit_requested: bool,
}

impl<'a> AppState<'a> {
    pub async fn new(option_text_params: TextParams<'a>) -> Self {
        let graph_axis_max_value = 500.0;

        // distributions
        let mut distributions = Vec::new();
        distributions.push(DistributionClass::new(DistributionType::Beta, "Beta", vec![0.001, 0.001, -graph_axis_max_value, -graph_axis_max_value], vec![10.0, 10.0, graph_axis_max_value, graph_axis_max_value]));
        distributions.push(DistributionClass::new(DistributionType::Cauchy, "Cauchy", vec![-graph_axis_max_value, 0.001], vec![graph_axis_max_value, 50.0]));
        distributions.push(DistributionClass::new(DistributionType::Exponent, "Exponent", vec![0.0], vec![10.0]));
        distributions.push(DistributionClass::new(DistributionType::Gamma, "Gamma", vec![0.001, 0.001], vec![50.0, 50.0]));
        distributions.push(DistributionClass::new(DistributionType::Gaussian, "Gaussian", vec![-graph_axis_max_value, -graph_axis_max_value], vec![graph_axis_max_value, graph_axis_max_value]));
        distributions.push(DistributionClass::new(DistributionType::LogNormal, "LogNormal", vec![-graph_axis_max_value, 0.001], vec![graph_axis_max_value, 5.0]));
        distributions.push(DistributionClass::new(DistributionType::Uniform, "Uniform", vec![-graph_axis_max_value, -graph_axis_max_value], vec![graph_axis_max_value, graph_axis_max_value]));

        let x_axis_distribution = DistributionSelectionWidget::new(20.0, 20.0, 20.0, distributions.clone(), "X axis", &option_text_params, 0.01);
        let y_axis_distribution = DistributionSelectionWidget::new(220.0, 20.0, 20.0, distributions, "Y axis", &option_text_params, 0.01);

        Self {
            x_axis_distribution_selector: x_axis_distribution,
            y_axis_distribution_selector: y_axis_distribution,
            user_exit_requested: false,
        }
    }
}