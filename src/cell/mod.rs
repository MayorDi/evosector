use crate::constants::{
    DEFAULT_ENERGY_CELL, DEFAULT_PROTECTION_BODY_CELL, SIZE_GRID, SIZE_RENDER_SECTOR,
};
use crate::event::Event;
use crate::genome::gene::Gene;
use crate::genome::Genome;
use crate::math::get_index;
use crate::resource::Resource;
use crate::traits::{
    Behavior, CountCellsOnSector, EnergyManagement, LightCoeff, MoveCoeff, Mutable,
    Render,
};
use nalgebra::Vector2;
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

#[derive(Debug, Clone)]
pub struct Cell {
    energy: f32,
    position: Vector2<f32>,
    current_sector: usize,
    genome: Genome,
    color: Color,
    lifetime: usize,
    _protection_body: f32,
}

impl Cell {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            energy: DEFAULT_ENERGY_CELL,
            position,
            current_sector: 0,
            genome: Genome::default(),
            lifetime: 0,
            color: Color::RGB(255, 255, 255),
            _protection_body: DEFAULT_PROTECTION_BODY_CELL,
        }
    }

    pub fn reproduction(&mut self) -> Result<Self, ()> {
        if self.energy / 2.0 < 40.0 {
            return Err(());
        }

        self.energy /= 2.0;

        let mut new_cell = self.clone();
        new_cell.lifetime = 0;
        new_cell.genome.step = 0;
        new_cell.mutate();

        Ok(new_cell)
    }
}

impl Render for Cell {
    fn render(&self, canvas: &mut WindowCanvas) {
        canvas.set_draw_color(self.color);

        let rect = Rect::new(
            self.position.x as i32,
            self.position.y as i32,
            SIZE_RENDER_SECTOR / 2,
            SIZE_RENDER_SECTOR / 2,
        );
        canvas.fill_rect(rect).unwrap();
    }
}

impl Mutable for Cell {
    fn mutate(&mut self) -> bool {
        if self.genome.mutate() {
            self.color = Color::RGB(
                rand::thread_rng().gen_range(0..255),
                rand::thread_rng().gen_range(0..255),
                rand::thread_rng().gen_range(0..255),
            );
            return true;
        }

        false
    }
}

impl EnergyManagement for Cell {
    fn energy_decrease(&mut self, val: f32) {
        self.energy -= val;
    }

    fn energy_increase(&mut self, val: f32) {
        self.energy += val;
    }
}

impl Behavior for Cell {
    fn update(&mut self, index: usize) {
        
    }
}