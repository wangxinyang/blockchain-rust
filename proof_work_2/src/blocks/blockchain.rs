use super::block::Block;
use crate::ProofWork;

pub struct BlockChain {
    pub blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new_genesis_block() -> Self {
        let mut block_chain = vec![];
        let mut block = Block::new("Genesis Block", "");
        let proof_work = ProofWork::new();
        proof_work.run(&mut block);
        block_chain.push(block);
        Self {
            blocks: block_chain,
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block = self.blocks.last().unwrap();
        let mut new_block = Block::new(data, &prev_block.hash);
        let proof_work = ProofWork::new();
        proof_work.run(&mut new_block);
        self.blocks.push(new_block)
    }
}
