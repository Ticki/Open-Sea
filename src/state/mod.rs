use piston::event::RenderArgs;

use opengl_graphics::GlGraphics;

use renderer::Renderer;


/// A state, i.e. an rendering state
pub trait State {
    /// Render the view.
    fn render(&self,
              //dt: f64, // render should not care about dt
              args: &RenderArgs,
              gl: &mut GlGraphics,
              renderer: &mut Renderer);

    /// This gets called when the view starts
    fn start(&mut self) {}
    /// This gets called when the view ends
    fn end(&mut self) {}
}

//mod game; // TODO uncomment. squelching errors.
//pub use self::game::Game; // TODO uncomment. squelching errors.
