//! A command line interface for Eloquentlog.
//!
//! # Examples
//!
//! ```zsh
//! % eloquentlog --help
//! ```
extern crate dirs;
extern crate serde;
extern crate structopt;
extern crate toml;

use structopt::StructOpt;

mod config;
mod runner;

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

    /// Path to config file
    #[structopt(short, long, default_value = "")]
    config_file: String,
}

impl Default for Opts {
    fn default() -> Opts {
        let config_file =
            Config::default_config_file().to_str().unwrap().to_string();
        Opts {
            debug: false,
            config_file,
        }
    }
}

fn main() {
    let opts = Opts::from_args();
    std::process::exit(match runner::run(opts) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{:?}", e);
            1
        }
    });
}
