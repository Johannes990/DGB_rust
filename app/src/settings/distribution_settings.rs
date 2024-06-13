use distributions::beta_distribution::BetaDistribution;
use distributions::cauchy_distribution::CauchyDistribution;
use distributions::exponential_distribution::ExponentialDistribution;
use distributions::gamma_distribution::GammaDistribution;
use distributions::gaussian_distribution::GaussianDistribution;
use distributions::log_normal_distribution::LogNormalDistribution;
use distributions::ProbabilityDistribution;
use distributions::uniform_distribution::UniformDistribution;

pub struct GlobalDistributionSettings {
    pub selected_distribution: DistributionType,
    pub gaussian_dist: Option<GaussianDistribution>,
    pub beta_dist: Option<BetaDistribution>,
    pub uniform_dist: Option<UniformDistribution>,
    pub exponential_dist: Option<ExponentialDistribution>,
    pub gamma_dist: Option<GammaDistribution>,
    pub log_normal_dist: Option<LogNormalDistribution>,
    pub cauchy_dist: Option<CauchyDistribution>,
}

pub enum DistributionType {
    Gaussian,
    Beta,
    Uniform,
    Exponent,
    Gamma,
    LogNormal,
    Cauchy,
}

impl Default for GlobalDistributionSettings {
    fn default() -> Self {
        Self {
            selected_distribution: DistributionType::Gaussian,
            beta_dist: None,
            gaussian_dist: None,
            uniform_dist: None,
            exponential_dist: None,
            gamma_dist: None,
            log_normal_dist: None,
            cauchy_dist: None,
        }
    }
}

pub static mut SETTINGS: GlobalDistributionSettings = GlobalDistributionSettings::default();
