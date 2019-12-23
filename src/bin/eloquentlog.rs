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
    /// Path to config file
    #[structopt(short, long, default_value = "")]
    config_file: String,

    /// Run debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Server URL
    #[structopt(short, long, default_value = "")]
    server_url: String,
}

impl Default for Opts {
    fn default() -> Opts {
        let config_file =
            Config::default_config_file().to_str().unwrap().to_string();
        Opts {
            config_file,
            debug: false,
            server_url: "".to_string(),
        }
    }
}

impl From<Opts> for runner::Args {
    fn from(item: Opts) -> Self {
        Self {
            config_file: item.config_file,
            debug: item.debug,
            server_url: item.server_url,
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
