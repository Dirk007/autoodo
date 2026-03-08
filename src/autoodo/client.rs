use crate::autoodo::Config;
use anyhow::Result;
use reqwest::header::HeaderMap;
use serde::de::DeserializeOwned;
use url::Url;

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
}

impl ToString for ClockodoEndpoint {
    fn to_string(&self) -> String {
        match self {
            ClockodoEndpoint::Myself => "/api/v4/users/me".to_string(),
        }
    }
}

impl ClockodoClient {
    pub fn new(config: &Config) -> Result<Self> {
        let conf = config.clone();
        let client = reqwest::Client::new();
        let app_ident = format!("{};{}", MY_APP_IDENT, conf.clocko_email);

        let mut headers = HeaderMap::new();
        headers.insert(HEADER_API_USER, config.clocko_email.parse()?);
        headers.insert(HEADER_API_KEY, config.clocko_api_key.parse()?);
        headers.insert(HEADER_EXTERNAL_APP, app_ident.parse()?);

        Ok(ClockodoClient {
            config: conf,
            client: client,
            headers: headers,
        })
    }

    pub async fn get<T: DeserializeOwned>(&self, endpoint: ClockodoEndpoint) -> Result<T> {
        let url = Url::parse(&self.config.clocko_base_url)?.join(endpoint.to_string().as_str())?;

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
