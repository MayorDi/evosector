use std::path::Path;

use crate::constants::SIZE_GRID;
use crate::math::get_position;
use crate::sector::Sector;
use crate::traits::Render;
use noise::{utils::*, *};

#[derive(Debug, Clone)]
pub struct Grid {
    pub sectors: Vec<Sector>,
}

impl Grid {
    pub fn generate_noisemap(seed: u32) -> NoiseMap {
        let billow = Billow::<Perlin>::default()
            .set_seed(seed)
            .set_frequency(1.67456);

        let noisemap = PlaneMapBuilder::new(billow)
            .set_size(SIZE_GRID.0, SIZE_GRID.1)
            .set_x_bounds(0.0, 0.5)
            .set_y_bounds(0.0, 0.5)
            .build();

        noisemap
    }

    pub fn generate(seed: u32) -> Self {
        let noisemap = Self::generate_noisemap(seed);

        let mut grid = Self {
            sectors: vec![Sector::default(); SIZE_GRID.2],
        };

        for (index, sector) in grid.sectors.iter_mut().enumerate() {
            let pos = get_position(index, SIZE_GRID.0);
            sector.altitude = noisemap.get_value(pos.x as usize, pos.y as usize);
        }

        grid
    }
}

use noise::utils::{NoiseImage, NoiseMap};
pub fn write_image_to_file(image: &NoiseImage, filename: &str) {
    use std::fs;

    let target = Path::new("./assets/textures").join(Path::new(filename));

    fs::create_dir_all(target.clone().parent().expect("No parent directory found."))
        .expect("Failed to create directories.");

    image.write_to_file(&target)
}

impl Render for Grid {
    fn render(&self) {}
}
