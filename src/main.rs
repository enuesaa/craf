use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(Debug, Subcommand)]
enum Action {
   Add,
   Remove,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}