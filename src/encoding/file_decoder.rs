use crate::{io::files::FileHandler, components::visuals::Point};

use super::decoder::Decoder;

pub struct FileDecoder {
    file_handler: FileHandler,
    decoder: Option<Decoder<Vec<u8>>>,
}

impl FileDecoder {
    // Constructor: Only requires input path
    pub fn new(input_path: &String) -> Self {
        FileDecoder {
            file_handler: FileHandler::new(input_path),
            decoder: None,
        }
    }
    pub fn decode_file(&self) -> std::io::Result<Vec<Point>> {
        // Read bytes from the file
        let bytes = self.file_handler.read_bytes()?;

        // Create a new Decoder with the bytes
        let mut decoder = Decoder::new(bytes);

        // Decode the bytes and return the result
        decoder.decode().ok_or(std::io::Error::new(std::io::ErrorKind::Other, "Decoding error"))
    }

}
