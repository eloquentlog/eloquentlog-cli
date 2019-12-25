use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use dirs;
use serde::Deserialize;

use crate::runner::Args;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub credential: Credential,
    pub server: Server,
    pub user: User,

    #[serde(skip)]
    pub args: Args,
}

#[derive(Debug, Deserialize)]
pub struct Credential {
    pub api_key: String,
    pub secret: String,
}

#[derive(Debug, Deserialize)]
pub struct Server {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
}

// defaults
pub const CONFIG_FILE: &str = "eloquentlog.toml";
pub const CONFIG_ROOT: &str = "eloquentlog";

const DEFAULT_CONTENTS: &str = r#"[credential]
api_key = "<api_key>"
secret = "<secret>"

[server]
url = "https://eloquentlog.com"

[user]
name = "<username>"
"#;

impl Config {
    pub fn config_home() -> PathBuf {
        [dirs::config_dir().unwrap(), PathBuf::from(CONFIG_ROOT)]
            .iter()
            .collect()
    }

    pub fn default_config_file() -> PathBuf {
        [
            dirs::config_dir().unwrap(),
            PathBuf::from(CONFIG_ROOT),
            PathBuf::from(CONFIG_FILE),
        ]
        .iter()
        .collect()
    }

    pub fn create_file() -> Result<(), std::io::Error> {
        let config_file = Self::default_config_file();
        let mut file = File::create(&config_file)?;
        file.write_all(DEFAULT_CONTENTS.to_string().into_bytes().as_slice())?;
        file.sync_data()
    }

    pub fn load_from_local_file(
        config_file: Option<PathBuf>,
    ) -> Result<Self, std::io::Error> {
        let mut file = File::open(match config_file {
            Some(f) => f,
            None => Self::default_config_file(),
        })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // TODO
        let c: Config = toml::from_str(&contents).unwrap();
        Ok(c)
    }

    pub fn is_debug(&self) -> bool {
        self.args.debug
    }

    pub fn is_valid(&self) -> bool {
        // TODO
        true
    }
}

impl Default for Config {
    fn default() -> Self {
        let credential = Credential {
            api_key: "".to_string(),
            secret: "".to_string(),
        };

        let server = Server {
            url: "https://eloquentlog.com/".to_string(),
        };

        let user = User {
            name: "".to_string(),
        };

        Self {
            credential,
            server,
            user,
            args: Args {
                ..Default::default()
            },
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_debug() {
        let tests = vec![
            (
                Args {
                    debug: false,
                    ..Default::default()
                },
                false,
            ),
            (
                Args {
                    debug: true,
                    ..Default::default()
                },
                true,
            ),
        ];
        for (args, want) in tests.into_iter() {
            let config = Config {
                args,
                ..Default::default()
            };
            assert_eq!(config.is_debug(), want);
        }
    }

    #[test]
    fn test_is_valid() {
        let config = Config {
            ..Default::default()
        };
        assert_eq!(config.is_valid(), true);
    }
}
