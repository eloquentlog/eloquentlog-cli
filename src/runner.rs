use std::fs;
use std::io::{stdin, stdout, ErrorKind, Write};
use std::path::PathBuf;

use crate::client::Client;
use crate::config::Config;

#[derive(Debug)]
pub struct Args {
    pub config_file: String,
    pub debug: bool,
    pub server_url: String,
    pub command: String,
}

impl Default for Args {
    fn default() -> Self {
        let config_file =
            Config::default_config_file().to_str().unwrap().to_string();
        Self {
            config_file,
            debug: false,
            server_url: "".to_string(),
            command: "".to_string(),
        }
    }
}

fn invoke(action: &str, config: Config) -> Result<(), &'static str> {
    if !config.is_valid() {
        return Err("Usage: eloquentlog <ACTION> <OPTION>, ...");
    }
    if config.is_debug() {
        println!("debug mode: on");
    }

    match action {
        "get" => {
            // FIXME
            let client = Client::new(config);
            let result = client.lrange_message("dummy".to_string(), 0, 9);
            println!("res: {:#?}", result);
        }
        _ => {
            eprintln!("unknown action: {}", action);
        }
    }

    Ok(())
}

const ANSWER_YES: [&str; 4] = ["y", "Y", "yes", "Yes"];
const ANSWER_NO: [&str; 4] = ["n", "N", "no", "No"];

fn want_config_file() -> bool {
    let mut s = String::new();
    print!("Would you like to create eloquentlog.toml? [Yes/No] ");
    let _ = stdout().flush();
    stdin().read_line(&mut s).expect("Something wrong :'(");

    let v = s.trim();
    if ANSWER_NO.contains(&v) {
        false
    } else if ANSWER_YES.contains(&v) {
        true
    } else {
        eprintln!("Sorry, response '{}' not understood.", v);
        want_config_file()
    }
}

pub fn run(cmd: &str, args: Args) -> Result<(), &'static str> {
    if fs::create_dir_all(Config::config_home()).is_err() {
        return Err("");
    }

    let config_file = if !args.config_file.is_empty() {
        Some(PathBuf::from(&args.config_file))
    } else {
        None
    };

    match Config::load_from_local_file(config_file) {
        Ok(c) => invoke(cmd, c.update(args)),
        Err(e) if e.kind() == ErrorKind::NotFound => {
            if !want_config_file() {
                println!("Quitting.");
                return Ok(());
            }
            Config::create_file().map_err(|e| {
                eprintln!("err: {}", e);
                "Something wrong :'("
            })
        }
        Err(e) => {
            eprintln!("err: {}", e);
            Err("")
        }
    }
}
