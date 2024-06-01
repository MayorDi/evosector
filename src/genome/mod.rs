use std::ops::{Index, IndexMut};

use crate::constants::COUNT_GENES;
use crate::genome::gene::Gene;
use crate::traits::Mutable;

pub mod gene;

#[derive(Debug, Clone, PartialEq)]
pub struct Genome {
    pub genes: [Option<Gene>; COUNT_GENES],
    pub step: usize,
}

impl Genome {
    pub fn new() -> Self {
        Self {
            genes: [None; COUNT_GENES],
            step: 0,
        }
    }
}

impl Index<usize> for Genome {
    type Output = Option<Gene>;
    fn index(&self, index: usize) -> &Self::Output {
        &self.genes[index]
    }
}

impl IndexMut<usize> for Genome {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.genes[index]
    }
}

impl Mutable for Genome {
    fn mutate(&mut self) -> bool {
        todo!()
    }
}

impl Default for Genome {
    fn default() -> Self {
        let mut genome = Genome::new();
        

        todo!()
    }
}
