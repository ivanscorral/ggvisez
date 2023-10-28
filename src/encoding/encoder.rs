use super::Encodable;
use crate::{
    components::math::Size2i,
    data_structures::quadtree::Quadtree,
    io::files::{EncodedFile, FileHandler},
};

pub struct Encoder<T: Encodable> {
    pub size: Size2i,
    data: T,
}

impl<T: Encodable> Encoder<T> {
    pub fn new(size: Size2i, data: T) -> Self {
        Encoder { size, data }
    }

    pub fn encode(&self) -> Vec<u8> {
        self.data.encode(&self.size)
    }

    pub fn to_file(&self, path: &String) -> std::io::Result<EncodedFile> {
        let bytes = self.encode();
        let file_handler = FileHandler::new(path);
        file_handler.write_bytes(&bytes).map(|_| EncodedFile::new(path))
    }
}

impl Encodable for Vec<Vec<u8>> {
    fn encode(&self, size: &Size2i) -> Vec<u8> {
        // The first two bytes represent the width, and the next two bytes represent the height.
        let mut encoded: Vec<u8> = Vec::new();

        encoded.extend_from_slice(&size.width.to_be_bytes());
        encoded.extend_from_slice(&size.height.to_be_bytes());

        let mut accumulator = 0u16;
        let mut bit_position = 0;

        for &val in self.iter().flatten() {
            if val == 1 {
                accumulator |= 1 << bit_position;
            }

            bit_position += 1;

            // After accumulating 16 bits, we push it to our byte array and reset
            if bit_position == 16 {
                encoded.extend_from_slice(&accumulator.to_ne_bytes());
                accumulator = 0;
                bit_position = 0;
            }
        }

        // If there are any remaining bits stored, when not a multiple of 16, we push them as well
        if bit_position != 0 {
            encoded.extend_from_slice(&accumulator.to_ne_bytes());
        }

        encoded
    }
}

impl Encodable for Quadtree {
    fn encode(&self, size: &Size2i) -> Vec<u8> {
        self.to_grid().encode(size)
    }
}
