use crate::constants::{
    COUNT_GENES, DEFAULT_ENERGY_CELL, DEFAULT_PROTECTION_BODY_CELL, SIZE_RENDER_SECTOR,
};
use crate::event::Event;
use crate::genome::gene::Gene;
use crate::genome::Genome;
use crate::resource::Resource;
use crate::traits::{Behavior, Render};
use nalgebra::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
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
            genome: Genome::default(),
            protection_body: DEFAULT_PROTECTION_BODY_CELL,
        }
    }

    pub fn reproduction(&mut self) -> Self {
        let mut new_cell = self.clone();
        new_cell.genome.step = 0;

        new_cell
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

impl Behavior for Cell {
    fn update(&mut self, index: usize) -> Vec<Event> {
        let mut events = Vec::new();

        if self.energy < 20.0 {
            events.push(Event::new(move |cells, _| {
                cells.remove(index);
            }));
            return events;
        }

        if let Some(gene) = self.genome.genes[self.genome.step] {
            match gene {
                Gene::ProtectionMutate(_) => {}
                Gene::ProtectionBody(_) => {}
                Gene::VectorMove(vector_move) => self.position += vector_move,
                Gene::Attack(_) => {}
                Gene::ResourceExtraction(resource) => match resource {
                    Resource::Photosynthesis => {
                        events.push(Event::new(|_, _| {}));
                    }
                    Resource::Chemosynthesis => {}
                },
                Gene::Reproduction => events.push(Event::new(move |cells, _| {
                    let new_cell = cells[index].reproduction();
                    cells.push(new_cell);
                })),
            }
        }

        if self.genome.step + 1 < COUNT_GENES {
            self.genome.step += 1;
        } else {
            self.genome.step = 0;
        }

        events
    }
}
