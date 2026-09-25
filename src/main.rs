#![feature(associated_type_defaults)]

pub mod cli;
use cli::Cli;

pub mod tui;
pub mod network;
pub mod posts;
pub mod data;

fn main() {
    let cli = Cli::from_args();
    match cli.process() {
        Err(e) => eprintln!("{e}"),
        _ => {}
    }
}
