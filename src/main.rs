extern crate sdl2;

use evosector::cell::Cell;
use evosector::event::Event as EventMessage;
use evosector::grid::Grid;
use evosector::traits::{Behavior, Render};
use nalgebra::Vector2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Evosector", 800, 600)
        .resizable()
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut grid = Grid::generate(1);
    let mut cells: Vec<Cell> = vec![Cell::new(Vector2::new(250.0, 250.0))];

    'running: loop {
        let mut events = Vec::new();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        grid.render(&mut canvas);
        cells.render(&mut canvas);

        for (idx, cell) in cells.iter_mut().enumerate() {
            events.append(&mut cell.update(idx));
        }

        event_handler(events, &mut cells, &mut grid);
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

fn event_handler(events: Vec<EventMessage>, cells: &mut Vec<Cell>, grid: &mut Grid) {
    for event in events.iter() {
        event.0(cells, grid);
    }
}
