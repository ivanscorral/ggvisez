use crate::components::{visuals::Point, math::Size2i};


pub mod encoder;
pub mod decoder;
pub mod rle;
pub trait Encodable {
    fn encode(&self, size: &Size2i) -> Vec<u8>;
}

pub trait Decodable {
    fn decode(&self) -> Vec<Point>;
}


