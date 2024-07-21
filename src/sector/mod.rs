#[derive(Debug, Clone, Copy, Default)]
pub struct Sector {
    /// `altitude >= 0` solid \
    /// `altitude < 0` water
    pub altitude: f32,
    pub count_of_cells: usize,
}
