use crate::Opts;

pub struct Config {
    pub opts: Opts,
}

impl Config {
    pub fn new(opts: Opts) -> Self {
        Config { opts }
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
        let config = Config::new(opts);
        assert_eq!(config.is_debug(), false);
    }
}
