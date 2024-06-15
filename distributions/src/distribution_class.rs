use crate::distribution_type::DistributionType;

#[derive(Clone, PartialEq)]
pub struct DistributionClass {
    pub distribution_type: DistributionType,
    pub distribution_name: String,
}

pub trait HasName {
    fn get_name(&self) -> &str;
}

pub trait HasDistributionType {
    fn get_distribution_type(&self) -> DistributionType;
}

impl HasName for DistributionClass {
    fn get_name(&self) -> &str {
        &self.distribution_name
    }
}

impl HasDistributionType for DistributionClass {
    fn get_distribution_type(&self) -> DistributionType {
        self.distribution_type.clone()
    }
}

impl DistributionClass {
    pub fn new(distribution_type: DistributionType, distribution_name: String) -> Self {
        Self { distribution_type, distribution_name }
    }
}
