extern crate core;

use std::path::Path;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    rom: String,
}

fn main() -> Result<(), String> {
    let args = Args::parse();

    let path = Path::new(args.rom.as_str());
    println!("Loading {}...", path.display());

    Ok(())
}
