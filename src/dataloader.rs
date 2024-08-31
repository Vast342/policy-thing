use std::{fs::File, io::{Read, Seek, SeekFrom}, mem::{size_of, transmute}};
use crate::{arch::BATCH_SIZE, types::datapoint::Datapoint};

pub struct Loader {
    pub batch: Box<[Datapoint; BATCH_SIZE]>,
    current: usize,
    file: File,
    pub file_size: u64,
}

impl Loader {
    pub fn new() -> Self {
        let mut file = File::open("test-data.bin").expect("Failed to open file");
        let file_size = file.seek(SeekFrom::End(0)).expect("Failed to get file size");
        file.seek(SeekFrom::Start(0)).expect("Failed to reset file position");
        
        Self {
            current: BATCH_SIZE, // This ensures load_batch() is called on first get_position()
            batch: Box::new([Datapoint::new(); BATCH_SIZE]),
            file,
            file_size,
        }
    }

    pub fn load_batch(&mut self) {
        self.current = 0;
        for datapoint in self.batch.iter_mut() {
            let mut buffer = [0u8; size_of::<Datapoint>()];
            match self.file.read_exact(&mut buffer) {
                Ok(_) => {
                    // SAFETY: This assumes that the byte representation in the file
                    // exactly matches the in-memory representation of Datapoint
                    *datapoint = unsafe { transmute(buffer) };
                },
                Err(_) => {
                    println!("epoch done");
                    self.file.seek(SeekFrom::Start(0)).expect("Failed to reset file position");
                    self.file.read_exact(&mut buffer).expect("Failed to read after reset");
                    *datapoint = unsafe { transmute(buffer) };
                }
            }
        }
    }

    pub fn get_position(&mut self) -> Datapoint {
        if self.current == BATCH_SIZE {
            self.load_batch();
        }
        let datapoint = self.batch[self.current];
        self.current += 1;
        datapoint
    }
}