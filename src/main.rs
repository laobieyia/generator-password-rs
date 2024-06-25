// use crate::;
use anyhow::{bail, Error, Ok, Result};
use clap::Parser;
use password_generator_rs::generator_password;
#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    seed: String,
    #[clap(short, long, default_value_t = 15)]
    length: usize,
}

fn main() -> Result<(), Error> {
    let args = Args::parse();
    let (seed, length) = (args.seed, args.length);
  
    let password = generator_password(&seed, length)?;
    println!("seed: {},password: {}", seed, password);
    Ok(())
}
