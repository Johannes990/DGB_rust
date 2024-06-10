use probability::distribution::Exponential;

pub struct ExponentialDistribution {
    lambda: f64,
    pub(crate) dist: Exponential,
}

impl ExponentialDistribution {
    pub fn new(lambda: f64) -> Result<ExponentialDistribution, String> {
        if lambda <= 0.0 {
            return Err("Lambda should be a non-negative real-valued number.".to_string())
        }
        let exp = Exponential::new(lambda);

        Ok( ExponentialDistribution { lambda, dist: exp } )
    }
}