use probability::distribution::Uniform;

pub struct UniformDistribution {
    a: f64,
    b: f64,
    pub(crate) dist: Uniform,
}

impl UniformDistribution {
    pub fn new(a: f64, b: f64) -> Result<UniformDistribution, String> {
        if a < b {
            return Err("a must be greater than b.".to_string())
        }
        let uniform = Uniform::new(a, b);

        Ok( UniformDistribution { a, b, dist: uniform } )
    }
}