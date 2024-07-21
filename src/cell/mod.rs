use crate::constants::{DEFAULT_ENERGY_CELL, SIZE_GRID, SIZE_RENDER_CELL};
use crate::genome::gene::Gene;
use crate::genome::Genome;
use crate::grid::Grid;
use crate::math::get_index;
use crate::traits::{Behavior, Mutable, Render};
use nalgebra::Vector2;

#[derive(Debug, Clone)]
pub struct Cell {
    energy: f32,
    position: Vector2<f32>,
    genome: Genome,
    // color: Color,
    lifetime: usize,
}

impl Cell {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            energy: DEFAULT_ENERGY_CELL,
            position,
            genome: Genome::default(),
            lifetime: 0,
            // color: Color::RGB(255, 255, 255),
        }
    }

    pub fn reproduction(&mut self) -> Result<Self, ()> {
        let mut new_cell = self.clone();
        new_cell.genome.step = 0;

        Ok(new_cell)
    }
}

impl Render for Cell {
    fn render(&self) {
        let vert_data = [
            self.position.x - SIZE_RENDER_CELL,
            self.position.y - SIZE_RENDER_CELL,
            0.0,
            self.position.x + SIZE_RENDER_CELL,
            self.position.y - SIZE_RENDER_CELL,
            0.0,
            self.position.x - SIZE_RENDER_CELL,
            self.position.y + SIZE_RENDER_CELL,
            0.0,
            self.position.x - SIZE_RENDER_CELL,
            self.position.y + SIZE_RENDER_CELL,
            0.0,
            self.position.x + SIZE_RENDER_CELL,
            self.position.y + SIZE_RENDER_CELL,
            0.0,
            self.position.x + SIZE_RENDER_CELL,
            self.position.y - SIZE_RENDER_CELL,
            0.0,
        ];

        unsafe {
            gl::BufferSubData(
                gl::ARRAY_BUFFER,
                0,
                (vert_data.len() * std::mem::size_of::<f32>()) as isize,
                vert_data.as_ptr() as *const _,
            );

            gl::DrawArrays(gl::TRIANGLES, 0, 6);
        }
    }
}

impl Behavior for Cell {
    type GlobalImpact = Gene;
    fn update(&mut self, grid: &Grid) -> Option<Self::GlobalImpact> {
        if let Some(gene) = self.genome[self.genome.step] {
            match gene {
                Gene::Move(move_to) => {
                    if limit_move(self.position, move_to) {
                        let k = grid.sectors[get_index(self.position, SIZE_GRID.0)];
                        self.position += move_to * (1.0 - k.altitude);
                    }
                }

                _ => {}
            }
        }

        self.genome.next_step();
        None
    }
}

impl Mutable for Cell {
    fn mutate(&mut self) -> bool {
        self.genome.mutate()
    }
}

fn limit_move(move_from: Vector2<f32>, move_to: Vector2<f32>) -> bool {
    let new_pos = move_from + move_to;

    new_pos.x >= 0.0
        && new_pos.y >= 0.0
        && new_pos.x <= (SIZE_GRID.0 as f32)
        && new_pos.y <= (SIZE_GRID.1 as f32)
}
