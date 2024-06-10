use probability::distribution::Sample;
use probability::source;
use crate::gaussian_distribution::GaussianDistribution;
use crate::beta_distribution::BetaDistribution;
use crate::cauchy_distribution::CauchyDistribution;
use crate::exponential_distribution::ExponentialDistribution;
use crate::gamma_distribution::GammaDistribution;
use crate::log_normal_distribution::LogNormalDistribution;
use crate::uniform_distribution::UniformDistribution;
mod gaussian_distribution;
mod beta_distribution;
mod uniform_distribution;
mod exponential_distribution;
mod gamma_distribution;
mod log_normal_distribution;
mod cauchy_distribution;



trait ProbabilityDistribution {
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
