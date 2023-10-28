pub struct EncodedFile {
    pub path: String,
    pub bytes: Option<Vec<u8>>
}

impl EncodedFile {
    pub fn new(path: &str) -> EncodedFile {
        EncodedFile {
            path: path.to_string(),
            bytes: None,
        }
    }
}

pub struct FileHandler {
    pub context: FileHandlerContext
}

pub struct FileHandlerContext {
    pub path: String,
}

impl FileHandler {
    pub fn read_bytes(&self) -> std::io::Result<Vec<u8>> {
        std::fs::read(&self.context.path)
    }

    pub fn write_bytes(&self, bytes: Vec<u8>) -> std::io::Result<()> {
        std::fs::write(&self.context.path, bytes)
    }
}
pub struct FileHandlerBuilder {
    pub path: Option<String>,
}

impl FileHandlerBuilder {
    pub fn new() -> FileHandlerBuilder {
        FileHandlerBuilder {
            path: None,
        }
    }

    pub fn with_path(mut self, path: String) -> Self {
        self.path = Some(path);
        self
    }

    pub fn build(self) -> FileHandler {
        FileHandler {
            context: FileHandlerContext {
                path: self.path.expect("Must provide a path for the file handler"),
            }
        }
    }
}
