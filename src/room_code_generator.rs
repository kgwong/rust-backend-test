use block_id::{Alphabet, BlockId};

use std::time::{SystemTime, UNIX_EPOCH};


static mut count: u64 = 0;

pub fn generate_room_code() -> String {
    let seed = 0; // TODO: seed current time
    let length = 4;
    
    let generator = BlockId::new(
        Alphabet::lowercase_alpha(), seed, length);
    
    // TODO
    unsafe { count += 1 };

    return generator.encode_string(unsafe { count } );
}