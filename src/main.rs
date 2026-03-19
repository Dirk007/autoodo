use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;
use autoodo::{
    self,
    autoodo::{
        Action, AutoodoEngine, Config, Engine, MeResponse, PresencesResponse, Trigger,
        any::Any,
        client::{ClockodoClient, ClockodoEndpoint},
        state::State,
    },
};
use dotenv::dotenv;

struct ClockChangeTrigger {
    old: Option<State>,
}

impl Trigger for ClockChangeTrigger {
    fn should_trigger(&mut self, state: &State) -> Result<(bool, Option<HashMap<String, Any>>)> {
        if let Some(old) = &self.old {
            let mut changed_users = Vec::new();

            for presence in &state.users_presence {
                if let Some(old_presence) = old.users_presence.iter().find(|p| p.id == presence.id) {
                    let old_has_clock = old_presence.running_clock.is_some();
                    let new_has_clock = presence.running_clock.is_some();

                    if old_has_clock != new_has_clock {
                        changed_users.push(Any::String(presence.name.clone()));
                    }
                }
            }

            if !changed_users.is_empty() {
                let mut params = HashMap::new();
                params.insert("user_names".to_string(), Any::List(changed_users));
                self.old = Some(state.clone());
                return Ok((true, Some(params)));
            }
        }

        self.old = Some(state.clone());
        Ok((false, None))
    }
}

impl ClockChangeTrigger {
    pub fn new() -> Self {
        Self { old: None }
    }
}

struct PrinterAction {}

#[async_trait]
impl Action for PrinterAction {
    async fn execute(&self, params: Option<HashMap<String, Any>>, _: &State) -> Result<()> {
        if let Some(params) = params {
            println!("Clock changes detected: {:?}", params.get("user_names"));
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let conf = Config::new();

    let client = ClockodoClient::new(&conf)?;
    let myself = client.get::<MeResponse>(ClockodoEndpoint::Myself).await?.data;

    println!("Your Clockodo ID: {}", myself.id);

    let presences = client.get::<PresencesResponse>(ClockodoEndpoint::Presences).await?.data;

    for presence in &presences.users {
        println!(
            "User: {} ({}) -> {}",
            presence.name,
            presence.id,
            presence.running_clock.is_some()
        );
    }

    let mut engine = AutoodoEngine::new();
    engine
        .add_logic(Box::new(ClockChangeTrigger::new()), Box::new(PrinterAction {}))
        .await?;

    let mut state = State::new();
    state.users_presence = presences.users;
    engine.run_one(state).await?;

    Ok(())
}
