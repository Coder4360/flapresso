extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderEvent, UpdateEvent};
use piston::window::WindowSettings;
use piston::input::{Event, Input, ButtonState};

mod app;

#[forbid(unsafe_code)]
fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create a Glutin window.

    let mut window: Window = WindowSettings::new("Flapresso", app::WINDOW_SIZE)
        .graphics_api(opengl)
        .resizable(false)
        .exit_on_esc(true)
        .decorated(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = app::App::new(GlGraphics::new(opengl));
    app.on_init();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.on_render(&args);
        }

        if let Some(args) = e.update_args() {
            app.on_update(&args);
        }

        // Check if the event is an input event.
        match e {
            Event::Input(input, _time) => {
                match input {
                    Input::Button(args) => {
                        match args.state {
                            ButtonState::Press => {
                                app.on_press(args);
                            }
                            ButtonState::Release => {
                                app.on_release(args);
                            }
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }
}