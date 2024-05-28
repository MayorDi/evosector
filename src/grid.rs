use crate::constants::{SIZE_GRID, SIZE_RENDER_SECTOR};
use crate::sector::solid::Solid;
use crate::sector::Sector;
use crate::traits::Render;
use gen_world::noise::Noise;
use gen_world::traits::{GenerateNoise, NoiseBody};
use rand::Rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::WindowCanvas;

#[derive(Debug, Copy, Clone)]
pub struct Grid {
    sectors: [Sector; SIZE_GRID.0 * SIZE_GRID.1],
}

impl Grid {
    // It needs to be redone.
    pub fn generate(seed: u64) -> Self {
        let mut noise: Noise<u8> = Noise::new(seed, SIZE_GRID.0, SIZE_GRID.1, 0..55).gen(|noise| {
            for x in noise.body().iter_mut() {
                if rand::thread_rng().gen_range(0.0..1.0) < 0.1 {
                    *x = rand::thread_rng().gen_range(0..55);
                }
            }
        });
        for _ in 0..5 {
            noise.smooth();
        }

        let mut grid = Grid {
            sectors: [Sector::Water(crate::sector::water::Water::default());
                SIZE_GRID.0 * SIZE_GRID.1],
        };

        for (idx, n) in noise.body().iter().enumerate() {
            if *n >= 2 {
                grid.sectors[idx] = Sector::Solid(Solid::default());
            }
        }

        grid
    }
}

impl Render for Grid {
    fn render(&self, canvas: &mut WindowCanvas) {
        for (idx, sector) in self.sectors.iter().enumerate() {
            let pos = crate::math::get_position(idx, SIZE_GRID.0);
            let rect = Rect::new(
                pos.x as i32 * SIZE_RENDER_SECTOR as i32,
                pos.y as i32 * SIZE_RENDER_SECTOR as i32,
                SIZE_RENDER_SECTOR,
                SIZE_RENDER_SECTOR,
            );

            match *sector {
                Sector::Solid(solid) => {
                    canvas.set_draw_color(Color::RGB(100, 100, 100));
                }
                Sector::Water(water) => {
                    canvas.set_draw_color(Color::RGB(50, 50, 150));
                }
            }

            canvas.fill_rect(rect).unwrap();
        }
    }
}
