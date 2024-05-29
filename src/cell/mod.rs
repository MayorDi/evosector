use crate::constants::{
    DEFAULT_ENERGY_CELL, DEFAULT_PROTECTION_BODY_CELL, SIZE_GRID, SIZE_RENDER_SECTOR,
};
use crate::event::Event;
use crate::genome::gene::Gene;
use crate::genome::Genome;
use crate::resource::Resource;
use crate::traits::{Behavior, Checkable, Render};
use nalgebra::Vector2;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

#[derive(Debug, Clone)]
pub struct Cell {
    energy: f32,
    position: Vector2<f32>,
    genome: Genome,
    _protection_body: f32,
}

impl Cell {
    pub fn new(position: Vector2<f32>) -> Self {
        Self {
            energy: DEFAULT_ENERGY_CELL,
            position,
            genome: Genome::default(),
            _protection_body: DEFAULT_PROTECTION_BODY_CELL,
        }
    }

    pub fn reproduction(&mut self) -> Result<Self, ()> {
        if self.energy / 2.0 < 40.0 {
            return Err(());
        }

        self.energy /= 2.0;

        let mut new_cell = self.clone();
        new_cell.genome.step = 0;

        Ok(new_cell)
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

impl Checkable for Cell {
    fn is_viability(&self, index: usize) -> Result<(), Event> {
        if self.energy < 20.0 {
            return Err(Event::new(move |cells, _| {
                cells.remove(index);
            }));
        }

        Ok(())
    }
}

impl Behavior for Cell {
    fn update(&mut self, index: usize) -> Vec<Event> {
        if let Err(event) = self.is_viability(index) {
            return vec![event];
        }

        let mut events = Vec::new();

        if let Some(gene) = self.genome.current_gene() {
            match gene {
                Gene::ProtectionMutate(_) => {}
                Gene::ProtectionBody(_) => {}
                Gene::VectorMove(vector) => events.push(move_cell(vector, index)),
                Gene::Attack(_) => {}
                Gene::ResourceExtraction(resource) => match resource {
                    Resource::Photosynthesis => events.push(Event::new(|_, _| {})),
                    Resource::Chemosynthesis => {}
                },
                Gene::Reproduction => events.push(reproduce_cell(index)),
            }
        }

        self.genome.next_step();

        events
    }
}

fn move_cell(vector_move: Vector2<f32>, index: usize) -> Event {
    Event::new(move |cells, _grid| {
        if limit_move(cells[index].position) {
            cells[index].position += vector_move;
        }
    })
}

fn reproduce_cell(index: usize) -> Event {
    Event::new(move |cells, _| {
        if let Ok(cell) = cells[index].reproduction() {
            cells.push(cell);
        }
    })
}

fn limit_move(pos: Vector2<f32>) -> bool {
    (pos.x as usize + 2) < SIZE_GRID.0 * SIZE_RENDER_SECTOR as usize
        && (pos.y as usize + 2) < SIZE_GRID.1 * SIZE_RENDER_SECTOR as usize
}
