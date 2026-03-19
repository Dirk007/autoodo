use std::collections::HashMap;

use anyhow::Result;

use crate::autoodo::{any::Any, state::State};
pub trait Trigger
where
    Self: Send + Sync,
{
    fn should_trigger(&mut self, state: &State) -> Result<(bool, Option<HashMap<String, Any>>)>;
}
