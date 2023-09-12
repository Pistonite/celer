use celerctypes::{DocPoorText, ExecDoc, RouteMetadata};
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub mod api;
pub mod comp;
pub mod exec;
pub mod json;
pub mod lang;
pub mod pack;
pub mod plug;

pub mod metrics;
pub mod util;

use comp::{CompLine, Compiler, CompilerError};
use lang::Preset;
use metrics::CompilerMetrics;
use pack::{PackedProject, PackerError, ResourceLoader, ResourcePath, ResourceResolver};

pub async fn test_number(n: u64) -> u64 {
    let mut sum = 0;
    for i in 0..n {
        sum += test2(i).await;
    }
    sum
}

async fn test2(n: u64) -> u64 {
    return n+1;
}
