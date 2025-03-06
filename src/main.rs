use clap::Parser;
use tapm::cli::args::Args;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    Args::parse();

    let mut seed = String::new();
    std::io::stdin().read_line(&mut seed).unwrap();
    println!("password: {}", tapm::api::tapm::clean_hash(&seed));
}
