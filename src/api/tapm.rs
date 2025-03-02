use sha3::Digest;

pub fn raw_hash(seed: &str) -> String {
    let mut hasher = sha3::Sha3_512::new();
    hasher.update(seed);
    String::from_utf8_lossy(&hasher.finalize()).to_string()
}

pub fn clean_hash(seed: &str) -> String {
    let raw = raw_hash(&seed);
    let mut clean = String::new();

    for byte in raw.as_bytes() {
        // byte within range 32-126
        if *byte > 31 && *byte < 127 {
            clean += &String::from(*byte as char);
        }
    }
    clean
}
