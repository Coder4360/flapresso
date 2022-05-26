extern crate opengl_graphics;
extern crate piston;

use opengl_graphics::{GlGraphics, Texture, TextureSettings};
use piston::input::{UpdateArgs, RenderArgs};
use graphics::{Image, DrawState};
use graphics::rectangle::square;
use graphics::ImageSize;
use graphics::math::{rotate_radians, multiply, translate, scale};
use std::path::Path;
use piston::input::{ButtonArgs, Button, Key};
use utils::Direction;

mod player;
pub mod utils;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const WINDOW_SIZE: [u32; 2] = [640, 480];

const BACKGROUND_IMAGE_PATH: &str = "data/gfx/bg.png";
const PLAYER_IMAGE_PATH: &str = "data/gfx/player.png";

const PLAYER_X_VELOCITY: f64 = 3.0;
const DEBUG_INFO: bool = false;

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    background_image: Image,
    background_texture: Texture,
    player: player::Player,
    player_image: Image,
    player_texture: Texture,
    paused: bool,
}

impl App {
    pub fn new(gl: GlGraphics) -> Self {
        // Don't initialize the images, just create the values.
        Self {
            gl: gl,
            background_image: Image::new(),
            background_texture: Texture::new(0, 0, 0),
            player: player::Player::new(0.0, 0.0, 0.0, 0.0, PLAYER_X_VELOCITY, WINDOW_SIZE[0]),
            player_image: Image::new(),
            player_texture: Texture::new(0, 0, 0),
            paused: false,
        }
    }

    pub fn on_init(&mut self) {
        // Load images to memory
        self.background_image.rect(square(0.0, 0.0, 1.0));
        self.background_texture = Texture::from_path(
            &Path::new(BACKGROUND_IMAGE_PATH),
            &TextureSettings::new(),
        ).unwrap();
        self.player_image.rect(square(0.0, 0.0, 1.0));
        self.player_texture = Texture::from_path(
            &Path::new(PLAYER_IMAGE_PATH),
            &TextureSettings::new(),
        ).unwrap();
        self.player.width = self.player_texture.get_size().0 as f64;
        self.player.height = self.player_texture.get_size().1 as f64;
    }

    pub fn on_render(&mut self, args: &RenderArgs) {
        use graphics::clear;

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLACK, gl);

            let player_y = self.player.get_y();

            let scale_x = if self.player.get_direction() == Direction::Right {
                1.0
            } else {
                -1.0
            };

            let (_, bg_height) = self.background_texture.get_size();
            let background_y_index = player_y / bg_height as f64 + 0.5;
            let background_y = |idx| (idx as i64 * bg_height as i64) as f64;

            if DEBUG_INFO { println!("Background y: {}", background_y(background_y_index)); } // debug

            // Draw the background image.
            self.background_image.draw(
                &self.background_texture,
                &DrawState::default(),
                multiply(c.transform, translate([
                    0.0,
                    background_y(background_y_index - 1.0) - player_y])),
                gl
            );

            // Draw images above and below the background image.
            self.background_image.draw(
                &self.background_texture,
                &DrawState::default(),
                multiply(c.transform, translate([
                    0.0,
                    background_y(background_y_index) - player_y])),
                gl
            );

            self.background_image.draw(
                &self.background_texture,
                &DrawState::default(),
                multiply(c.transform, translate([
                    0.0,
                    background_y(background_y_index - 3.0) - player_y])),
                gl
            );


            let player_pos = [self.player.get_x(), (WINDOW_SIZE[1] / 2) as f64];

            // Rotate the player around it's center uising the multiply and rotate_radians functions.
            let player_pos_transform = multiply(
                translate(player_pos),
                multiply(
                    rotate_radians(self.player.get_angle()),
                    scale(scale_x, 1.0),
                ),
            );

            // Get the player transform.
            let player_transform = multiply(
                c.transform,
                player_pos_transform,
            );

            if DEBUG_INFO { println!("Angle: {}°", self.player.get_angle_degrees()); } // debug

            if DEBUG_INFO { println!("Player pos: {:?}", player_pos); } // debug

            // Move the player by the player X
            self.player_image.draw(
                &self.player_texture,
                &DrawState::default(),
                player_transform,
                gl
            );
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
        }
    }

    pub fn on_press(&mut self, args: ButtonArgs) {
        match args.button {
            Button::Keyboard(key) => {
                match key {
                    Key::Space => {
                        self.player.jump();
                    }
                    Key::P => {
                        self.paused = !self.paused;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
    }

    pub fn on_release(&mut self, _args: ButtonArgs) {

    }
}