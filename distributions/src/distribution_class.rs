use crate::distribution_type::DistributionType;

#[derive(Clone, PartialEq)]
pub struct DistributionClass {
    pub payload: DistributionType,
    pub name: String,
}

pub trait HasName {
    fn get_name(&self) -> &str;
}

pub trait HasPayload {
    type Payload;
    fn get_payload(&self) -> &Self::Payload;
}

impl HasName for DistributionClass {
    fn get_name(&self) -> &str {
        &self.name
    }
}

impl HasPayload for DistributionClass {
    type Payload = DistributionType;
    fn get_payload(&self) -> &Self::Payload {
        &self.payload
    }
}

impl DistributionClass {
    pub fn new(distribution_type: DistributionType, distribution_name: String) -> Self {
        Self { payload: distribution_type, name: distribution_name }
    }
}
