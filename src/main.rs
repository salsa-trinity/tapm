fn main() {
    loop {
        let mut seed = String::new();
        std::io::stdin().read_line(&mut seed).unwrap();
        println!("hash: {}", tapm::api::tapm::clean_hash(&seed));
        println!("");
    }
}
