use std::ops::{Sub, Div};

use rand::Rng;

pub trait RandomGen<T> {
    fn gen_range(min: T, max_x: T, max_y: T) -> Self;
}


#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Size2f {
    pub width: f32,
    pub height: f32,
}

impl Size2f {
    pub const fn new(width: f32, height: f32) -> Size2f {
        Size2f { width, height }
    }
}
// Implement substraction of numerical values to size
impl Sub for Size2f {
    type Output = Size2f;
    fn sub(self, rhs: Size2f) -> Self::Output {
        Size2f {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl RandomGen<f32> for Size2f {
    fn gen_range(min: f32, max_x: f32, max_y: f32) -> Self {
        let mut rng = rand::thread_rng();
        Size2f {
            width: rng.gen_range(min..max_x),
            height: rng.gen_range(min..max_y),
        }
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Size2i {
    pub width: i16,
    pub height: i16,
}

impl Div<i16> for Size2i {
    type Output = Size2i;
    fn div(self, rhs: i16) -> Self::Output {
        Size2i {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl RandomGen<i16> for Size2i {
    fn gen_range(min: i16, max_x: i16, max_y: i16) -> Self {
        let mut rng = rand::thread_rng();
        Size2i {
            width: rng.gen_range(min..max_x),
            height: rng.gen_range(min..max_y),
        }
    }
}

impl Size2i {
    pub const fn new(width: i16, height: i16) -> Size2i {
        Size2i { width, height }
    }
}
