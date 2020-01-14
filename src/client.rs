use reqwest;
use reqwest::header::{
    HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT,
};

use crate::config::Config;

pub struct Client {
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    fn build_headers(&self) -> HeaderMap {
        let mut m = HeaderMap::new();
        // default headers
        m.insert(USER_AGENT, HeaderValue::from_static("eloquentlog-cli"));
        m.insert(ACCEPT, HeaderValue::from_static("application/json"));

        if self.config.credential.secret.is_empty() {
            return m;
        }

        m.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Bearer {}",
                self.config.credential.secret,
            ))
            .map_err(|e| {
                if self.config.is_debug() {
                    println!("err: {}", e);
                }
                e
            })
            .unwrap_or_else(|_| HeaderValue::from_static("")),
        );
        m
    }

    fn build_params(&self) -> String {
        if !self.config.credential.api_key.is_empty() {
            return format!("api_key={}", self.config.credential.api_key);
        }
        "".to_string()
    }

    pub fn get_messages(&self) -> Result<(), Box<dyn std::error::Error>> {
        let url = format!(
            "{base_url}/_api/{resource}?{params}",
            base_url = self.config.server.url,
            resource = "messages",
            params = self.build_params(),
        );
        if self.config.is_debug() {
            println!("url: {}", url);
        }

        let agent = reqwest::Client::new();
        let res = agent.get(&url).headers(self.build_headers()).send()?;
        if self.config.is_debug() {
            println!("res: {:#?}", res);
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::config::{Credential, Server};

    use mockito::mock;

    #[test]
    fn test_build_headers_with_config_contains_empty_credential() {
        let config = Config {
            ..Default::default()
        };
        let client = Client::new(config);
        let result = client.build_headers();

        assert_eq!(result.len(), 2);

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("eloquentlog-cli"));
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        assert_eq!(result, headers);
    }

    #[test]
    fn test_build_headers_with_config_contains_invalid_credential() {
        let credential = Credential {
            api_key: "api-key".to_string(),
            secret: "secret\n".to_string(),
        };
        let config = Config {
            credential,
            ..Default::default()
        };
        let client = Client::new(config);
        let result = client.build_headers();

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "eloquentlog-cli".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, "".parse().unwrap());
        assert_eq!(result, headers);
    }

    #[test]
    fn test_build_headers_with_config_contains_credential() {
        let credential = Credential {
            api_key: "api-key".to_string(),
            secret: "secret".to_string(),
        };
        let config = Config {
            credential,
            ..Default::default()
        };
        let client = Client::new(config);
        let result = client.build_headers();

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, "eloquentlog-cli".parse().unwrap());
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert(AUTHORIZATION, "Bearer secret".parse().unwrap());
        assert_eq!(result, headers);
    }

    #[test]
    fn test_build_params_with_config_contains_empty_credential() {
        let config = Config {
            ..Default::default()
        };
        let client = Client::new(config);
        let result = client.build_params();

        assert_eq!(result, "");
    }

    #[test]
    fn test_build_params_with_config_contains_empty_api_key() {
        let credential = Credential {
            api_key: "".to_string(),
            secret: "secret".to_string(),
        };
        let config = Config {
            credential,
            ..Default::default()
        };
        let client = Client::new(config);
        let result = client.build_params();

        assert_eq!(result, "");
    }

    #[test]
    fn test_build_params_with_config_contains_api_key() {
        let credential = Credential {
            api_key: "api-key".to_string(),
            secret: "secret".to_string(),
        };
        let config = Config {
            credential,
            ..Default::default()
        };
        let client = Client::new(config);
        let result = client.build_params();

        assert_eq!(result, "api_key=api-key");
    }

    #[test]
    fn test_get_messages() {
        let _m = mock("GET", "/_api/messages?api_key=<api-key>")
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
