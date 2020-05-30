use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use serde::Deserialize;

use crate::runner::Args;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub credential: Credential,
    pub server: Server,
    pub user: User,
    pub log: Log,

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

#[derive(Debug, Deserialize)]
pub struct Log {
    pub severity: String,
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

[log]
severity = "warn"
"#;

fn format_url(url: String) -> String {
    if url.ends_with('/') {
        return url[..(url.len() - 1)].to_string();
    }
    url
}

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

        let mut c: Config = toml::from_str(&contents).unwrap();
        c.server.url = format_url(c.server.url);
        Ok(c)
    }

    /// apply fields updates via args.
    pub fn update(mut self, args: Args) -> Self {
        if !args.server_url.is_empty() {
            self.server.url = format_url(args.server_url);
        }
        self.log.severity =
            (if args.debug { "debug" } else { "warn" }).to_string();
        self
    }

    pub fn is_debug(&self) -> bool {
        self.log.severity == "debug"
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

        let log = Log {
            severity: "warn".to_string(),
        };

        Self {
            credential,
            server,
            user,
            log,

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
    fn test_is_debug_via_args() {
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
            let mut config = Config {
                ..Default::default()
            };
            config = config.update(args);
            assert_eq!(config.is_debug(), want);
        }
    }

    #[test]
    fn test_is_debug_via_toml() {
        let tests = vec![
            (
                Log {
                    severity: "warn".to_string(),
                },
                false,
            ),
            (
                Log {
                    severity: "info".to_string(),
                },
                false,
            ),
            (
                Log {
                    severity: "debug".to_string(),
                },
                true,
            ),
        ];
        for (log, want) in tests.into_iter() {
            let config = Config {
                log,
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
