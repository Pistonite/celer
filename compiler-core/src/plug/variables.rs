//! Variables and Assertion plugins

use std::collections::HashMap;

use serde_json::{json, Map, Number, Value};

use crate::api::{CompilerContext, CompilerMetadata};
use crate::comp::{CompDoc, CompLine};
use crate::json::Coerce;
use crate::macros::async_trait;
use crate::pack::PackerResult;
use crate::types::{DocRichText, DocDiagnostic};
use crate::util::async_for;
use crate::{lang, prop};

use super::{PlugResult, PluginRuntime, operation};

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

fn float_to_string(a: f64) -> String {
    let rounded = a.round();
    if float_eq(a, rounded) {
        (rounded as i64).to_string()
    } else {
        a.to_string()
    }
}

#[inline]
fn floor_nonneg(a: f64) -> Option<u64> {
    let i = a.floor() as i64;
    if i < 0 {
        None
    } else {
        Some(i as u64)
    }
}

fn to_hex(a: f64) -> String {
    match floor_nonneg(a) {
        Some(i) => format!("{i:x}"),
        None => float_to_string(a)
    }
}

fn to_hex_upper(a: f64) -> String {
    match floor_nonneg(a) {
        Some(i) => format!("{i:X}"),
        None => float_to_string(a)
    }
}

fn to_roman(a: f64) -> String {
    match to_roman_core(a) {
        Some(mut s) => {
            s.make_ascii_lowercase();
            s
        }
        None => float_to_string(a),
    }
}

fn to_roman_upper(a: f64) -> String {
    to_roman_core(a).unwrap_or_else(|| float_to_string(a))
}

#[inline]
fn to_roman_core(a: f64) -> Option<String> {
    match floor_nonneg(a) {
        Some(i) if i >= 1 && i <= roman::MAX as u64 => roman::to(i as i32),
        _ => None,
    }
}

enum Operator<'a> {
    Add(Operand<'a>),
    Sub(Operand<'a>),
    Mul(Operand<'a>),
    Div(Operand<'a>),
    Assign(Operand<'a>),
}

impl<'a> Operator<'a> {
    /// Apply the operator to value `v`
    pub fn apply(&self, v: &f64, vars: &HashMap<String, f64>) -> f64 {
        match self {
            Self::Add(op) => *v + op.eval(vars),
            Self::Sub(op) => *v - op.eval(vars),
            Self::Mul(op) => *v * op.eval(vars),
            Self::Div(op) => *v / op.eval(vars),
            Self::Assign(op) => op.eval(vars),
        }
    }
}

enum Operand<'a> {
    Num(f64),
    Var(&'a str),
}

impl<'a> Operand<'a> {
    pub fn try_num(arg: &str) -> Option<Self> {
        arg.parse::<f64>().ok().map(Operand::Num)
    }

    pub fn eval(&self, vars: &HashMap<String, f64>) -> f64 {
        match self {
            Operand::Num(num) => *num,
            Operand::Var(var) => vars.get(*var).copied().unwrap_or(0.0),
        }
    }
}

impl<'a> From<&'a str> for Operand<'a> {
    fn from(s: &'a str) -> Self {
        if let Ok(num) = s.parse::<f64>() {
            Operand::Num(num)
        } else {
            Operand::Var(s)
        }
    }
}

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
        Value::Object(
            self.current
                .iter()
                .map(|(k, v)| (k.to_owned(), json!(*v)))
                .collect(),
        )
    }

    pub fn transform_text(&self, diagnostics: &mut Vec<DocDiagnostic>, text: &mut DocRichText, new_tag: &str) {
        if let Err(e) = self.transform_text_with_tag(text, new_tag) {
            diagnostics.push(DocDiagnostic { 
                msg: lang::parse_poor(&e), 
                msg_type: "error".to_string(),
                source: "plugin/variables".to_string(),
            });
        }
    }

    fn transform_text_with_tag(&self, text: &mut DocRichText, new_tag: &str) -> Result<(), String> {
        match text.tag.as_ref().map(String::as_ref) {
            Some(VAR) => self.transform_text_fn(text, float_to_string, |x: f64|float_to_string(x.floor())),
            Some(VAR_HEX)  => self.transform_text_fn(text, to_hex, to_hex),
            Some(VAR_HEX_UPPER)=> self.transform_text_fn(text, to_hex_upper, to_hex_upper),
            Some(VAR_ROMAN) => self.transform_text_fn(text, to_roman, to_roman),
            Some(VAR_ROMAN_UPPER) => self.transform_text_fn(text, to_roman_upper, to_roman_upper),
            _ => return Ok(()),
        }?;

        text.tag = Some(new_tag.to_string());
        Ok(())
    }

    fn transform_text_fn<FExact, FRound>(&self, text: &mut DocRichText, fn_exact: FExact, fn_round: FRound) -> Result<(), String>
    where
        FExact: Fn(f64) -> String,
        FRound: Fn(f64) -> String,
    {
        let mut iter = text.text.split(':').rev();
        let mut new_text = if let Some(variable) = iter.next() {
    let value = self.current.get(variable).copied().unwrap_or(0.0);
            fn_round(value)
        } else {
    let value = self.current.get(&text.text).copied().unwrap_or(0.0);
    fn_exact(value)
        };

        while let Some(op) = iter.next() {
            if let Some(x) = op.strip_prefix("pad") {
                let mut iter = x.chars();
                let pad = iter.next().ok_or("`pad` must be followed by the character to pad")?;
                let width = iter.collect::<String>().parse::<usize>().map_err(|_| {
                    format!("`pad` must be followed by the character to pad, then the width as a number")
                })?;
                if new_text.len() < width {
                    let pad_len = width - new_text.len();
                    let mut padding = String::with_capacity(pad_len);
                    for _ in 0..pad_len {
                        padding.push(pad);
                    }
                    new_text.insert_str(0, &padding);
                }
                continue;
            }
            if let Some(x) = op.strip_prefix("last") {
                let width = x.parse::<usize>().map_err(|_| {
                    format!("`last` must be followed by the width as a non-negative number")
                })?;
                if new_text.len() > width {
                    new_text.replace_range(0..(new_text.len() - width), "");
                }
            }
            return Err(format!("`{op}` is not a valid format function."));
        }

        text.text = new_text;
        Ok(())
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
                Some(VAR) => Operator::Assign(Operand::Var(&text)),
                Some(ADD) => Operator::Add(text_ref.into()),
                Some(SUB) => Operator::Sub(text_ref.into()),
                Some(MUL) => Operator::Mul(text_ref.into()),
                Some(DIV) => Operator::Div(text_ref.into()),
                Some(other) => return Err(format!("`{other}` is not a valid operator tag")),
            };
            let v = self.current.get(k).unwrap_or(&0.0);
            let new_v = op.apply(v, &self.current);
            updates.push((k, new_v));
        });
        let _ = async_for!((k, v) in updates, {
            match self.current.get_mut(k) {
                Some(v_ref) => {*v_ref = v;}// likely
                None => {self.current.insert(k.to_string(), v);}
            };
        });
        Ok(())
    }
}

#[async_trait(?Send)]
impl PluginRuntime for VariablesPlugin {
    async fn on_pre_compile(&mut self, ctx: &mut CompilerContext) -> PackerResult<()> {
        // add the val tag if not defined already
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
            operation::for_each_rich_text_except_counter!(block in line {
                self.transform_text(&mut line.diagnostics, block, VAL);
            });
            if let Some(t) = line.counter_text.as_mut() {
                let tag = t.text.to_string();
                self.transform_text(&mut line.diagnostics, t, &tag);
            }
            if self.expose {
                line.properties.insert(prop::VALS.to_string(), self.get_vals());
            }
            line
        });

        Ok(())
    }
}
