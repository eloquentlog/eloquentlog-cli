//! A command line interface for Eloquentlog.
//!
//! # Examples
//!
//! ```zsh
//! % eloquentlog --help
//! ```
extern crate structopt;

use structopt::StructOpt;

mod config;
use config::Config;

/// command options.
#[derive(Debug, StructOpt)]
#[structopt(
    name = "eloquentlog",
    about = "\nA command line interface for Eloquentlog."
)]
pub struct Opts {
    /// Run debug mode
    #[structopt(short, long)]
    debug: bool,
}

impl Default for Opts {
    fn default() -> Opts {
        Opts { debug: false }
    }
}

fn main() {
    let opts = Opts::from_args();
    let c = Config::new(opts);
    if !c.is_valid() {
        eprintln!("Usage: eloquentlog <ACTION> <OPTION>, ...");
        std::process::exit(1);
    }

    if c.is_debug() {
        println!("debug mode: on");
    }

    println!("Hoi");
}
