use probability::prelude::*;
use crate::normal_distribution::NormalDistribution;
use crate::beta_distribution::BetaDistribution;
mod normal_distribution;
mod beta_distribution;


trait ProbabilityDistribution {
    fn generate_random_sample(&self) -> f64;
    fn generate_random_pair(&self) -> (f64, f64);
}

impl ProbabilityDistribution for normal_distribution::NormalDistribution {
    fn generate_random_sample(&self) -> f64 {
        let mut source = source::default(1911);
        self.dist.sample(&mut source)
    }

    fn generate_random_pair(&self) -> (f64, f64) {
        let mut source = source::default(3753);
        let rand1: f64 = self.dist.sample(&mut source);
        let rand2: f64 = self.dist.sample(&mut source);

        (rand1, rand2)
    }
}