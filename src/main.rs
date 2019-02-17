//! A command line interface for Eloquentlog.
//!
//! # Examples
//!
//! ```zsh
//! % el --help
//! ```
extern crate structopt;

use structopt::StructOpt;

mod config;
use config::Config;

/// command options.
#[derive(Debug, StructOpt)]
#[structopt(name = "el", about = "\nA command line interface for Eloquentlog.")]
pub struct Opts {}

fn main() {
    let opts = Opts::from_args();
    let c = Config::new(opts);
    if !c.is_valid() {
        eprintln!("Usage: el <ACTION> <OPTION>, ...");
        std::process::exit(1);
    }

    println!("Hoi");
}
