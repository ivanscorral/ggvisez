use crate::{io::files::{FileHandler, FileHandlerBuilder}, components::visuals::Point};

use super::decoder::Decoder;

pub struct FileDecoder {
    file_handler: FileHandler,
    decoder: Option<Decoder<Vec<u8>>>,
}

impl FileDecoder {
    // Constructor: Only requires input path
    pub fn new(input_path: &str) -> Self {
        FileDecoder {
            file_handler: FileHandlerBuilder::new().with_path(input_path.to_string()).build(),
            decoder: None,
        }
    }
    pub fn decode_file(&mut self) -> std::io::Result<Vec<Point>> {
        // Read bytes from the file
        let bytes = self.file_handler.read_bytes()?;

        // Create a new Decoder with the bytes
        let mut decoder = Decoder::new(bytes);

        // Decode the bytes and return the result
        decoder.decode().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Decoding error"))
    }

    pub fn write_decoded_to_file(&self, output_path: &str, decoded_content: &[u8]) -> std::io::Result<()> {
        // Instead of mutating the existing file_handler, create a new FileHandler with the desired path
        let output_handler = FileHandlerBuilder::new().with_path(output_path.to_string()).build();
        output_handler.write_bytes(decoded_content.to_vec())
    }
}
