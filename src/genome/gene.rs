use crate::alias::EnergyConsumptionCoefficient;
use crate::resource::Resource;
use nalgebra::Vector2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Gene {
    ProtectionMutate(EnergyConsumptionCoefficient),
    ProtectionBody(EnergyConsumptionCoefficient),
    VectorMove(Vector2<f32>),
    Attack(EnergyConsumptionCoefficient),
    ResourceExtraction(Resource),
    Reproduction,
}
