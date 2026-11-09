use crate::app::Frame;
use crate::gfx::Gl;
use crate::render::Renderer;

pub trait Scene: 'static {
    fn update(&mut self, _gl: &Gl, _frame: &Frame) {}
    fn render(&mut self, renderer: &mut Renderer, frame: &Frame);
}
