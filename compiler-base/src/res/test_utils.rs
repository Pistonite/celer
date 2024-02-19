//! Test utilities for resource
use crate::env::RefCounted;
use crate::macros::async_trait;

use super::{Loader, ResPath, ResResult};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StubLoader;
#[async_trait(auto)]
impl Loader for StubLoader {
    async fn load_raw(&self, _: &ResPath) -> ResResult<RefCounted<[u8]>> {
        panic!("stub loader called")
    }
}
