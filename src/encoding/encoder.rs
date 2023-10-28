use crate::{components::math::Size2i, data_structures::quadtree::Quadtree};
use super::Encodable;



pub struct EncoderContext {
    pub size: Size2i,
}

pub struct Encoder {
    pub context: EncoderContext,
    data: Box<dyn Encodable>,
}

impl Encoder {
    pub fn encode(&self) -> Vec<u8> {
        self.data.encode(&self.context)
    }

    pub fn to_file(&self, path: &str) -> std::io::Result<()> {
        use std::fs::File;
        use std::io::Write;

        let mut file = File::create(path)?;
        file.write_all(&self.encode())?;

        Ok(())
    }

}

pub struct EncoderBuilder {
    size: Option<Size2i>,
    data: Option<Box<dyn Encodable>>,
}

impl EncoderBuilder {
    pub fn new() -> EncoderBuilder {
        EncoderBuilder {
            size: None,
            data: None,
        }
    }

    pub fn sized(mut self, size: Size2i) -> Self {
        self.size = Some(size);
        self
    }

    pub fn with_data<T: 'static + Encodable>(mut self, data: T) -> Self {
        self.data = Some(Box::new(data));
        self
    }

    pub fn build(self) -> Encoder {
        Encoder {
            context: EncoderContext {
                size: self.size.expect("Must provide a size for the encoder"),
            },
            data: self.data.expect("Must provide data for the encoder"),
        }
    }
}


impl Encodable for Vec<Vec<u8>> {

    fn encode(&self, context: &EncoderContext) -> Vec<u8> {
        // The first two bytes represent the width, and the next two bytes represent the height.
        let mut k = Vec::new();

        // Pack the height and width into the first two bytes
        k.extend_from_slice(&(context.size.width as u16).to_be_bytes());
        k.extend_from_slice(&(context.size.height as u16).to_be_bytes());

        let mut accumulator = 0u16;
        let mut bit_position = 0;

        for &val in self.iter().flatten() {
            if val == 1 {
                accumulator |= 1 << bit_position;
            }

            bit_position += 1;

            // After accumulating 16 bits, we push it to our byte array and reset
            if bit_position == 16 {
                k.extend_from_slice(&accumulator.to_ne_bytes());
                accumulator = 0;
                bit_position = 0;
            }
        }

        // If there are any remaining bits stored, when not a multiple of 16, we push them as well
        if bit_position !=0 {
            k.extend_from_slice(&accumulator.to_ne_bytes());
        }

        k
    }
}

impl Encodable for Quadtree {
    fn encode(&self, context: &EncoderContext) -> Vec<u8> {
        self.to_grid().encode(context)
    }
}

