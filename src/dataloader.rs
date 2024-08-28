use std::{fs::File, io::Read, mem::{size_of, transmute}};
use crate::{arch::BATCH_SIZE, types::datapoint::Datapoint};

pub struct Loader {
    batch: [Datapoint; BATCH_SIZE],
    current: usize,
    file: File,
}

impl Loader {
    pub fn new() -> Self {
        Self {
            current: BATCH_SIZE,  // This ensures load_batch() is called on first get_position()
            batch: [Datapoint::new(); BATCH_SIZE],
            file: File::open("test-data.bin").expect("Failed to open file")
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
                    // Fill the rest of the batch with new Datapoints
                    *datapoint = Datapoint::new();
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