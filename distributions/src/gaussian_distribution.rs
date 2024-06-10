use probability::distribution::Gaussian;

pub struct GaussianDistribution {
    mean: f64,
    std_dev: f64,
    pub(crate) dist: Gaussian,
}

impl GaussianDistribution {
    pub fn new(mean: f64, std_dev: f64) -> Result<GaussianDistribution, String> {
        if std_dev < 0.0 {
            return Err("Standard deviation must be a non-negative real_valued number.".to_string());
        }
        let norm_dist = Gaussian::new(mean, std_dev);

        Ok( GaussianDistribution{ mean, std_dev, dist: norm_dist })
    }

    pub fn dist(&self) -> &Gaussian {
        &self.dist
    }
}