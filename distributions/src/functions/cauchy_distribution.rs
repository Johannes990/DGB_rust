use probability::distribution::Cauchy;

pub struct CauchyDistribution {
    x_0: f64,
    gamma: f64,
    pub(crate) dist: Cauchy,
}

impl CauchyDistribution {
    pub fn new(x_0: f64, gamma: f64) -> Result<CauchyDistribution, String> {
        if gamma <= 0.0 {
            return Err("Gamma must be a positive real-valued number.".to_string())
        }

        let cauchy = Cauchy::new(x_0, gamma);

        Ok( CauchyDistribution{ x_0, gamma, dist: cauchy })
    }
}