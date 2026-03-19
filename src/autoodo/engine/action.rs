use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;

use crate::autoodo::{any::Any, state::State};

#[async_trait]
pub trait Action
where
    Self: Send + Sync,
{
    async fn execute(&self, params: Option<HashMap<String, Any>>, state: &State) -> Result<()>;
}
