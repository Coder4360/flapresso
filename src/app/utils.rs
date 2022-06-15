const PI: f64 = std::f64::consts::PI;

pub fn clamp<T>(value: T, min: T, max: T) -> T
    where T: PartialOrd 
{
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn min<T>(value: T, min: T) -> T
    where T: PartialOrd 
{
    if value < min {
        value
    } else {
        min
    }
}

pub fn to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub enum Direction {
    Left,
    Right,
}

impl PartialEq for Direction {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Direction::Left, Direction::Left) => true,
            (Direction::Right, Direction::Right) => true,
            _ => false,
        }
    }
}

impl Clone for Direction {
    fn clone(&self) -> Direction {
        match self {
            Direction::Left => Direction::Left,
            Direction::Right => Direction::Right,
        }
    }
}

impl Copy for Direction {}

pub fn left_justify_str(string: String, width: usize) -> String {
    if string.len() >= width {
        string
    } else {
        let mut result = String::with_capacity(width);
        result.push_str(&" ".repeat(width - string.len()));
        result.push_str(&string);
        result
    }
}