use crate::constants::{DEFAULT_ENERGY_CELL, DEFAULT_PROTECTION_BODY_CELL, SIZE_RENDER_SECTOR};
use crate::genome::Genome;
use crate::traits::Render;
use nalgebra::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::render::WindowCanvas;

#[derive(Debug, Clone)]
pub struct Cell {
    energy: f32,
    position: Vector2<f32>,
    genome: Genome,
    protection_body: f32,
}

impl Cell {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            energy: DEFAULT_ENERGY_CELL,
            position,
            genome: Genome::new(),
            protection_body: DEFAULT_PROTECTION_BODY_CELL,
        }
    }
}

impl Render for Cell {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));

        let rect = Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            SIZE_RENDER_SECTOR / 2,
            SIZE_RENDER_SECTOR / 2,
        );
        canvas.fill_rect(rect).unwrap();
    }
}
