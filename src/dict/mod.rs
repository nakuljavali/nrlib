use std::hash::{Hash, SipHasher, Hasher};
use std::ops::Rem;

// PreHash
struct KV {
    key: i32,
    value: i32,
}

impl Hash for KV {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
    }
}

fn hash<KV: Hash>(t: &KV) -> u64 {
    let mut s = SipHasher::new();
    t.hash(&mut s);
    s.finish()
}

// Hash
pub fn hash_mod (hash: u64) -> u8 {
    hash.rem(7) as u8
}

#[allow(dead_code)]
struct Dict {
    name: i32,
}


pub fn hello() -> String {
    let p = KV {key: 5, value: 6};
    let z = hash(&p); 
    hash_mod(z).to_string()
}