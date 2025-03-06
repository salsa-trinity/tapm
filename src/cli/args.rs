use clap::{Parser, Subcommand};

/// The Amnesic Password Manager
#[derive(Parser, Debug)]
#[clap(version)]
pub struct Args {
    /// test flag
    hello: Option<i32>,
}
