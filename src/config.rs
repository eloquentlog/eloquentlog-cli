use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use dirs;
use serde::Deserialize;

use crate::Opts;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub secret: String,

    #[serde(skip)]
    pub opts: Opts,
}

// defaults
pub const CONFIG_FILE: &str = "eloquentlog.toml";
pub const CONFIG_ROOT: &str = "eloquentlog";

const DEFAULT_CONTENTS: &str = r#"
secret = "secret"
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

    pub fn create_file() -> Result<Self, std::io::Error> {
        let config_file = Self::default_config_file();
        let mut file = File::create(&config_file)?;
        file.write_all(DEFAULT_CONTENTS.to_string().into_bytes().as_slice())?;
        file.sync_data()?;

        Self::load_from_local_file(Some(config_file))
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
        self.opts.debug
    }

    pub fn is_valid(&self) -> bool {
        // TODO
        true
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_debug() {
        let opts = Opts {
            ..Default::default()
        };

        let config = Config {
            secret: "".to_string(),
            opts,
        };
        assert_eq!(config.is_debug(), false);
    }
}
