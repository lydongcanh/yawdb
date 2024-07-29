use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::Path;

pub struct Storage {
    file: File,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(path)
            .expect("Unable to open file");
        Storage { file }
    }

    pub fn write(&mut self, offset: u64, data: &[u8]) {
        self.file.seek(SeekFrom::Start(offset)).expect("Unable to seek");
        self.file.write_all(data).expect("Unable to write data");
    }

    pub fn read(&mut self, offset: u64, size: usize) -> Vec<u8> {
        self.file.seek(SeekFrom::Start(offset)).expect("Unable to seek");
        let mut buffer = vec![0; size];
        self.file.read_exact(&mut buffer).expect("Unable to read data");
        buffer
    }
    
    pub fn append(&mut self, data: &[u8]) {
        self.file.seek(SeekFrom::End(0)).expect("Unable to seek");
        self.file.write_all(data).expect("Unable to write data");
    }
}
