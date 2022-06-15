extern crate piston;
extern crate rand;
extern crate graphics;
extern crate opengl_graphics;

use super::{DEBUG_INFO, SHOW_HITBOXES, BLUE};
use super::utils::left_justify_str;
use graphics::{Context, Image, DrawState};
use graphics::math::{translate, multiply};
use opengl_graphics::{GlGraphics, Texture};

pub struct Bean {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

pub enum BeanEvent {
    None,
    Eaten,
    OffScreen,
}

impl PartialEq for BeanEvent {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (BeanEvent::None, BeanEvent::None) => true,
            (BeanEvent::Eaten, BeanEvent::Eaten) => true,
            _ => false,
        }
    }
}

impl Clone for BeanEvent {
    fn clone(&self) -> Self {
        match self {
            BeanEvent::None => BeanEvent::None,
            BeanEvent::Eaten => BeanEvent::Eaten,
            BeanEvent::OffScreen => BeanEvent::OffScreen,
        }
    }
}

impl Copy for BeanEvent {}

impl Bean {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x: x,
            y: y,
            width: width,
            height: height,
        }
    }

    pub fn random(width: f64, height: f64, player_y: f64) -> Self {
        let x = rand::random::<f64>() * (width - 20.0);
        let y = player_y + rand::random::<f64>() * (height - 20.0);
        Self::new(x, y, 1.0, 1.0)
    }

    #[inline(always)]
    pub fn render(&self, c: Context, gl: &mut GlGraphics, image: &Image, texture: &Texture, player_y: f64) {
        use graphics::rectangle;

        let transform = multiply(
            c.transform,
            translate([self.x, self.y - player_y]),
        );
        image.draw(texture, &DrawState::default(), transform, gl);
        
        if SHOW_HITBOXES {
            let rectangle_transform = multiply(
                c.transform,
                translate([self.x, self.y - player_y]),
            );
            let square = rectangle::Rectangle::new_border(BLUE, self.width);
            square.draw(
                [0.0, 0.0, self.width * 20.0, self.height * 20.0],
                &DrawState::default(),
                rectangle_transform,
                gl,
            );
        }
    }

    #[inline(always)]
    pub fn update(&self, player_x: f64, player_y: f64, window_height: f64) -> BeanEvent {
        let relative_player_pos = [self.x - player_x, self.y - player_y - window_height / 2.0];
        if DEBUG_INFO {
            let x = left_justify_str(format!("{:.2}", relative_player_pos[0]), 7);
            let y = left_justify_str(format!("{:.2}", relative_player_pos[1]), 7);
            println!("Relative player pos: ({}, {})", x, y);
        }
        let a = self.y;
        let b = player_y - window_height / 2.0;
        println!("a: {}, b: {}", a, b);
        if a < b {
            if DEBUG_INFO {
                println!("Bean is off screen");
            }
            return BeanEvent::OffScreen
        } else if relative_player_pos[0] >= 0.0 && relative_player_pos[0] <= self.width * 40.0 &&
           relative_player_pos[1] >= 0.0 && relative_player_pos[1] <= self.height * 40.0 {
            return BeanEvent::Eaten
        }
        BeanEvent::None
    }
}
