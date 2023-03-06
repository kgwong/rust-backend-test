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
        RoomCodeGenerator{
            seed: 0, // TODO: seed current time
            count: 0,
            length: length,
        }
    }

    pub fn generate(&mut self) -> String {
        let generator = BlockId::new(
            Alphabet::lowercase_alpha(), self.seed, self.length);

        self.count += 1;

        return generator.encode_string(self.count as u64);
    }
}