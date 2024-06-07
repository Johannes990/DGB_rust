use probability::distribution::Sample;
use probability::source;
use crate::normal_distribution::NormalDistribution;
use crate::beta_distribution::BetaDistribution;
mod normal_distribution;
mod beta_distribution;


trait ProbabilityDistribution {
    type Source;
    fn generate_random_sample(&self, source: &mut Self::Source) -> f64;
    fn generate_random_pair(&self, source: &mut Self::Source) -> (f64, f64);
}

impl ProbabilityDistribution for NormalDistribution {
    type Source = source::Default;
    fn generate_random_sample(&self, source: &mut Self::Source) -> f64 {
        self.dist.sample(source)
    }

    fn generate_random_pair(&self, source: &mut Self::Source) -> (f64, f64) {
        let rand1: f64 = self.generate_random_sample(source);
        let rand2: f64 = self.generate_random_sample(source);

        (rand1, rand2)
    }
}

impl ProbabilityDistribution for BetaDistribution {
    type Source = source::Default;

    fn generate_random_sample(&self, source: &mut Self::Source) -> f64 {
        self.dist.sample(source)
    }

    fn generate_random_pair(&self, source: &mut Self::Source) -> (f64, f64) {
        let rand1: f64 = self.generate_random_sample(source);
        let rand2: f64 = self.generate_random_sample(source);

        (rand1, rand2)
    }
}