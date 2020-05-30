//! A command line interface for Eloquentlog.
//!
//! # Examples
//!
//! ```zsh
//! % eloquentlog --help
//! ```
extern crate clap;
extern crate structopt;

use std::env;
use std::io::{self, Write};

use clap::App;
use structopt::StructOpt;

use libeloquentlog::config;
use libeloquentlog::runner;

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

impl Opts {
    pub fn print_help(self, app: &mut App) {
        if app.print_help().is_ok() {
            let mut out = io::stdout();
            let _ = writeln!(out);
        }
    }
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

impl From<&Opts> for runner::Args {
    fn from(opts: &Opts) -> Self {
        Self {
            config_file: opts.config_file.to_owned(),
            debug: opts.debug,
            server_url: opts.server_url.to_owned(),
            command: "".to_string(),
        }
    }
}

const COMMANDS: [&str; 1] = ["get"];

fn main() {
    let mut app = Opts::clap();

    let v_args: Vec<_> = env::args().collect();

    // get only global args start with -
    let opts =
        Opts::from_iter(&mut v_args.iter().filter(|s| s.starts_with('-')));
    let args = runner::Args::from(&opts);

    // cmd
    let c = v_args.iter().skip(1).find(|x| !x.starts_with('-'));
    if c.is_none() {
        opts.print_help(&mut app);
        return;
    }
    let cmd = c.unwrap();
    if !COMMANDS.contains(&cmd.as_str()) {
        eprintln!("no such command: {}", cmd);
        opts.print_help(&mut app);
        return;
    }

    std::process::exit(match runner::run(&cmd.as_str(), args) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{:?}", e);
            1
        }
    });
}
