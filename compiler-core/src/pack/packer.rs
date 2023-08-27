use super::{ResourceResolver, ResourceLoader};

/// Packer resolves resources and packs them into a single json blob for compiling
pub struct Packer {
    resolver: Box<dyn ResourceResolver>,
    loader: Box<dyn ResourceLoader>,
}
