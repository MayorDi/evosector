use crate::event::Event;
use sdl2::render::WindowCanvas;

pub trait Render {
    fn render(&self, canvas: &mut WindowCanvas);
}

pub trait Behavior {
    fn update(&mut self, index: usize) -> Vec<Event>;
}
