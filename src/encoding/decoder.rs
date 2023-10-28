use ggez::input;

use crate::{components::{visuals::Point}, io::files::{EncodedFile, FileHandlerBuilder}};

use super::Decodable;
pub struct Decoder<T: Decodable> {
    input: Option<T>,
    cached_data: Option<Vec<Point>>,
}

impl<T: Decodable> Decoder<T> {
    pub fn new(input: T) -> Self {
        Decoder {
            input: Some(input),
            cached_data: None,
        }
    }

    pub fn decode(&mut self) -> Option<Vec<Point>> {
        if let Some(cached_data) = &self.cached_data {
            return Some(cached_data.clone());
        }

        if let Some(input) = &self.input {
            let decoded_data = input.decode();
            self.cached_data = Some(decoded_data.clone());
            return Some(decoded_data);
        }

        None
    }

    pub fn is_cached(&self) -> bool {
        self.cached_data.is_some()
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
                        if   x < width as usize && y < height as usize {
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
            // Create a file handler
            let file_handler = FileHandlerBuilder::new()
                .with_path(self.path.clone())
                .build();
            // Get the file's bytes, handing possible errors
            let bytes = match file_handler.read_bytes() {
                Ok(bytes) => bytes,
                Err(err) => {
                    panic!("Error reading file: {}", err);
                }
            };
            // Decode the bytes
            let decoded_data = bytes.decode();
            decoded_data
        }
    }




pub struct DecoderState {
    //
    pub data: Option<Vec<Point>>,
}
