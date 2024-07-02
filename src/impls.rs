use std::cell::RefCell;
use std::rc::Rc;

use crate::cell::Cell;
use crate::traits::Render;

impl Render for Rc<RefCell<Vec<Cell>>> {
    fn render(&self) {
        todo!()
        // for cell in ((**self).borrow()).iter() {
        //     cell.render(canvas);
        // }
    }
}
