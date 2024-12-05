fn main() {
    let mut api = tapm::api::Api::new();

    let mut flags: Vec<String> = std::env::args().collect();
    if flags.len() != 1 {
        api.process_flags(&flags);
    }

    let mpass = rpassword::prompt_password("Master: ").unwrap();
    let site = rpassword::prompt_password("Site: ").unwrap();
    let id = rpassword::prompt_password("ID(opt): ").unwrap();

    if flags.len() == 1 {
        let _flags = rpassword::prompt_password("Flags(opt): ").unwrap();
        for word in _flags.split_whitespace() {
            flags.push(word.to_string());
        }
    }

    api.main(&mpass, &site, &id, &flags);
}
