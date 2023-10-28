use crate::components::visuals::Point;

use self::encoder::EncoderContext;

pub mod encoder;
pub mod decoder;
pub mod file_decoder;
pub trait Encodable {
    fn encode(&self, context: &EncoderContext) -> Vec<u8>;
}

pub trait Decodable {
    fn decode(&self) -> Vec<Point>;
}


