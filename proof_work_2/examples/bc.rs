use proof_work_2::BlockChain;

fn main() {
    let mut block_chain = BlockChain::new_genesis_block();

    block_chain.add_block("Send 1 BTC to Ivan");
    block_chain.add_block("Send 2 more BTC to Ivan");

    for block in block_chain.blocks {
        println!("Prev. hash: {}", block.prev_block_hash);
        println!("Data: {}", block.data);
        println!("Hash: {}", block.hash);
        println!(" ");
    }
}
