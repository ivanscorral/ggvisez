
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }

    pub fn rand(max_x: u32, max_y: u32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..=max_x),
            y: rng.gen_range(0..=max_y),
        }
    }
}

// Implement the `From` trait to easily convert (u32, u16) to `Point`.
impl From<(u32, u32)> for Point {
    fn from(pos: (u32, u32)) -> Self {
        Point { x: pos.0, y: pos.1 }
    }
}

impl From<(u16, u16)> for Point {
    fn from(pos: (u16, u16)) -> Self {
        Point { x: pos.0 as u32, y: pos.1 as u32 }
    }
}
