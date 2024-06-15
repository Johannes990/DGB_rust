use probability::distribution::Sample;
use probability::source;
pub mod functions;
pub mod distribution_class;
pub mod distribution_type;

use crate::functions::beta_distribution::BetaDistribution;
use crate::functions::cauchy_distribution::CauchyDistribution;
use crate::functions::exponential_distribution::ExponentialDistribution;
use crate::functions::gamma_distribution::GammaDistribution;
use crate::functions::gaussian_distribution::GaussianDistribution;
use crate::functions::log_normal_distribution::LogNormalDistribution;
use crate::functions::uniform_distribution::UniformDistribution;


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
