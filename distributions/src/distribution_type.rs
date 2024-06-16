#[derive(PartialEq, Clone, Eq, Hash)]
pub enum DistributionType {
    Gaussian,
    Beta,
    Uniform,
    Exponent,
    Gamma,
    LogNormal,
    Cauchy,
}
