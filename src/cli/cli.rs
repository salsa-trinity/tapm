pub fn cli_main() {
    let mut seed = String::new();
    std::io::stdin().read_line(&mut seed).unwrap();
    println!("password: {}", crate::api::tapm::clean_hash(&seed));
}
