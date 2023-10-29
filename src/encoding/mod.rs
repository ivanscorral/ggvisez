use crate::{components::{visuals::Point, math::Size2i}, io::files::EncodedFile};


pub mod encoder;
pub mod decoder;
pub mod rle;


pub enum Compressor {
    Rle,
}

impl Compressor {
    fn compress(&self, data: &Vec<u8>) -> Vec<u8> {
        match self {
            Compressor::Rle => rle::encode(data),
        }
    }

}

pub trait Encodable {
    fn encode(&self, size: &Size2i) -> Vec<u8>;
}

pub trait Decodable {
    fn decode(&self) -> Vec<Point>;
}

pub fn encode<T: Encodable>(input: &T, size: &Size2i) -> Vec<u8> {
    input.encode(size)
}

pub fn compress(bytes: &Vec<u8>, compressor: Compressor) -> Vec<u8> {
    compressor.compress(&bytes)
}
