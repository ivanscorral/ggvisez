pub struct EncodedFile {
    file_handler: FileHandler,
}

impl EncodedFile {
    pub fn new(path: &String) -> Self {
        let file_handler = FileHandler::new(path);
        Self { file_handler }
    }

    pub fn bytes(&self) -> Vec<u8> {
        match self.file_handler.read_bytes() {
            Ok(bytes) => bytes.clone(),
            Err(e) => {
                println!("Error reading bytes: {}", e);
                vec![]
            }
        }
    }

}

pub struct FileHandler {
    pub path: String,
}



impl FileHandler {
    pub fn new(path: &String) -> FileHandler {
        FileHandler { path: path.to_string() }
    }

    pub fn read_bytes(&self) -> std::io::Result<Vec<u8>> {
        std::fs::read(&self.path)
    }

    pub fn write_bytes(&self, bytes: &Vec<u8>) -> std::io::Result<()> {
        std::fs::write(&self.path, bytes)
    }
}
