use anyhow::Result;
use autoodo::{
    self,
    autoodo::{
        Config, MeResponse, PresencesResponse,
        client::{ClockodoClient, ClockodoEndpoint},
    },
};
use dotenv::dotenv;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let conf = Config::new();

    let client = ClockodoClient::new(&conf)?;
    let myself = client.get::<MeResponse>(ClockodoEndpoint::Myself).await?.data;

    println!("Your Clockodo ID: {}", myself.id);

    let presences = client.get::<PresencesResponse>(ClockodoEndpoint::Presences).await?.data;

    for presence in presences.users {
        println!(
            "User: {} ({}) -> {}",
            presence.name,
            presence.id,
            presence.running_clock.is_some()
        );
    }

    Ok(())
}
