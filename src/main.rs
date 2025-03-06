use clap::Parser;
use tapm::cli::args::Args;

fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    Args::parse();

    tapm::cli::cli::cli_main();
}
