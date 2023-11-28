use std::collections::BTreeMap;

use crate::macros::test_suite;

use super::Preset;

impl Preset {
    /// Optimize the preset
    ///
    /// This will mutate the preset in-place. `presets` should contain unoptimized presets and `optimized_presets` should contain optimized presets.
    /// Neither map should contain `self`.
    ///
    /// This will also attempt to optimize any preset referred to by this preset. However, this
    /// will not give error if the referred preset is not found or there is a cycle. It will leave it unoptimized and
    /// defer to the compiler to give error, if this preset is ever instantiated.
    pub async fn optimize(
        &mut self,
        _presets: &mut BTreeMap<String, Preset>,
        _optimized_presets: &mut BTreeMap<String, Preset>,
    ) {
        // TODO #114: optimize presets

        // Generally should work like this:
        // if self top level has any tempstr key, it's not optimizable
        // because the key could be "presets"
        //
        // Otherwise, if there is a static key "presets", we can optimize
        //
        // if the preset is a static preset (i.e. TempStr::is_literal)
        // parse it as PresetInst and hydrate it, store it temporarily
        // do this until the first non-static preset is found
        //
        // Lastly, apply self to the hydrated presets
    }
}

#[test_suite]
mod test {}
