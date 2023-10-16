//! Variables and Assertion plugins

use std::collections::HashMap;

use serde_json::{Value, Number, json};

use crate::api::{CompilerMetadata, CompilerContext};
use crate::comp::CompDoc;
use crate::json::Coerce;
use crate::pack::PackerResult;
use crate::macros::async_trait;
use crate::prop;
use crate::types::DocRichText;

use super::{PluginRuntime, PlugResult};

const ADD: &str = "add";
const SUB: &str = "sub";
const MUL: &str = "mul";
const DIV: &str = "div";
const VAR: &str = "var";
const VAL: &str = "val";

#[derive(Debug, Clone, Default)]
pub struct VariablesPlugin {
    current: HashMap<String, f64>,
    expose: bool,
}
impl VariablesPlugin {
    pub fn from_props(props: &Value) -> Self {
        let mut plugin = Self::default();
        if let Some(m) = props.as_object() {
            if let Some(expose) = m.get(prop::EXPOSE) {
                if expose.coerce_truthy() {
                    plugin.expose = true;
                } 
            }
            if let Some(init) = m.get(prop::INIT) {
                if let Some(init_map) = init.as_object() {
                    for (k, v) in init_map {
                        if let Some(num) = v.try_coerce_to_f64() {
                            plugin.current.insert(k.to_string(), num);
                        }
                    }
                }
            }
        }

        plugin
    }

    /// Get a copy of current values map as a json object
    pub fn get_vals(&self) -> Value {
        Value::Object(self.current.iter().map(|(k, v)| (k.to_owned(), json!(*v))).collect())
    }

    pub fn transform_tag(&self, text: &mut Vec<DocRichText>) {
    }
}

#[async_trait(?Send)]
impl PluginRuntime for VariablesPlugin {
    async fn on_pre_compile(&mut self, ctx: &mut CompilerContext) -> PackerResult<()>
    {
        // add the val tag if not defined already
        ctx.phase0
            .project
            .tags
            .entry(VAL.to_string())
            .or_default();
        Ok(())
    }
    async fn on_compile(&mut self, _: &CompilerMetadata, comp_doc: &mut CompDoc) -> PlugResult<()> {
        comp_doc.known_props.insert(prop::VARS.to_string());
        comp_doc.known_props.insert(prop::VALS.to_string());

        todo!()
    }
}
