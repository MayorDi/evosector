use sdl2::render::WindowCanvas;

pub trait Render {
    fn render(&self, canvas: &mut WindowCanvas);
}
