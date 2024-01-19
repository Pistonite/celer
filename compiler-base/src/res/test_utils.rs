//! Test utilities for resource
use std::borrow::Cow;

use crate::macros::async_trait;

use super::{Loader, ResPath, ResResult};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct StubLoader;
#[async_trait(auto)]
impl Loader for StubLoader {
    async fn load_raw<'s>(&'s self, _: &ResPath<'_>) -> ResResult<Cow<'s, [u8]>> {
        panic!("stub loader called")
    }
}
