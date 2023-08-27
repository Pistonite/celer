//! Packs [`MapMetadata`]

use celerctypes::MapMetadata;
use serde_json::Value;

use super::PackerResult;

/// Parses the `map` property in a config json blob
pub async fn pack_map(value: Value, index: usize) -> PackerResult<MapMetadata> {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_pack_map() {
        todo!()
    }
}
