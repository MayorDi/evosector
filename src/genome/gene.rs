use nalgebra::Vector2;
use rand::Rng;

use crate::{constants::COUNT_GENES, traits::Mutable};

#[derive(Debug, Copy, Clone, PartialEq, variant_count::VariantCount)]
pub enum Gene {
    /// Global impact
    Reproduction,
    /// Local impact
    Move(Vector2<f32>),
    If(Condition),
    EndIf,
    Goto(usize),
}

impl From<usize> for Gene {
    fn from(value: usize) -> Self {
        match value {
            0 => Gene::Reproduction,
            1 => Gene::Move(Vector2::default()),
            2 => Gene::If(Condition::default()),
            3 => Gene::EndIf,
            _ => Gene::Goto(Default::default()),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Default, variant_count::VariantCount)]
pub enum Condition {
    IsLight,
    #[default]
    IsWater,
    IsSolid,
}

impl From<usize> for Condition {
    fn from(value: usize) -> Self {
        match value {
            0 => Condition::IsLight,
            1 => Condition::IsSolid,
            2 => Condition::IsWater,
            _ => panic!("From<u8> for Condition => {}", value),
        }
    }
}

impl Mutable for Gene {
    fn mutate(&mut self) -> bool {
        match self {
            Self::Move(v) => {
                *v = Vector2::new(
                    v.x + rand::thread_rng().gen_range(-1.0..1.0),
                    v.y + rand::thread_rng().gen_range(-1.0..1.0),
                )
            }
            Self::If(condition) => {
                *condition =
                    Condition::from(rand::thread_rng().gen_range(0..Condition::VARIANT_COUNT));
            }
            Self::Goto(v) => {
                *v = rand::thread_rng().gen_range(0..COUNT_GENES);
            }
            _ => {}
        }

        return true;
    }
}
