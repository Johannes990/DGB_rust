use std::error::Error;
use probability::distribution::Gaussian;

pub struct NormalDistribution {
    mean: f64,
    std_dev: f64,
    pub(crate) dist: Gaussian,
}

impl NormalDistribution {
    pub fn new(mean: f64, std_dev: f64) -> Result<NormalDistribution, dyn Error> {
        if std_dev < 0.0 {
            return Err("Standard deviation must be a non-negative real_valued number.".to_string());
        }
        let norm_dist = Gaussian::new(mean, std_dev);

        Ok( NormalDistribution { mean, std_dev, dist: norm_dist } )
    }
}