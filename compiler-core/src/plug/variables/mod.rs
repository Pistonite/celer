//! Variables plugin

use std::borrow::Cow;
use std::collections::HashMap;

use serde_json::{json, Map, Value};

use crate::api::{CompilerContext, CompilerMetadata};
use crate::comp::CompDoc;
use crate::json::Coerce;
use crate::lang;
use crate::macros::async_trait;
use crate::pack::PackerResult;
use crate::prop;
use crate::types::{DocColor, DocDiagnostic, DocRichText, DocTag};
use crate::util::async_for;

use super::{operation, PlugResult, PluginRuntime};

mod convert;
mod transform;

const ADD: &str = "add";
const SUB: &str = "sub";
const MUL: &str = "mul";
const DIV: &str = "div";
const VAR: &str = "var";
const VAL: &str = "val";

const VAR_HEX: &str = "var-hex";
const VAR_HEX_UPPER: &str = "var-hex-upper";
const VAR_ROMAN: &str = "var-roman";
const VAR_ROMAN_UPPER: &str = "var-roman-upper";

#[inline]
fn float_eq(a: f64, b: f64) -> bool {
    (a - b).abs() < f64::EPSILON
}

enum Operator<'a> {
    Add(Operand<'a>),
    Sub(Operand<'a>),
    Mul(Operand<'a>),
    Div(Operand<'a>),
    Assign(Operand<'a>),
}

macro_rules! map_for_var {
    ($k:ident, $($self:tt)+) => {
        {
            if $k.starts_with('_') {
                $($self)*.temporary
            } else {
                $($self)*.current
            }
        }
    }
}

impl<'a> Operator<'a> {
    /// Apply the operator to value `v`
    pub fn apply(&self, v: f64, vars: &VariablesPlugin) -> f64 {
        match self {
            Self::Add(op) => v + op.eval(vars),
            Self::Sub(op) => v - op.eval(vars),
            Self::Mul(op) => v * op.eval(vars),
            Self::Div(op) => v / op.eval(vars),
            Self::Assign(op) => op.eval(vars),
        }
    }
}

enum Operand<'a> {
    Num(f64),
    Var(Cow<'a, str>),
}

impl<'a> Operand<'a> {
    pub fn try_num(arg: &str) -> Option<Self> {
        arg.parse::<f64>().ok().map(Operand::Num)
    }

    pub fn eval(&self, vars: &VariablesPlugin) -> f64 {
        match self {
            Operand::Num(num) => *num,
            Operand::Var(var) => vars.get(var.as_ref()),
        }
    }
}

impl<'a> From<&'a str> for Operand<'a> {
    fn from(s: &'a str) -> Self {
        if let Ok(num) = s.parse::<f64>() {
            Operand::Num(num)
        } else {
            Operand::Var(Cow::Borrowed(s))
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct VariablesPlugin {
    current: HashMap<String, f64>,
    temporary: HashMap<String, f64>,
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
                            plugin.insert(k.to_string(), num);
                        }
                    }
                }
            }
        }

        plugin
    }

    pub fn insert(&mut self, k: String, v: f64) {
        map_for_var!(k, &mut self).insert(k, v);
    }

    pub fn get(&self, k: &str) -> f64 {
        map_for_var!(k, &self).get(k).copied().unwrap_or(0.0)
    }

    pub fn get_mut(&mut self, k: &str) -> Option<&mut f64> {
        map_for_var!(k, &mut self).get_mut(k)
    }

    /// Get a copy of current values map as a json object
    pub fn get_vals(&self) -> Value {
        Value::Object(
            self.current
                .iter()
                .map(|(k, v)| (k.to_owned(), json!(*v)))
                .collect(),
        )
    }

    pub fn transform_text(
        &self,
        diagnostics: &mut Vec<DocDiagnostic>,
        text: &mut DocRichText,
        new_tag: &str,
    ) {
        if let Err(e) = self.transform_text_with_tag(text, new_tag) {
            diagnostics.push(DocDiagnostic {
                msg: lang::parse_poor(&e),
                msg_type: "error".to_string(),
                source: "plugin/variables".to_string(),
            });
        }
    }

    fn transform_text_with_tag(&self, text: &mut DocRichText, new_tag: &str) -> Result<(), String> {
        let text_ref = &text.text;
        let get_fn = |t: &str| self.get(t);
        text.text = match text.tag.as_ref().map(String::as_ref) {
            Some(VAR) => transform::transform_text_fn(
                text_ref,
                get_fn,
                convert::float_to_string,
                |x: f64| convert::float_to_string(x.round()),
            ),
            Some(VAR_HEX) => {
                transform::transform_text_fn(text_ref, get_fn, convert::to_hex, convert::to_hex)
            }
            Some(VAR_HEX_UPPER) => transform::transform_text_fn(
                text_ref,
                get_fn,
                convert::to_hex_upper,
                convert::to_hex_upper,
            ),
            Some(VAR_ROMAN) => {
                transform::transform_text_fn(text_ref, get_fn, convert::to_roman, convert::to_roman)
            }
            Some(VAR_ROMAN_UPPER) => transform::transform_text_fn(
                text_ref,
                get_fn,
                convert::to_roman_upper,
                convert::to_roman_upper,
            ),
            _ => return Ok(()),
        }?;

        text.tag = Some(new_tag.to_string());
        Ok(())
    }

    pub fn increment(&mut self, var: &str) {
        match self.get_mut(var) {
            Some(v) => {
                *v += 1.0;
            } // likely
            None => {
                self.insert(var.to_string(), 1.0);
            }
        };
    }

    pub fn clear_temporary(&mut self) {
        self.temporary.clear();
    }

    pub async fn update_vars(&mut self, diagnostics: &mut Vec<DocDiagnostic>, vars: &Value) {
        if let Err(e) = self.update_vars_internal(vars).await {
            diagnostics.push(DocDiagnostic {
                msg: lang::parse_poor(&e),
                msg_type: "error".to_string(),
                source: "plugin/variables".to_string(),
            });
        }
    }

    async fn update_vars_internal(&mut self, vars: &Value) -> Result<(), String> {
        match vars {
            Value::Object(map) => self.update_vars_map(map).await?,
            Value::Array(arr) => {
                let _ = async_for!(v in arr, {
                    let map = v.as_object().ok_or("vars array must contain objects".to_string())?;
                    self.update_vars_map(map).await?;
                });
            }
            _ => return Err("vars must be an object or an array of objects".to_string()),
        }
        Ok(())
    }

    async fn update_vars_map(&mut self, vars: &Map<String, Value>) -> Result<(), String> {
        let mut updates = vec![];
        let _ = async_for!((k, v) in vars, {
            let text = v.coerce_to_string();
            let ops = lang::parse_rich(&text);
            let mut iter = ops.into_iter();
            let op = iter.next().ok_or(format!("invalid empty operation: `{text}`"))?;
            if iter.next().is_some() {
                return Err(format!("invalid operation: `{text}`"));
            }
            let text = op.text;
            let text_ref: &str = &text;
            let op = match op.tag.as_ref().map(String::as_ref) {
                None => Operator::Assign(Operand::try_num(&text).ok_or(format!("`{text}` is not a valid number. If you meant to assign the variable, use `.var({text})`"))?),
                Some(VAR) => Operator::Assign(Operand::Var(Cow::Borrowed(&text))),
                Some(ADD) => Operator::Add(text_ref.into()),
                Some(SUB) => Operator::Sub(text_ref.into()),
                Some(MUL) => Operator::Mul(text_ref.into()),
                Some(DIV) => Operator::Div(text_ref.into()),
                Some(other) => return Err(format!("`{other}` is not a valid operator tag")),
            };
            let new_v = op.apply(self.get(k), self);
            updates.push((k, new_v));
        });
        let _ = async_for!((k, v) in updates, {
            match self.get_mut(k) {
                Some(v_ref) => {*v_ref = v;}// likely
                None => {self.insert(k.to_string(), v);}
            };
        });
        Ok(())
    }
}

#[async_trait(?Send)]
impl PluginRuntime for VariablesPlugin {
    async fn on_pre_compile(&mut self, ctx: &mut CompilerContext) -> PackerResult<()> {
        // add the val tag if not defined already
        let tag = DocTag {
            color: Some(DocColor::LightDark {
                light: Some("#800".to_string()),
                dark: Some("#ffc0cb".to_string()),
            }),
            ..Default::default()
        };
        ctx.phase0
            .project
            .tags
            .entry(VAL.to_string())
            .and_modify(|t| t.apply_to_default(&tag))
            .or_insert(tag);
        ctx.phase0.project.tags.entry(VAL.to_string()).or_default();
        Ok(())
    }
    async fn on_compile(&mut self, _: &CompilerMetadata, comp_doc: &mut CompDoc) -> PlugResult<()> {
        comp_doc.known_props.insert(prop::VARS.to_string());
        comp_doc.known_props.insert(prop::VALS.to_string());

        let _ = async_for!(preface in comp_doc.preface.iter_mut(), {
            for block in preface.iter_mut() {
                self.transform_text(&mut comp_doc.diagnostics, block, VAL);
            }
        });
        operation::for_each_line!(line in comp_doc {
            if let Some(vars) = line.properties.get(prop::VARS) {
                self.update_vars(&mut line.diagnostics, vars).await
            }
            if let Some(t) = line.counter_text.as_mut() {
                let tag = t.text.to_string();
                self.increment(&tag);
                self.transform_text(&mut line.diagnostics, t, &tag);
            }
            operation::for_each_rich_text_except_counter!(block in line {
                self.transform_text(&mut line.diagnostics, block, VAL);
            });
            if self.expose {
                line.properties.insert(prop::VALS.to_string(), self.get_vals());
            }
            self.clear_temporary();
            line
        });

        Ok(())
    }
}
