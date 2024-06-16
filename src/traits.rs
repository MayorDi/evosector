use std::{cell::RefCell, rc::Rc};

use sdl2::render::WindowCanvas;

use crate::cell::Cell;

pub trait Render {
    fn render(&self, canvas: &mut WindowCanvas);
}

pub trait Behavior {
    fn update(&mut self, index: usize, cells: Rc<RefCell<Vec<Cell>>>);
}

pub trait Mutable {
    fn mutate(&mut self) -> bool;
}
