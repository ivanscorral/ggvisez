use self::files::{FileHandler, EncodedFile};

pub mod files;

pub fn write_bytes_to_file(filename: &str, bytes: &Vec<u8>) {
    let file_handler = FileHandler::new(&filename.to_string());
    file_handler.write_bytes(bytes);
}

pub fn write_encoded_file(filename: &str, bytes: &Vec<u8>) -> Result<EncodedFile, std::io::Error> {
    let file_handler = FileHandler::new(&filename.to_string());
    match file_handler.write_bytes(bytes) {
        Ok(_) => Ok(EncodedFile::new(&filename.to_string())),
        Err(e) => Err(e),
    }
}
