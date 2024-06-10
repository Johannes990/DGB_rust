use probability::distribution::Beta;

pub struct BetaDistribution {
    alpha: f64,
    beta: f64,
    a: f64,
    b: f64,
    pub(crate) dist: Beta,
}

impl BetaDistribution {
    pub fn new(alpha: f64, beta: f64, a: f64, b: f64) -> Result<BetaDistribution, String> {
        if alpha <= 0.0 {
            return Err("Alpha must be a positive real-valued number.".to_string())
        }
        if beta <= 0.0 {
            return Err("Beta must be a positive real-valued number.".to_string())
        }
        if a < b {
            return Err("a must be greater than b.".to_string())
        }
        let beta_dist = Beta::new(alpha, beta, a, b);

        Ok( BetaDistribution{ alpha, beta, a, b, dist: beta_dist })
    }
}
