extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::input::{UpdateArgs, RenderArgs};
use graphics::{Image, DrawState};
use graphics::rectangle::square;
use graphics::ImageSize;
use graphics::math::{multiply, translate};
use std::path::Path;
use piston::input::{ButtonArgs, Button, Key, MouseButton};

mod player;
mod beans;
pub mod utils;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WINDOW_SIZE: [u32; 2] = [640, 480];

const BACKGROUND_IMAGE_PATH: &str = "data/gfx/bg.png";
const PLAYER_IMAGE_PATH: &str = "data/gfx/player.png";
const BEAN_IMAGE_PATH: &str = "data/gfx/bean.png";

const PLAYER_X_VELOCITY: f64 = 170.0;
const DEBUG_INFO: bool = false;
const SHOW_HITBOXES: bool = false; // Only for debugging.

const BLUE: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    background_image: Image,
    background_texture: Texture,
    player_image: Image,
    player_texture: Texture,
    bean_image: Image,
    bean_texture: Texture,
    player: player::Player,
    paused: bool,
    beans: std::vec::Vec<beans::Bean>,
    bean_size: [f64; 2],
    bean_count: usize,
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        // Don't initialize the images, just create the values.
        Self {
            gl: gl,
            background_image: Image::new(),
            background_texture: Texture::new(0, 0, 0),
            player_image: Image::new(),
            player_texture: Texture::new(0, 0, 0),
            bean_image: Image::new(),
            bean_texture: Texture::new(0, 0, 0),
            player: player::Player::new(0.0, 0.0, 0.0, 0.0, PLAYER_X_VELOCITY, WINDOW_SIZE[0]),
            paused: false,
            beans: std::vec::Vec::new(),
            bean_size: [0.0, 0.0],
            bean_count: 5,
        }
    }

    pub fn on_init(&mut self) {
        // Load images to memory
        self.background_image.rect(square(0.0, 0.0, 1.0));
        self.background_texture = Texture::from_path(
            &Path::new(BACKGROUND_IMAGE_PATH),
            &TextureSettings::new(),
        ).unwrap_or_else(|e| panic!("Error loading background image: {}", e));

        self.player_image.rect(square(0.0, 0.0, 1.0));
        self.player_texture = Texture::from_path(
            &Path::new(PLAYER_IMAGE_PATH),
            &TextureSettings::new(),
        ).unwrap_or_else(|e| panic!("Error loading player image: {}", e));
        self.player.width = self.player_texture.get_size().0 as f64;
        self.player.height = self.player_texture.get_size().1 as f64;

        self.bean_image.rect(square(0.0, 0.0, 1.0));
        self.bean_texture = Texture::from_path(
            &Path::new(BEAN_IMAGE_PATH),
            &TextureSettings::new(),
        ).unwrap_or_else(|e| panic!("Error loading bean image: {}", e));
        self.bean_size[0] = self.bean_texture.get_size().0 as f64;
        self.bean_size[1] = self.bean_texture.get_size().1 as f64;
    }

    pub fn on_render(&mut self, args: &RenderArgs) {
        use graphics::clear;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let player_y = self.player.get_y();

            let (_, bg_height) = self.background_texture.get_size();
            let background_y_index = player_y / bg_height as f64 - 0.5;
            let background_y = |idx| (idx as i64 * bg_height as i64) as f64 - player_y;

            if DEBUG_INFO { println!("Background y: {}", background_y(background_y_index)); } // debug

            // Draw the background image.
            self.background_image.draw(
                &self.background_texture,
                &DrawState::default(),
                multiply(c.transform, translate([
                    0.0,
                    background_y(background_y_index - 1.0)])),
                gl
            );

            // Draw images above and below the background image.
            self.background_image.draw(
                &self.background_texture,
                &DrawState::default(),
                multiply(c.transform, translate([
                    0.0,
                    background_y(background_y_index)])),
                gl
            );

            self.background_image.draw(
                &self.background_texture,
                &DrawState::default(),
                multiply(c.transform, translate([
                    0.0,
                    background_y(background_y_index + 1.0)])),
                gl
            );

            self.player.render(c, gl, WINDOW_SIZE[1], &mut self.player_image, &self.player_texture);

            for bean in &mut self.beans {
                bean.render(c, gl, &mut self.bean_image, &self.bean_texture, self.player.get_y());
            }
        });
    }

    pub fn on_update(&mut self, args: &UpdateArgs) {
        if self.paused { return; }
        self.player.update(args);
        if DEBUG_INFO {
            println!("Y velocity: {}", self.player.get_y_velocity()); // debug
            println!("Y: {}", self.player.get_y()); // debug
            println!("X velocity: {}", self.player.get_x_velocity()); // debug
            println!("X: {}", self.player.get_x()); // debug
            println!("Angle: {}", self.player.get_angle()); // debug
        }
        if self.beans.len() < self.bean_count {
            self.beans.push(beans::Bean::random(WINDOW_SIZE[0] as f64, WINDOW_SIZE[1] as f64, self.player.get_y()));
        }
        
        let player_x = self.player.get_x();
        let player_y = self.player.get_y();
        let mut bean_states: std::vec::Vec<beans::BeanEvent> = std::vec::Vec::with_capacity(self.beans.len());
        for bean in self.beans.iter() {
            bean_states.push(bean.update(player_x, player_y, WINDOW_SIZE[1] as f64));
        }
        for (index, bean_state) in bean_states.iter().enumerate() {
            if *bean_state == beans::BeanEvent::Eaten {
                self.beans.remove(index);
            }

            if *bean_state == beans::BeanEvent::OffScreen {
                self.beans.remove(index);
                let bean = beans::Bean::random(WINDOW_SIZE[0] as f64, WINDOW_SIZE[1] as f64, self.player.get_y() - WINDOW_SIZE[1] as f64 / 2.0);
                self.beans.push(bean);
            }
        }
    }

    pub fn on_press(&mut self, args: ButtonArgs) {
        match args.button {
            Button::Keyboard(key) => {
                match key {
                    Key::Space => { self.player.jump(); }
                    Key::Up => { self.player.jump(); }
                    Key::P => { self.paused = !self.paused; }
                    _ => {}
                }
            }
            Button::Mouse(button) => {
                match button {
                    MouseButton::Left => { self.player.jump(); }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn on_release(&mut self, _args: ButtonArgs) {

    }
}