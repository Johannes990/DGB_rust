use probability::distribution::Sample;
use probability::source;
use crate::gaussian_distribution::GaussianDistribution;
use crate::beta_distribution::BetaDistribution;
use crate::cauchy_distribution::CauchyDistribution;
use crate::exponential_distribution::ExponentialDistribution;
use crate::gamma_distribution::GammaDistribution;
use crate::log_normal_distribution::LogNormalDistribution;
use crate::uniform_distribution::UniformDistribution;
pub mod gaussian_distribution;
pub mod beta_distribution;
pub mod uniform_distribution;
pub mod exponential_distribution;
pub mod gamma_distribution;
pub mod log_normal_distribution;
pub mod cauchy_distribution;

pub trait ProbabilityDistribution {
    type Source;
    fn generate_random_sample(&self, source: &mut Self::Source) -> f64;
    fn generate_random_pair(&self, source: &mut Self::Source) -> (f64, f64) {
        let rand1 = self.generate_random_sample(source);
        let rand2 = self.generate_random_sample(source);

        (rand1, rand2)
    }
}

impl ProbabilityDistribution for GaussianDistribution {
    type Source = source::Default;
    fn generate_random_sample(&self, source: &mut Self::Source) -> f64 {
        self.dist.sample(source)
    }
}

impl ProbabilityDistribution for BetaDistribution {
    type Source = source::Default;
    fn generate_random_sample(&self, source: &mut Self::Source) -> f64 {
        self.dist.sample(source)
    }
}

impl ProbabilityDistribution for UniformDistribution {
    type Source = source::Default;
    fn generate_random_sample(&self, source: &mut Self::Source) -> f64 {
        self.dist.sample(source)
    }
}

impl ProbabilityDistribution for ExponentialDistribution {
    type Source = source::Default;

    fn generate_random_sample(&self, source: &mut Self::Source) -> f64 {
        self.dist.sample(source)
    }
}

impl ProbabilityDistribution for GammaDistribution {
    type Source = source::Default;

    fn generate_random_sample(&self, source: &mut Self::Source) -> f64 {
        self.dist.sample(source)
    }
}

impl ProbabilityDistribution for LogNormalDistribution {
    type Source = source::Default;

    fn generate_random_sample(&self, source: &mut Self::Source) -> f64 {
        self.dist.sample(source)
    }
}

impl ProbabilityDistribution for CauchyDistribution {
    type Source = source::Default;

    fn generate_random_sample(&self, source: &mut Self::Source) -> f64 {
        self.dist.sample(source)
    }
}
