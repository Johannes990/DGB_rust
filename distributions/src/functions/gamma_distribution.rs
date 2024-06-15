use probability::distribution::Gamma;

pub struct GammaDistribution {
    k: f64,
    theta: f64,
    pub(crate) dist: Gamma,
}

impl GammaDistribution {
    pub fn new(k: f64, theta: f64) -> Result<GammaDistribution, String> {
        if k <= 0.0 {
            return Err("k must be a positive real-valued number.".to_string())
        }
        if theta <= 0.0 {
            return Err("theta must be a positive real_valued number.".to_string())
        }
        let gamma = Gamma::new(k, theta);

        Ok( GammaDistribution{ k, theta, dist: gamma })
    }
}