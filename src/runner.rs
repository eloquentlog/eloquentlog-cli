use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::config::Config;
use crate::Opts;

fn invoke(mut config: Config, opts: Opts) -> Result<(), &'static str> {
    config.opts = opts;

    if !config.is_valid() {
        return Err("Usage: eloquentlog <ACTION> <OPTION>, ...");
    }
    if config.is_debug() {
        println!("debug mode: on");
    }
    println!("Hoi");
    Ok(())
}

pub fn run(opts: Opts) -> Result<(), &'static str> {
    if fs::create_dir_all(Config::config_home()).is_err() {
        return Err("");
    }

    let config_file = if !opts.config_file.is_empty() {
        Some(PathBuf::from(&opts.config_file))
    } else {
        None
    };

    match Config::load_from_local_file(config_file) {
        Ok(c) => invoke(c, opts),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // TODO: ask
            match Config::create_file() {
                Err(e) => {
                    eprintln!("err: {}", e);
                    Err("")
                }
                Ok(c) => invoke(c, opts),
            }
        }
        Err(e) => {
            eprintln!("err: {}", e);
            Err("")
        }
    }
}
