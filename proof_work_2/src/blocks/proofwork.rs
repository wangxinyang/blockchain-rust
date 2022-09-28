use std::ops::Shl;

use crate::utils::{hash_to_str, hash_to_u8, serialize};

use super::block::Block;
use primitive_types::U256;

const MAX_NONCE: usize = usize::MAX;
const TARGET_BITS: usize = 8;

#[derive(Debug)]
pub struct ProofWork {
    target: U256,
}

impl ProofWork {
    pub fn new() -> Self {
        let mut target = U256::from(1);
        target = target.shl(256 - TARGET_BITS);
        Self { target }
    }

    pub fn run(&self, block: &mut Block) {
        let mut nonce = 0;

        while nonce < MAX_NONCE {
            let join_str = format!(
                "{}{}{}{}{}",
                block.prev_block_hash, block.data, block.timestamp, TARGET_BITS, nonce
            );
            let pre_hash = serialize(&join_str);
            let mut hash_u = [0; 32];
            hash_to_u8(&pre_hash, &mut hash_u);
            let pre_hash_int = U256::from(hash_u);
            println!("pre_hash_int is : {}", pre_hash_int);
            println!("target is : {}", &self.target);
            if pre_hash_int.lt(&self.target) {
                block.set_hash(hash_to_str(&pre_hash));
                break;
            } else {
                nonce += 1;
            }
        }
    }
}

impl Default for ProofWork {
    fn default() -> Self {
        Self::new()
    }
}
