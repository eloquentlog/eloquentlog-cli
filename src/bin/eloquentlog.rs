//! A command line interface for Eloquentlog.
//!
//! # Examples
//!
//! ```zsh
//! % eloquentlog --help
//! ```
extern crate structopt;

use structopt::StructOpt;

use eloquentlog_cli::config;
use eloquentlog_cli::runner;

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

impl From<Opts> for runner::Args {
    fn from(item: Opts) -> Self {
        Self {
            debug: item.debug,
            config_file: item.config_file,
        }
    }
}

fn main() {
    let args = runner::Args::from(Opts::from_args());
    std::process::exit(match runner::run(args) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{:?}", e);
            1
        }
    });
}
