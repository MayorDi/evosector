use std::cell::RefCell;
use std::rc::Rc;

use crate::cell::Cell;
use crate::traits::Render;
use sdl2::render::WindowCanvas;

impl Render for Rc<RefCell<Vec<Cell>>> {
    fn render(&self, canvas: &mut WindowCanvas) {
        for cell in ((**self).borrow()).iter() {
            cell.render(canvas);
        }
    }
}
