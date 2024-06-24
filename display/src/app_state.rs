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

        let distributions = Self::initialize_distribution_vector(graph_axis_max_value);

        let x_axis_distribution = DistributionSelectionWidget::new(20.0, 20.0, 20.0, distributions.clone(), "X axis", &option_text_params, 0.01);
        let y_axis_distribution = DistributionSelectionWidget::new(220.0, 20.0, 20.0, distributions.clone(), "Y axis", &option_text_params, 0.01);

        Self {
            x_axis_distribution_selector: x_axis_distribution,
            y_axis_distribution_selector: y_axis_distribution,
            user_exit_requested: false,
        }
    }

    fn initialize_distribution_vector(max_range: f32) -> Vec<DistributionClass<'a>> {
        let mut distributions = Vec::new();

        distributions.push(DistributionClass::new(DistributionType::Beta, "Beta", vec![0.001, 0.001, -max_range, -max_range], vec![10.0, 10.0, max_range, max_range]));
        distributions.push(DistributionClass::new(DistributionType::Cauchy, "Cauchy", vec![-max_range, 0.001], vec![max_range, 50.0]));
        distributions.push(DistributionClass::new(DistributionType::Exponent, "Exponent", vec![0.0], vec![10.0]));
        distributions.push(DistributionClass::new(DistributionType::Gamma, "Gamma", vec![0.001, 0.001], vec![50.0, 50.0]));
        distributions.push(DistributionClass::new(DistributionType::Gaussian, "Gaussian", vec![-max_range, -max_range], vec![max_range, max_range]));
        distributions.push(DistributionClass::new(DistributionType::LogNormal, "LogNormal", vec![-max_range, 0.001], vec![max_range, 5.0]));
        distributions.push(DistributionClass::new(DistributionType::Uniform, "Uniform", vec![-max_range, -max_range], vec![max_range, max_range]));

        distributions
    }
}