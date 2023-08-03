use async_trait::async_trait;

#[async_trait]
pub trait DataValidator {
    async fn validate(&self) -> crate::Result<()>
    where
        Self: Sized;
}
