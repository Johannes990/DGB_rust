use distributions::distribution_type::DistributionType;
use distributions::distribution_class::DistributionClass;

#[derive(Clone)]
pub struct DistributionSettings {
    pub x_axis_distribution: DistributionClass,
    pub x_axis_distribution_var_1: Option<f32>,
    pub x_axis_distribution_var_2: Option<f32>,
    pub x_axis_distribution_var_3: Option<f32>,
    pub x_axis_distribution_var_4: Option<f32>,
    pub y_axis_distribution: DistributionClass,
    pub y_axis_distribution_var_1: Option<f32>,
    pub y_axis_distribution_var_2: Option<f32>,
    pub y_axis_distribution_var_3: Option<f32>,
    pub y_axis_distribution_var_4: Option<f32>,
}

impl DistributionSettings {
    pub fn new() -> Self {
        Self {
            x_axis_distribution: DistributionClass::new(DistributionType::Gaussian, "Gaussian".to_string()),
            x_axis_distribution_var_1: Some(0.0),
            x_axis_distribution_var_2: Some(1.0),
            x_axis_distribution_var_3: None,
            x_axis_distribution_var_4: None,
            y_axis_distribution: DistributionClass::new(DistributionType::Gaussian, "Gaussian".to_string()),
            y_axis_distribution_var_1: Some(0.0),
            y_axis_distribution_var_2: Some(1.0),
            y_axis_distribution_var_3: None,
            y_axis_distribution_var_4: None,
        }
    }
}
