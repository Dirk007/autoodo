use std::time::Duration;

use anyhow::Result;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use url::Url;

use crate::autoodo::Config;

const HEADER_API_USER: &'static str = "X-ClockodoApiUser";
const HEADER_API_KEY: &'static str = "X-ClockodoApiKey";
const HEADER_EXTERNAL_APP: &'static str = "X-Clockodo-External-Application";

const MY_APP_IDENT: &'static str = "autoodo-rust-cli";

pub struct ClockodoClient {
    config: Config,
    client: reqwest::Client,
    headers: HeaderMap,
}

pub enum ClockodoEndpoint {
    Myself,
    Presences,
}

const MYSELF_ENDPOINT: &'static str = "/api/v4/users/me";
const PRESENCES_ENDPOINT: &'static str = "/api/v2/users/presences";

impl ToString for ClockodoEndpoint {
    fn to_string(&self) -> String {
        self.as_ref().to_string()
    }
}

impl AsRef<str> for ClockodoEndpoint {
    fn as_ref(&self) -> &str {
        match self {
            ClockodoEndpoint::Myself => MYSELF_ENDPOINT,
            ClockodoEndpoint::Presences => PRESENCES_ENDPOINT,
        }
    }
}

impl ClockodoClient {
    pub fn new(config: &Config) -> Result<Self> {
        let conf = config.clone();
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(2))
            .read_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .user_agent(MY_APP_IDENT)
            .build()?;

        let app_ident = format!("{};{}", MY_APP_IDENT, conf.clocko_email);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_API_USER, config.clocko_email.parse()?);
        headers.insert(HEADER_API_KEY, config.clocko_api_key.parse()?);
        headers.insert(HEADER_EXTERNAL_APP, app_ident.parse()?);

        Ok(ClockodoClient {
            config: conf,
            client,
            headers,
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: ClockodoEndpoint) -> Result<T> {
        let url = Url::parse(&self.config.clocko_base_url)?.join(endpoint.as_ref())?;

        Ok(self
            .client
            .get(url)
            .headers(self.headers.clone())
            .send()
            .await?
            .json::<T>()
            .await?)
    }
}
