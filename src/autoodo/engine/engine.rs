use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use tokio::sync::Mutex;

use crate::autoodo::{Action, Trigger, state::State};

#[async_trait]
pub trait Engine<S: State + Send + Sync> {
    async fn add_logic(
        &mut self,
        trigger: Box<dyn Trigger + Send + Sync>,
        action: Box<dyn Action + Send + Sync>,
    ) -> Result<()>;
    async fn run_one(&self, state: S) -> Result<()>
    where
        S: 'async_trait;
}

pub struct AutoodoEngine {
    logic: Arc<Mutex<Vec<(Box<dyn Trigger + Send + Sync>, Box<dyn Action + Send + Sync>)>>>,
}

#[async_trait]
impl<S: State + Send + Sync> Engine<S> for AutoodoEngine {
    async fn add_logic(
        &mut self,
        trigger: Box<dyn Trigger + Send + Sync>,
        action: Box<dyn Action + Send + Sync>,
    ) -> Result<()> {
        self.logic.lock().await.push((trigger, action));
        Ok(())
    }

    async fn run_one(&self, state: S) -> Result<()>
    where
        S: 'async_trait,
    {
        let logic = self.logic.lock().await;
        for (trigger, action) in logic.iter() {
            if trigger.should_trigger(&state)? {
                action.execute().await?;
            }
        }
        Ok(())
    }
}
