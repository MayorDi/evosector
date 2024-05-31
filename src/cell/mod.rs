use crate::constants::{
    DEFAULT_ENERGY_CELL, DEFAULT_PROTECTION_BODY_CELL, SIZE_GRID, SIZE_RENDER_SECTOR,
};
use crate::event::Event;
use crate::genome::gene::Gene;
use crate::genome::Genome;
use crate::math::get_index;
use crate::resource::Resource;
use crate::traits::{
    Behavior, Checkable, CountCellsOnSector, EnergyManagement, MoveCoeff, Mutable, Render,
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

impl Checkable for Cell {
    fn is_viability(&self, index: usize) -> Result<(), Event> {
        if self.energy < 20.0 || self.lifetime > 100 {
            return Err(Event::new(move |cells, _| {
                cells.remove(index);
            }));
        }

        Ok(())
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
    fn update(&mut self, index: usize) -> Vec<Event> {
        if let Err(event) = self.is_viability(index) {
            return vec![event];
        }

        let mut events = Vec::new();

        let new_sector = get_index(self.position / SIZE_RENDER_SECTOR as f32, SIZE_GRID.0);

        if new_sector != self.current_sector {
            events.push(decrement_count_cells(self.current_sector));

            self.current_sector = get_index(self.position / SIZE_RENDER_SECTOR as f32, SIZE_GRID.0);

            events.push(increment_count_cells(self.current_sector));
        }

        if let Some(gene) = self.genome.current_gene() {
            match gene {
                Gene::ProtectionMutate(_) => {}
                Gene::ProtectionBody(_) => {}
                Gene::VectorMove(vector) => events.push(move_cell(vector, index)),
                Gene::Attack(_) => {}
                Gene::ResourceExtraction(resource) => match resource {
                    Resource::Photosynthesis => {
                        events.push(Event::new(move |cells, grid| {
                            if cells.len() <= index {
                                return;
                            }

                            let sector = grid.sectors[cells[index].current_sector];
                            if !sector.is_zero() {
                                cells[index].energy_increase(2.0 / sector.get_count_cells() as f32);
                            }
                        }));
                    }
                    Resource::Chemosynthesis => {
                        events.push(Event::new(move |cells, grid| {
                            if cells.len() <= index {
                                return;
                            }

                            let sector = grid.sectors[cells[index].current_sector];
                            if !sector.is_zero() {
                                cells[index].energy_increase(5.5 / sector.get_count_cells() as f32);
                            }
                        }));
                    }
                },
                Gene::Reproduction => events.push(reproduce_cell(index)),
            }
        }

        self.genome.next_step();
        self.lifetime += 1;
        events
    }
}

fn move_cell(vector_move: Vector2<f32>, index: usize) -> Event {
    Event::new(move |cells, grid| {
        if cells.len() <= index {
            return;
        }
        if limit_move(cells[index].position) {
            let idx_sector = get_index(
                cells[index].position / SIZE_RENDER_SECTOR as f32,
                SIZE_GRID.0,
            );
            cells[index].position += vector_move * grid.sectors[idx_sector].get_move_coeff();
            cells[index].energy_decrease(vector_move.len() as f32);
        }
    })
}

fn reproduce_cell(index: usize) -> Event {
    Event::new(move |cells, _| {
        if cells.len() <= index {
            return;
        }
        if let Ok(cell) = cells[index].reproduction() {
            cells.push(cell);
        }
    })
}

fn increment_count_cells(index: usize) -> Event {
    Event::new(move |_, grid| {
        grid.sectors[index].increment_count_cells();
    })
}

fn decrement_count_cells(index: usize) -> Event {
    Event::new(move |_, grid| {
        grid.sectors[index].decrement_count_cells();
    })
}

fn limit_move(pos: Vector2<f32>) -> bool {
    (pos.x as usize + 2) < SIZE_GRID.0 * SIZE_RENDER_SECTOR as usize
        && (pos.y as usize + 2) < SIZE_GRID.1 * SIZE_RENDER_SECTOR as usize
}
