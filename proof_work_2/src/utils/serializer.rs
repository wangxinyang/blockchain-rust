use crypto::{digest::Digest, sha3::Sha3};

pub fn serialize<T>(data: &T) -> Vec<u8>
where
    T: ?Sized + serde::Serialize,
{
    bincode::serialize(data).unwrap()
}

pub fn hash_to_str(data: &[u8]) -> String {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result_str()
}

pub fn hash_to_u8(data: &[u8], out: &mut [u8]) {
    let mut hasher = Sha3::sha3_256();
    hasher.input(data);
    hasher.result(out);
}
