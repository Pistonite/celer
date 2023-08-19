mod pack_metadata;
mod pack_use;
pub use pack_use::*;

#[async_trait::async_trait]
pub trait Resource {
    /// Request another resource based on this resource
    async fn resolve_use(&self, target: &Use) -> Result<Box<dyn Resource>, ResourceError>;

    /// Get the bytes of this resource
    async fn get(&self) -> Result<Vec<u8>, ResourceError>;
}

// trait ResourceContext {
// async fn get_relative(requester: &dyn Resource, path: &str) -> Result<Box<dyn Resource>, ResourceError> {
// }
    // async fn get_absolute(&self, requester: &dyn Resource, path: &str) -> Result<Box<dyn Resource>, ResourceError>;
    // async fn get_remote(&self, requester: &dyn Resource, path: &str) -> Result<Box<dyn Resource>, ResourceError>;
// }
//


#[derive(Debug, thiserror::Error)]
pub enum ResourceError {
    #[error("Resource not found: {0}")]
    NotFound(String),
}
