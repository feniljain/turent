use std::io::Write;

use bytes::{Bytes, BytesMut};

use crate::errors::ClientError;

pub struct File {
    path: String,
    name: String,
    bytes: BytesMut,
}

impl File {
    pub fn new(path: String, name: String) -> Self {
        Self {
            path,
            name,
            bytes: BytesMut::new(),
        }
    }

    pub fn chunkify(&self) -> Result<Vec<Vec<u8>>, ClientError> {
        let full_path = self.path.clone() + "/" + self.name.as_str();
        match std::fs::read(full_path.as_str()) {
            Ok(bytes) => {
                println!("File Size: {:?}", bytes.len());
                let chunks: Vec<Vec<u8>>;
                // Chunks of 5MB = 5 * 10,00,000 bytes
                chunks = bytes.clone().chunks(5000000).map(|x| x.to_vec()).collect();
                return Ok(chunks);
            }
            Err(err) => Err(ClientError::ErrReadingFile(err.to_string())),
        }
    }

    pub fn build_file(&self) -> Result<(), ClientError> {
        let mut file = std::fs::File::create("foo.txt")
            .map_err(|err| ClientError::ErrWritingFile(err.to_string()))?;

        file.write_all(&self.bytes)
            .map_err(|err| ClientError::ErrWritingFile(err.to_string()))?;

        println!("File Written!");

        Ok(())
    }

    pub fn append_bytes(&mut self, bytes: Bytes) {
        self.bytes.extend_from_slice(&bytes);
    }
}
