#[derive(Clone)]
pub struct DistributionSettings {
    pub x_axis_distribution: DistributionType,
    pub x_axis_distribution_var_1: Option<f32>,
    pub x_axis_distribution_var_2: Option<f32>,
    pub x_axis_distribution_var_3: Option<f32>,
    pub x_axis_distribution_var_4: Option<f32>,
    pub y_axis_distribution: DistributionType,
    pub y_axis_distribution_var_1: Option<f32>,
    pub y_axis_distribution_var_2: Option<f32>,
    pub y_axis_distribution_var_3: Option<f32>,
    pub y_axis_distribution_var_4: Option<f32>,
}

#[derive(PartialEq, Clone)]
pub enum DistributionType {
    Gaussian,
    Beta,
    Uniform,
    Exponent,
    Gamma,
    LogNormal,
    Cauchy,
}

impl DistributionSettings {
    pub fn new() -> Self {
        Self {
            x_axis_distribution: DistributionType::Gaussian,
            x_axis_distribution_var_1: Some(0.0),
            x_axis_distribution_var_2: Some(1.0),
            x_axis_distribution_var_3: None,
            x_axis_distribution_var_4: None,
            y_axis_distribution: DistributionType::Gaussian,
            y_axis_distribution_var_1: Some(0.0),
            y_axis_distribution_var_2: Some(1.0),
            y_axis_distribution_var_3: None,
            y_axis_distribution_var_4: None,
        }
    }
}
