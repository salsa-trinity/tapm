use sha3::Digest;

pub fn clean_hash(seed: &str) -> String {
    // raw hash
    let mut hasher = sha3::Sha3_512::new();
    hasher.update(&seed);
    let raw = hasher.finalize();

    // clean hash
    let mut clean = String::new();
    for byte in raw {
        // 0-255 -> 32-126
        let clean_byte = ((byte as f64 * (126 - 32) as f64 / 255f64) + 32f64).round() as u8;
        clean += &String::from(clean_byte as char);
    }
    clean
}
