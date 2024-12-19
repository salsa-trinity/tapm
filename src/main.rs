fn main() {
    let mut api = tapm::api::Api::new();

    let flags: Vec<String> = std::env::args().collect();
    api.process_flags(&flags);

    let mpass: String;
    let mut site = String::new();
    let mut id = String::new();

    if api.flag_s {
        mpass = rpassword::prompt_password("Single: ").unwrap();
    } else {
        mpass = rpassword::prompt_password("Master: ").unwrap();
        site = rpassword::prompt_password("Site: ").unwrap();
        id = rpassword::prompt_password("ID(opt): ").unwrap();
    }

    api.main(&mpass, &site, &id, &flags);
}
