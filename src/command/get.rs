use crate::config::Config;

pub struct Get {
    config: Config,
}

impl Get {
    pub fn new(c: Config) -> Self {
        Self { config: c }
    }

    pub fn invoke(&self) {
        let _ = self.config;
    }
}
