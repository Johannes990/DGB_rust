use probability::distribution::Lognormal;


pub struct LogNormalDistribution {
    mu: f64,
    sigma: f64,
    pub(crate) dist: Lognormal,
}

impl LogNormalDistribution {
    pub fn new(mu: f64, sigma: f64) -> Result<LogNormalDistribution, String> {
        if sigma <= 0.0 {
            return Err("Sigma must be a positive real-valued number.".to_string())
        }

        let lognorm_dist = Lognormal::new(mu, sigma);

        Ok( LogNormalDistribution{ mu, sigma, dist: lognorm_dist })
    }
}
