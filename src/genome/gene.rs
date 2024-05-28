use nalgebra::Vector2;
use crate::alias::EnergyConsumptionCoefficient;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Gene {
    ProtectionMutate(EnergyConsumptionCoefficient),
    ProtectionBody(EnergyConsumptionCoefficient),
    VectorMove(Vector2<f32>),
    Attack(EnergyConsumptionCoefficient),
    ResourceExtraction(Resource),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Resource {
    Photosynthesis,
    Chemosynthesis
}
