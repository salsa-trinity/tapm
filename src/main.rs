use clap::Parser;
use tapm::cli::args::Args;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let args = Args::parse();
    println!("{:?}", args);
    loop {
        let mut seed = String::new();
        std::io::stdin().read_line(&mut seed).unwrap();
        println!("hash: {}", tapm::api::tapm::clean_hash(&seed));
        println!("");
    }
}
