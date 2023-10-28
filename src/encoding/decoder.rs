use crate::{
    components::visuals::Point,
    io::files::EncodedFile,
};

use super::Decodable;
pub struct Decoder<T: Decodable> {
    input: T,
    cached_data: Option<Vec<Point>>,
}

impl<T: Decodable> Decoder<T> {
    pub fn new(input: T) -> Self {
        Decoder {
            input,
            cached_data: None,
        }
    }

    pub fn decode(&mut self) -> Option<Vec<Point>> {
        if let Some(cached) = &self.cached_data {
            return Some(cached.clone());
        }

        let data = self.input.decode();
        self.cached_data = Some(data.clone());
        Some(data)
    }
}

// Implement decodable for File

impl Decodable for Vec<u8> {
    fn decode(&self) -> Vec<Point> {
        let width = u16::from_be_bytes([self[0], self[1]]);
        let height = u16::from_be_bytes([self[2], self[3]]);

        let encoded_data = &self[4..];

        let mut points = Vec::new();

        for (i, chunk) in encoded_data.chunks(2).enumerate() {
            let data = u16::from_ne_bytes([chunk[0], chunk[1]]);

            // Check each bit of the u16

            for j in 0..16 {
                if (data & (1 << j)) != 0 {
                    let flat_position = i * 16 + j;
                    let x = flat_position % width as usize;
                    let y = flat_position / width as usize;

                    // Ensure we don't go beyond the grid's dimensions
                    if x < width as usize && y < height as usize {
                        points.push(Point::new(x as u32, y as u32));
                    }
                }
            }
        }
        points
    }
}

impl Decodable for EncodedFile {
    fn decode(&self) -> Vec<Point> {
        let bytes = self.bytes();
        if bytes.len() == 0 {
            println!("Error reading file bytes");
            return vec![];
        }
        bytes.decode()
    }
}
