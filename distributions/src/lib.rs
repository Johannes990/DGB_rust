use std::error::Error;
use probability::prelude::*;


trait ProbabilityDistribution {
    fn generate_random_sample(&self) -> f64;
    fn generate_random_pair(&self) -> (f64, f64);
}

struct NormalDistribution {
    mean: f64,
    std_dev: f64,
    dist: Gaussian,
}

impl NormalDistribution {
    pub fn new(mean: f64, std_dev: f64) -> Result<NormalDistribution, dyn Error> {
        if std_dev < 0.0 {
            return Err("Standard deviation must be a non-negative value".to_string());
        }
        let norm_dist = Gaussian::new(mean, std_dev);

        Ok( NormalDistribution { mean, std_dev, dist: norm_dist } )
    }
}

impl ProbabilityDistribution for NormalDistribution {
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