extern crate sdl2;

use evosector::cell::Cell;
use evosector::constants::{SIZE_GRID, SIZE_RENDER_SECTOR};
use evosector::grid::Grid;
use evosector::traits::{Behavior, Render};
use nalgebra::Vector2;
use noise::utils::{ColorGradient, ImageRenderer};
use sdl2::event::Event;
use sdl2::image::{InitFlag, LoadTexture};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Evosector", 1200, 600)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let seed = 0;
    let grid = Grid::generate(seed);

    evosector::grid::write_image_to_file(
        &ImageRenderer::new()
            .set_gradient(ColorGradient::new().build_terrain_gradient())
            .render(&Grid::generate_noisemap(seed)),
        "texture_grid.png",
    );

    let texture_creator = canvas.texture_creator();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();
    let texture_grid = texture_creator
        .load_texture("./assets/textures/texture_grid.png")
        .unwrap();

    let cells = Rc::new(RefCell::new(vec![Cell::new(Vector2::new(250.0, 250.0))]));

    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        {
            grid.render(&mut canvas);
            let rect = Rect::new(
                0,
                0,
                SIZE_GRID.0 as u32 * SIZE_RENDER_SECTOR,
                SIZE_GRID.1 as u32 * SIZE_RENDER_SECTOR,
            );
            canvas.copy(&texture_grid, None, Some(rect)).unwrap();
        }

        cells.render(&mut canvas);

        canvas.present();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 120));
    }

    Ok(())
}
