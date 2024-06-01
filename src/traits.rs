use crate::event::Event;
use sdl2::render::WindowCanvas;

pub trait Render {
    fn render(&self, canvas: &mut WindowCanvas);
}

pub trait Behavior {
    fn update(&mut self, index: usize);
}

pub trait LightCoeff {
    fn get_light_coeff(&self) -> f32;
}

pub trait MoveCoeff {
    fn get_move_coeff(&self) -> f32;
}

pub trait ResourceCoeff {
    fn get_resource_coeff(&self) -> f32;
}

pub trait CountCellsOnSector {
    fn get_count_cells(&self) -> usize;
    fn is_zero(&self) -> bool;
    fn decrement_count_cells(&mut self);
    fn increment_count_cells(&mut self);
}

pub trait Mutable {
    fn mutate(&mut self) -> bool;
}

pub trait EnergyManagement {
    fn energy_decrease(&mut self, val: f32);
    fn energy_increase(&mut self, val: f32);
}
