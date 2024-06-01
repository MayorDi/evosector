use nalgebra::Vector2;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Gene {
    Reproduction,
    Move(Vector2<f32>),
    If(Condition),
    EndIf,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Condition {
    IsLight,
    IsWater,
    IsSolid,
}
