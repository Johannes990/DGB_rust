use crate::distribution_type::DistributionType;

#[derive(Clone, PartialEq)]
pub struct DistributionClass<'a> {
    pub payload: DistributionType,
    pub name: &'a str,
    pub min_values: Vec<f32>,
    pub max_values: Vec<f32>,
}

pub trait HasName {
    fn get_name(&self) -> &str;
}

pub trait HasPayload {
    type Payload;
    fn get_payload(&self) -> &Self::Payload;
}

impl<'a> HasName for DistributionClass<'a> {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl<'a> HasPayload for DistributionClass<'a> {
    type Payload = DistributionType;
    fn get_payload(&self) -> &Self::Payload {
        &self.payload
    }
}

impl<'a> DistributionClass<'a> {
    pub fn new(distribution_type: DistributionType, distribution_name: &'a str, min_values: Vec<f32>, max_values: Vec<f32>) -> Self {
        Self { payload: distribution_type, name: distribution_name, min_values, max_values }
    }
}
