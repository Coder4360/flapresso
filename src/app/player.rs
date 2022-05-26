extern crate piston;
use piston::UpdateArgs;
use super::utils::{clamp, min, to_radians, Direction};

const PLAYER_ACCELERATION: f64 = 0.1; // 0.5 pixels down per second
const ROTATION_OFFSET: f64 = 25.0;

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
        clamp(self.velocity[1], -10.0, 5.0) * self.get_rotation_offset()
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
}