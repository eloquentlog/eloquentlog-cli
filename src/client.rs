use std::collections::HashMap;

use crate::config::Config;

pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn get_messages(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!("{}/_api/{}", self.config.server.url, "messages");
        if self.config.is_debug() {
            println!("url = {}", url);
        }

        let res: HashMap<String, String> = reqwest::get(&url)?.json()?;
        if self.config.is_debug() {
            println!("res = {:#?}", res);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::config::Server;

    use mockito::mock;

    #[test]
    fn test_get_messages() {
        let _m = mock("GET", "/_api/messages")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body("{}")
            .create();

        let server = Server {
            url: mockito::server_url(),
        };
        let config = Config {
            server,
            ..Default::default()
        };
        let client = Client::new(config);
        let res = client.get_messages();

        assert!(res.is_ok());
    }
}
