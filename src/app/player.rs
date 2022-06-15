extern crate piston;
extern crate graphics;
extern crate opengl_graphics;

use piston::{UpdateArgs};
use graphics::{Context, Image};
use graphics::math::{multiply, rotate_radians, translate, scale};
use opengl_graphics::{GlGraphics, Texture};
use super::utils::{clamp, min, to_radians, Direction};
use super::{SHOW_HITBOXES, BLUE};

const PLAYER_ACCELERATION: f64 = 0.5; // 0.5 pixels down per second
const ROTATION_OFFSET: f64 = 5.0;

pub struct Player {
    x: f64,
    y: f64,
    pub width: f64,
    pub height: f64,
    velocity: [f64; 2],
    jump_height: f64,
    direction: Direction,
    window_width: u32,
}

impl Player {
    pub fn new(x: f64, y: f64, width: f64, height: f64, x_velocity: f64, window_width: u32) -> Self {
        Self {
            x: x,
            y: y,
            width: width,
            height: height,
            velocity: [x_velocity, 0.0],
            jump_height: 0.5,
            direction: Direction::Right,
            window_width: window_width,
        }
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.velocity[1] = min(self.velocity[1] + PLAYER_ACCELERATION * args.dt, 50.0);
        self.y += self.velocity[1];
        match self.direction {
            Direction::Right => {
                self.x += self.velocity[0] * args.dt;
                if self.x > self.window_width as f64 - self.width {
                    self.direction = Direction::Left;
                }
            },
            Direction::Left => {
                self.x -= self.velocity[0] * args.dt;
                if self.x < self.width {
                    self.direction = Direction::Right;
                }
            }
        }
    }

    pub fn jump(&mut self) {
        self.velocity[1] = -self.jump_height;
    }

    #[inline(always)]
    pub fn get_angle(&self) -> f64 {
        to_radians(self.get_angle_degrees())
    }

    #[inline(always)]
    pub fn get_angle_degrees(&self) -> f64 {
        clamp(self.velocity[1], -10.0, 10.0) * self.get_rotation_offset()
    }

    #[inline(always)]
    pub fn get_x(&self) -> f64 {
        self.x
    }

    #[inline(always)]
    pub fn get_y(&self) -> f64 {
        self.y
    }

    #[inline(always)]
    pub fn get_rotation_offset(&self) -> f64 {
        if self.direction == Direction::Right {
            ROTATION_OFFSET
        } else {
            -ROTATION_OFFSET
        }
    }

    #[inline(always)]
    pub fn get_y_velocity(&self) -> f64 {
        self.velocity[0]
    }

    #[inline(always)]
    pub fn get_x_velocity(&self) -> f64 {
        if self.direction == Direction::Right {
            self.velocity[0]
        } else {
            -self.velocity[0]
        }
    }

    #[inline(always)]
    pub fn get_direction(&self) -> Direction {
        self.direction
    }

    #[inline(always)]
    pub fn render(&self, c: Context, gl: &mut GlGraphics, height: u32, player_image: &mut Image, player_texture: &Texture) {
        let scale_x = if self.get_direction() == Direction::Right {
            1.0
        } else {
            -1.0
        };

        use graphics::*;
        let player_pos = [self.get_x(), (height / 2) as f64];

        // Rotate the player around it's center uising the multiply and rotate_radians functions.
        let player_pos_transform = multiply(
            translate(player_pos),
            multiply(
                rotate_radians(self.get_angle()),
                scale(scale_x, 1.0),
            ),
        );

        // Get the player transform.
        let player_transform = multiply(
            c.transform,
            player_pos_transform,
        );

        // Move the player by the player X
        player_image.draw(
            player_texture,
            &DrawState::default(),
            player_transform,
            gl
        );

        // Draw the player's hitbox.
        if SHOW_HITBOXES {
            let hitbox_transform = multiply(
                player_transform,
                multiply(
                    rotate_radians(-self.get_angle()),
                    translate([self.width / 2.0, self.height / 2.0]),
                ),
            );
            let border = graphics::Rectangle::new_border(BLUE, 1.0);
            border.draw(
                [0.0, 0.0, 0.5, 0.5],
                &c.draw_state,
                hitbox_transform,
                gl
            );
        }
    }
}