use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::autoodo::{Action, Trigger, state::State};

#[async_trait]
pub trait Engine {
    async fn add_logic(&mut self, trigger: Box<dyn Trigger>, action: Box<dyn Action>) -> Result<()>;
    async fn run_one(&self, state: State) -> Result<()>;
}

pub struct AutoodoEngine {
    logic: Arc<Mutex<Vec<(Box<dyn Trigger>, Box<dyn Action>)>>>,
}

impl AutoodoEngine {
    pub fn new() -> Self {
        Self {
            logic: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl Engine for AutoodoEngine {
    async fn add_logic(&mut self, trigger: Box<dyn Trigger>, action: Box<dyn Action>) -> Result<()> {
        self.logic.lock().await.push((trigger, action));
        Ok(())
    }

    async fn run_one(&self, state: State) -> Result<()> {
        let mut logic = self.logic.lock().await;
        for (trigger, action) in logic.iter_mut() {
            let (should_trigger, params) = trigger.should_trigger(&state)?;
            if should_trigger {
                action.execute(params, &state).await?;
            }
        }
        Ok(())
    }
}
