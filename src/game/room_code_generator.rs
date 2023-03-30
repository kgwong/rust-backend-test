use block_id::{Alphabet, BlockId};

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct RoomCodeGenerator {
    seed: u128,
    count: usize,
    length: u8
}

impl RoomCodeGenerator {
    pub fn new(length: u8) -> Self {
        let start = SystemTime::now();
        let timestamp = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as u128;
        RoomCodeGenerator{
            seed: timestamp,
            count: 0,
            length: length,
        }
    }

    pub fn generate(&mut self) -> String {
        let generator = BlockId::new(
            Alphabet::lowercase_alpha(), self.seed, self.length);

        self.count += 1;

        return generator.encode_string(self.count as u64).to_ascii_uppercase();
    }
}