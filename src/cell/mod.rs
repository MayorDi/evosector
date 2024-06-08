use crate::constants::{
    DEFAULT_ENERGY_CELL, SIZE_RENDER_SECTOR
};
use crate::genome::Genome;
use crate::traits::{
    Mutable,
    Render,
};
use nalgebra::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

#[derive(Debug, Clone)]
pub struct Cell {
    energy: f32,
    position: Vector2<f32>,
    genome: Genome,
    color: Color,
    lifetime: usize,
}

impl Cell {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            energy: DEFAULT_ENERGY_CELL,
            position,
            genome: Genome::default(),
            lifetime: 0,
            color: Color::RGB(255, 255, 255),
        }
    }

    pub fn reproduction(&mut self) -> Result<Self, ()> {
        todo!()
    }
}

impl Render for Cell {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);

        let rect = Rect::new(
            self.position.x as i32 * SIZE_RENDER_SECTOR as i32,
            self.position.y as i32 * SIZE_RENDER_SECTOR as i32,
            SIZE_RENDER_SECTOR / 2,
            SIZE_RENDER_SECTOR / 2,
        );
        canvas.fill_rect(rect).unwrap();
    }
}

impl Mutable for Cell {
    fn mutate(&mut self) -> bool {
        self.genome.mutate()
    }
}
