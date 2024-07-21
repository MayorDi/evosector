use crate::constants::{DEFAULT_ENERGY_CELL, SIZE_GRID, SIZE_RENDER_CELL};
use crate::genome::gene::Gene;
use crate::genome::Genome;
use crate::grid::Grid;
use crate::math::get_index;
use crate::traits::{Behavior, Mutable, Render};
use nalgebra::Vector2;

#[derive(Debug, Clone)]
pub struct Cell {
    pub energy: f32,
    position: Vector2<f32>,
    genome: Genome,
    // color: Color,
    lifetime: usize,
    pub idx_sector: usize,
    pub is_alive: bool,
}

impl Cell {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            energy: DEFAULT_ENERGY_CELL,
            position,
            genome: Genome::default(),
            lifetime: 0,
            idx_sector: 0,
            is_alive: true,
            // color: Color::RGB(255, 255, 255),
        }
    }

    pub fn reproduction(&mut self) -> Option<Self> {
        if self.energy >= 60.0 {
            self.energy /= 2.0;
            self.lifetime = 0;

            self.mutate();

            let mut new_cell = self.clone();
            new_cell.genome.step = 0;

            return Some(new_cell);
        }

        None
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
    fn update(&mut self, grid: &mut Grid) -> Option<Self::GlobalImpact> {
        let mut ret_gene = None;

        if !self.is_alive {
            return ret_gene;
        }

        let mut sector = &mut grid.sectors[self.idx_sector];

        if let Some(gene) = self.genome[self.genome.step] {
            match gene {
                Gene::Move(move_to) => {
                    self.energy -= (move_to.len() as f32).powf(2.0);
                    limit_move(&mut self.position, move_to * (1.0 - sector.altitude));

                    let new_idx_sector = get_index(self.position, SIZE_GRID.0);

                    if new_idx_sector != self.idx_sector {
                        sector.count_of_cells -= 1.0;

                        self.idx_sector = new_idx_sector;
                        sector = &mut grid.sectors[self.idx_sector];
                        sector.count_of_cells += 1.0;
                    }
                }

                Gene::Goto(step) => self.genome.step = step,
                Gene::Reproduction => ret_gene = Some(Gene::Reproduction),

                _ => {}
            }
        }

        self.energy += 15.0 / (sector.count_of_cells * sector.count_of_cells * sector.count_of_cells);
        self.lifetime += 1;

        if self.lifetime >= 72 || self.energy <= 30.0 {
            self.is_alive = false;
            sector.count_of_cells -= 1.0;
        }

        self.genome.next_step();
        ret_gene
    }
}

impl Mutable for Cell {
    fn mutate(&mut self) -> bool {
        self.genome.mutate()
    }
}

fn limit_move(move_from: &mut Vector2<f32>, move_to: Vector2<f32>) {
    let mut new_pos = *move_from + move_to;

    if new_pos.x < 0.5 {
        new_pos.x = SIZE_GRID.0 as f32 - 0.5;
    } else if new_pos.x > SIZE_GRID.0 as f32 - 0.5 {
        new_pos.x = 0.5;
    }

    if new_pos.y < 0.5 {
        new_pos.y = SIZE_GRID.1 as f32 - 0.5;
    } else if new_pos.y > SIZE_GRID.1 as f32 - 0.5 {
        new_pos.y = 0.5;
    }

    *move_from = new_pos;
}
