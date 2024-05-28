use crate::cell::Cell;
use crate::traits::Render;
use sdl2::render::WindowCanvas;

impl Render for Vec<Cell> {
    fn render(&self, canvas: &mut WindowCanvas) {
        for cell in self.iter() {
            cell.render(canvas);
        }
    }
}
