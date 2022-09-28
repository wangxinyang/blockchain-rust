mod blocks;
mod utils;

pub use blocks::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_proofwork_new() {
        let mut block = Block::new("Genesis Block", "");
        let proofwork = ProofWork::new();
        proofwork.run(&mut block);
        println!("proof: {:?}", proofwork);
    }
}
