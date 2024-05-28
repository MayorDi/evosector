use nalgebra::Vector2;

pub fn get_index(pos: Vector2<f32>, row_width: usize) -> usize {
    pos.y as usize * row_width + pos.x as usize
}

pub fn get_position(index: usize, row_width: usize) -> Vector2<f32> {
    Vector2::new((index % row_width) as f32, (index / row_width) as f32)
}
