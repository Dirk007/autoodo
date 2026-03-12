use anyhow::Result;

use crate::autoodo::state::State;
pub trait Trigger {
    fn should_trigger(&self, state: &dyn State) -> Result<bool>;
}
