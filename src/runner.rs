use std::fs;
use std::io::ErrorKind;
use std::path::PathBuf;

use crate::config::Config;

#[derive(Debug)]
pub struct Args {
    pub config_file: String,
    pub debug: bool,
    pub server_url: String,
}

impl Default for Args {
    fn default() -> Self {
        let config_file =
            Config::default_config_file().to_str().unwrap().to_string();
        Self {
            config_file,
            debug: false,
            server_url: "".to_string(),
        }
    }
}

fn invoke(mut config: Config, args: Args) -> Result<(), &'static str> {
    config.args = args;

    if !config.is_valid() {
        return Err("Usage: eloquentlog <ACTION> <OPTION>, ...");
    }
    if config.is_debug() {
        println!("debug mode: on");
    }
    println!("Hoi");
    Ok(())
}

pub fn run(args: Args) -> Result<(), &'static str> {
    if fs::create_dir_all(Config::config_home()).is_err() {
        return Err("");
    }

    let config_file = if !args.config_file.is_empty() {
        Some(PathBuf::from(&args.config_file))
    } else {
        None
    };

    match Config::load_from_local_file(config_file) {
        Ok(c) => invoke(c, args),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            // TODO: ask
            match Config::create_file() {
                Err(e) => {
                    eprintln!("err: {}", e);
                    Err("")
                }
                Ok(c) => invoke(c, args),
            }
        }
        Err(e) => {
            eprintln!("err: {}", e);
            Err("")
        }
    }
}
