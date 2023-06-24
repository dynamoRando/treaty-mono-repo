use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub fn hash(
    passwd: &str,
) -> (
    String,
    sodiumoxide::crypto::pwhash::argon2id13::HashedPassword,
) {
    sodiumoxide::init().unwrap();
    let hash = sodiumoxide::crypto::pwhash::argon2id13::pwhash(
        passwd.as_bytes(),
        sodiumoxide::crypto::pwhash::argon2id13::OPSLIMIT_INTERACTIVE,
        sodiumoxide::crypto::pwhash::argon2id13::MEMLIMIT_INTERACTIVE,
    )
    .unwrap();
    let texthash = std::str::from_utf8(&hash.0).unwrap().to_string();
    (texthash, hash)
}

pub fn verify(hash: [u8; 128], passwd: &str) -> bool {
    sodiumoxide::init().unwrap();
    match sodiumoxide::crypto::pwhash::argon2id13::HashedPassword::from_slice(&hash) {
        Some(hp) => sodiumoxide::crypto::pwhash::argon2id13::pwhash_verify(&hp, passwd.as_bytes()),
        _ => false,
    }
}

pub fn calculate_hash_for_struct<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
