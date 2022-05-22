extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::GlGraphics;
use piston::input::{UpdateArgs, RenderArgs};

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WINDOW_SIZE: [u32; 2] = [640, 480];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        return Self {
            gl: gl
        };
    }

    pub fn on_render(&mut self, args: &RenderArgs) {
        use graphics::*;

        self.gl.draw(args.viewport(), |_c, gl| {
            // Clear the screen.
            clear(BLACK, gl);
        });
    }

    pub fn on_update(&mut self, _args: &UpdateArgs) {
        // nothing to do
    }
}