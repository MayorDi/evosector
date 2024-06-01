#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Water {
    pub light_coefficient: f32,
    pub move_coefficient: f32,
    pub resource_coefficient: f32,
    pub count_cells: usize,
}

impl Default for Water {
    fn default() -> Self {
        Self {
            light_coefficient: 0.1,
            move_coefficient: 1.0,
            resource_coefficient: 0.0,
            count_cells: 0,
        }
    }
}
