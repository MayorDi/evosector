use crate::constants::{DEFAULT_ENERGY_CELL, SIZE_RENDER_CELL};
use crate::genome::Genome;
use crate::traits::{Mutable, Render};
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
        todo!()
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

impl Mutable for Cell {
    fn mutate(&mut self) -> bool {
        self.genome.mutate()
    }
}
