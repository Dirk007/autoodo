use dotenv::dotenv;
use envconfig::Envconfig;

#[derive(Envconfig, Clone)]
pub struct Config {
    #[envconfig(from = "CLOCKO_API_KEY", default = "your_api_key")]
    pub clocko_api_key: String,
    #[envconfig(from = "CLOCKO_EMAIL", default = "your_email")]
    pub clocko_email: String,
    #[envconfig(from = "CLOCKO_ORG", default = "your_organization")]
    pub clocko_org: String,
    #[envconfig(from = "CLOCKO_BASE_URL", default = "clockodo base_url_here")]
    pub clocko_base_url: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();
        Config::init_from_env().unwrap()
    }
}
