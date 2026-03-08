use anyhow::Result;
use autoodo::{
    self,
    autoodo::{ClockodoClient, ClockodoEndpoint, Config, MeResponse},
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let conf = Config::new();

    let client = ClockodoClient::new(&conf)?;
    let myself = client
        .get::<MeResponse>(ClockodoEndpoint::Myself)
        .await?
        .data;

    println!("Your Clockodo ID: {}", myself.id);

    Ok(())
}
