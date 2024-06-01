#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Solid {
    pub light_coefficient: f32,
    pub move_coefficient: f32,
    pub resource_coefficient: f32,
    pub count_cells: usize,
}

impl Default for Solid {
    fn default() -> Self {
        Self {
            light_coefficient: 1.0,
            move_coefficient: 0.2,
            resource_coefficient: 0.0,
            count_cells: 0,
        }
    }
}
