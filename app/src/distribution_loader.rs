use display::app_state::AppState;
use distributions::distribution_class::HasPayload;
use distributions::distribution_type::DistributionType;
use distributions::functions::beta_distribution::BetaDistribution;
use distributions::functions::cauchy_distribution::CauchyDistribution;
use distributions::functions::exponential_distribution::ExponentialDistribution;
use distributions::functions::gamma_distribution::GammaDistribution;
use distributions::functions::gaussian_distribution::GaussianDistribution;
use distributions::functions::log_normal_distribution::LogNormalDistribution;
use distributions::functions::uniform_distribution::UniformDistribution;

pub struct DistributionLoader<'a> {
    pub x_axis_distribution_type: &'a DistributionType,
    pub y_axis_distribution_type: &'a DistributionType,
}

impl<'a> DistributionLoader<'a> {
    pub fn new(app_state: &'a AppState) -> Self {
        let x_axis_distribution_type = app_state.x_axis_distribution_selector.selection_list.selected.get_payload();
        let y_axis_distribution_type = app_state.y_axis_distribution_selector.selection_list.selected.get_payload();

        Self { x_axis_distribution_type, y_axis_distribution_type }
    }

    pub fn get_x_axis_distribution(&self, app_state: &'a AppState) {
        let dist_values = app_state.x_axis_distribution_selector.slider_values.get(self.x_axis_distribution_type);

        if Some(dist_values).is_some() {
            Self::get_distribution(&self, self.x_axis_distribution_type, dist_values.unwrap())
        }
    }

    pub fn get_y_axis_distribution(&self, app_state: &'a AppState) {
        let dist_values = app_state.y_axis_distribution_selector.slider_values.get(self.y_axis_distribution_type);

        if Some(dist_values).is_some() {
            Self::get_distribution(&self, self.y_axis_distribution_type, dist_values.unwrap())
        }
    }

    fn get_distribution(&self, dist_type: &DistributionType, parameter_values: &Vec<f32>) {
        match dist_type {
            DistributionType::Gaussian => {
                GaussianDistribution::new(parameter_values[0] as f64, parameter_values[1] as f64).expect("Can't create and return Gaussian distribution!");
            }
            DistributionType::Beta => {
                BetaDistribution::new(parameter_values[0] as f64, parameter_values[1] as f64, parameter_values[2] as f64, parameter_values[3] as f64).expect("Can't create and return Beta distribution!");

            }
            DistributionType::Uniform => {
                UniformDistribution::new(parameter_values[0] as f64, parameter_values[1] as f64).expect("Can't create and return Uniform distribution!");
            }
            DistributionType::Exponent => {
                ExponentialDistribution::new(parameter_values[0] as f64).expect("Can't create and return Exponential distribution!");
            }
            DistributionType::Gamma => {
                GammaDistribution::new(parameter_values[0] as f64, parameter_values[1] as f64).expect("Can't create and return Gamma distribution!");
            }
            DistributionType::LogNormal => {
                LogNormalDistribution::new(parameter_values[0] as f64, parameter_values[1] as f64).expect("Can't create and return LogNormal distribution!");
            }
            DistributionType::Cauchy => {
                CauchyDistribution::new(parameter_values[0] as f64, parameter_values[1] as f64).expect("Can't create and return Cauchy distribution!");
            }
        }
    }
}
