use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait Action {
    async fn execute(&self) -> Result<()>;
}
