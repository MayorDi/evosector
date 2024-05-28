use crate::cell::Cell;
use crate::grid::Grid;

pub struct Event(pub Box<dyn Fn(&mut Vec<Cell>, &mut Grid)>);

impl Event {
    pub fn new(f: impl Fn(&mut Vec<Cell>, &mut Grid) + 'static) -> Self {
        Self(Box::new(f))
    }
}
