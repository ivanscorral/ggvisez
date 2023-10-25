
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn new(x: i16, y: i16) -> Point {
        Point { x, y }
    }

}



// Implement the `From` trait to easily convert (i16, i16) to `GridPosition`.
impl From<(i16, i16)> for Point {
    fn from(pos: (i16, i16)) -> Self {
        Point { x: pos.0, y: pos.1 }
    }
}

impl From<(i32, i32)> for Point {
    fn from(pos: (i32, i32)) -> Self {
        Point { x: pos.0 as i16, y: pos.1 as i16 }
    }
}
