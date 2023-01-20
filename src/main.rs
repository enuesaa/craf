use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[clap(short = 'a', long = "aaa", default_value_t = String::from("aaa"))]
    aaa: String,
}

fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}