#[derive(Debug, Clone, Copy)]
pub struct Sector {
    /// `altitude >= 0` solid \
    /// `altitude < 0` water
    pub altitude: f32,
    pub count_of_cells: f32,
}

impl Default for Sector {
    fn default() -> Self {
        Self {
            altitude: 0.0,
            count_of_cells: 1.0,
        }
    }
}
