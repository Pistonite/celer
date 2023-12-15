//! Test utilities for resource
use std::borrow::Cow;

use crate::macros::async_trait;

use super::{Loader, ResPath, ResResult};

pub struct StubLoader;
#[async_trait(auto)]
impl Loader for StubLoader {
    async fn load_raw<'s, 'a>(&'s self, _: &ResPath<'a>) -> ResResult<Cow<'s, [u8]>> {
        panic!("stub loader called")
    }
}
