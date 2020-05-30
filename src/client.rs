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
        m.insert(
            "X-Requested-With",
            HeaderValue::from_static("XMLHttpRequest"),
        );

        if self.config.credential.secret.is_empty() {
            return m;
        }

        m.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!(
                "Token {}",
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
        let params = "".to_string();
        if params.is_empty() {
            return params;
        }
        format!("?{}", params)
    }

    pub fn lrange_message(
        &self,
        namespace: String,
        start: u64,
        stop: u64,
    ) -> Result<String, String> {
        let url = format!(
            "{base_url}/_api/{resource}/{action}{params}",
            base_url = self.config.server.url,
            resource = "message",
            action = format!("lrange/{}/{}/{}", namespace, start, stop),
            params = self.build_params(),
        );
        if self.config.is_debug() {
            println!("url: {}", url);
        }

        let agent = reqwest::Client::new();
        let res = agent
            .get(&url)
            .headers(self.build_headers())
            .send()
            .map_err(|e| {
                println!("err: {}", e);
                "".to_string()
            })?;
        if self.config.is_debug() {
            println!("res: {:#?}", res);
        }
        // FIXME
        Ok(format!("{:#?}", res))
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

        assert_eq!(result.len(), 3);

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("eloquentlog-cli"));
        headers.insert(ACCEPT, "application/json".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        assert_eq!(result, headers);
    }

    #[test]
    fn test_build_headers_with_config_contains_invalid_credential() {
        let credential = Credential {
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
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
        assert_eq!(result, headers);
    }

    #[test]
    fn test_build_headers_with_config_contains_credential() {
        let credential = Credential {
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
        headers.insert(AUTHORIZATION, "Token secret".parse().unwrap());
        headers.insert("X-Requested-With", "XMLHttpRequest".parse().unwrap());
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
    fn test_lrange_message() {
        let _m = mock("GET", "/_api/message/lrange/test/0/9")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("x-requested-with", "XMLHttpRequest")
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
        let res = client.lrange_message("dummy".to_string(), 0, 9);

        assert!(res.is_ok());
    }
}
