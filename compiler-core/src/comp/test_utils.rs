//! Test utilities
use std::borrow::Cow;
use std::collections::BTreeMap;

use instant::Instant;
use serde_json::Value;

use crate::json::RouteBlob;
use crate::lang::Preset;
use crate::pack::{CompileContext, Compiler};
use crate::prep::{
    Axis, CompilerMetadata, GameCoord, MapCoordMap, MapMetadata, RouteConfig, RouteMetadata,
    Setting,
};

use super::{LineContext, DEFAULT_SETTING};

impl Default for Compiler<'static> {
    /// Create a default/stub compiler for testing
    fn default() -> Self {
        Compiler {
            ctx: CompileContext {
                start_time: Instant::now(),
                config: Cow::Owned(RouteConfig::default()),
                meta: Cow::Owned(CompilerMetadata::default()),
                setting: &DEFAULT_SETTING,
            },
            route: Cow::Owned(RouteBlob::Prim(Value::Null)),
            color: Default::default(),
            coord: Default::default(),
            plugin_runtimes: Default::default(),
        }
    }
}

thread_local! {
    static DEFAULT_COMPILER: Compiler<'static> = Compiler::default();
}

impl Default for LineContext<'static, 'static> {
    fn default() -> Self {
        LineContext {
            compiler: DEFAULT_COMPILER.with(|c| c),
            line: Default::default(),
            errors: Default::default(),
        }
    }
}

impl<'c, 'p> LineContext<'c, 'p> {
    pub fn with_compiler(compiler: &'c Compiler<'p>) -> Self {
        LineContext {
            compiler: &compiler,
            ..Default::default()
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct CompilerBuilder {
    config: RouteConfig,
    presets: BTreeMap<String, Preset>,
    color: String,
    coord: GameCoord,
    default_icon_priority: i64,
}

impl CompilerBuilder {
    pub fn new(project: RouteConfig, color: String, coord: GameCoord) -> Self {
        CompilerBuilder {
            color,
            coord,
            config: project,
            ..Default::default()
        }
    }
    pub fn add_preset(&mut self, name: &str, preset: Preset) -> &mut Self {
        self.presets.insert(name.to_string(), preset);
        self
    }

    pub fn set_default_icon_priority(&mut self, priority: i64) -> &mut Self {
        self.default_icon_priority = priority;
        self
    }

    pub fn build(self) -> Compiler<'static> {
        Compiler {
            ctx: CompileContext {
                start_time: Instant::now(),
                config: Cow::Owned(self.config),
                meta: Cow::Owned(CompilerMetadata {
                    presets: self.presets,
                    default_icon_priority: self.default_icon_priority,
                    ..Default::default()
                }),
                setting: &DEFAULT_SETTING,
            },
            color: self.color,
            coord: self.coord,
            ..Default::default()
        }
    }
}

pub fn create_test_compiler_with_coord_transform() -> Compiler<'static> {
    let config = RouteConfig {
        map: Some(MapMetadata {
            coord_map: MapCoordMap {
                mapping_3d: (Axis::X, Axis::Y, Axis::Z),
                ..Default::default()
            },
            ..Default::default()
        }),
        ..Default::default()
    };
    let builder = CompilerBuilder::new(config, Default::default(), Default::default());
    builder.build()
}
