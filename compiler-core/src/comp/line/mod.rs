use serde_json::Value;

use crate::json::{Cast, Coerce, IntoSafeRouteBlob, SafeRouteBlob};
use crate::lang;
use crate::lang::PresetInst;
use crate::pack::Compiler;
use crate::prop;

use super::{validate_not_array_or_object, CompError, LineContext};

mod coord;
pub use coord::*;
mod marker;
pub use marker::*;
mod movement;
pub use movement::*;
mod note;
pub use note::*;
mod desugar;
mod preset;
pub use preset::*;
mod prop_map;
use prop_map::*;

impl<'c, 'p> LineContext<'c, 'p> {
    /// Compile a line
    ///
    /// 1. Text line is turned into {<text>: {}}
    /// 2. precedence of the presets (later overides previous)
    ///    - uses
    ///    - self text (if the preset doesn't define text)
    ///    - self preset
    ///    - self properties
    ///
    /// Errors are returned as an Err variant with the line and the errors.
    /// Diagnostics are not added to the line.
    pub fn parse_line(&mut self, value: SafeRouteBlob) {
        // Convert line into object form
        let (text, line_obj) = desugar::desugar_line(value);
        let line_obj = match line_obj {
            Ok(line) => line,
            Err(e) => {
                self.errors.push(e);
                self.line.text = lang::parse_rich(&text);
                return;
            }
        };

        // preprocess the `presets` property
        let mut properties = LinePropMap::new();
        let mut line_properties = Vec::with_capacity(line_obj.len());
        {
            let mut preset_value = None;
            for (k, v) in line_obj {
                match k.as_ref() {
                    prop::PRESETS => {
                        preset_value = Some(v);
                    }
                    _ => {
                        line_properties.push((k, v));
                    }
                }
            }
            if let Some(v) = preset_value {
                self.process_presets(0, v, &mut properties);
            }
        }

        // process the preset in the text
        if text.starts_with('_') {
            let preset_inst = PresetInst::try_parse(&text);
            if let Some(inst) = preset_inst {
                // At this level, we will only process the preset if it exists
                // otherwise treat the string as a regular string
                if self.compiler.meta.presets.contains_key(&inst.name) {
                    self.apply_preset(0, &inst, &mut properties);
                }
            }
        }

        // merge the line properties into preset properties
        // LinePropMap will auto desugar the properties
        for (k, v) in line_properties {
            properties.insert_value(k.into_owned(), v);
        }

        // expand presets in movements
        if let Some(movements) = properties.remove(prop::MOVEMENTS) {
            properties.insert_value(
                prop::MOVEMENTS.to_string(),
                self.expand_presets_in_movements(0, movements),
            );
        }

        // if the line doesn't have the text property yet, use the outer text
        if properties.get(prop::TEXT).is_none() {
            properties.insert_value(
                prop::TEXT.to_string(),
                Value::String(text.into_owned()).into_unchecked(),
            );
        }

        // apply each property
        for (key, value) in properties.evaluate() {
            self.apply_property(key, value);
        }
    }

    fn apply_property(&mut self, key: String, value: SafeRouteBlob<'_>) {
        match key.as_str() {
            prop::TEXT => {
                validate_not_array_or_object!(&value, &mut self.errors, prop::TEXT.to_string());
                self.line.text = lang::parse_rich(&value.coerce_into_string());
            }
            prop::COMMENT => {
                validate_not_array_or_object!(&value, &mut self.errors, prop::COMMENT.to_string());
                self.line.secondary_text = lang::parse_rich(&value.coerce_into_string());
            }
            prop::NOTES => {
                let value = match value.try_into_object() {
                    Ok(_) => {
                        self.errors
                            .push(CompError::InvalidLinePropertyType(prop::NOTES.to_string()));
                        return;
                    }
                    Err(value) => value,
                };
                match value.try_into_array() {
                    Ok(array) => {
                        for (i, v) in array.into_iter().enumerate() {
                            self.compile_note(&format!("{p}[{i}]", p = prop::NOTES), v);
                        }
                    }
                    Err(_) => {
                        self.compile_note(prop::NOTES, value);
                    }
                }
            }
            prop::SPLIT_NAME => {
                if validate_not_array_or_object!(&value, &mut self.errors, prop::SPLIT_NAME.to_string())
                {
                    self.line.split_name = Some(lang::parse_rich(&value.coerce_to_string()));
                }
            }
            prop::ICON_DOC => {
                if validate_not_array_or_object!(&value, &mut self.errors, prop::ICON_DOC.to_string()) {
                    if value.coerce_truthy() {
                        self.line.doc_icon = Some(value.coerce_into_string());
                    } else {
                        self.line.doc_icon = None;
                    }
                }
            }
            prop::ICON_MAP => {
                if validate_not_array_or_object!(&value, &mut self.errors, prop::ICON_MAP.to_string()) {
                    if value.coerce_truthy() {
                        self.line.map_icon = Some(value.coerce_to_string());
                    } else {
                        self.line.map_icon = None;
                    }
                }
            }
            prop::ICON_PRIORITY => {
                if validate_not_array_or_object!(
                    &value,
                    &mut self.errors,
                    prop::ICON_PRIORITY.to_string()
                ) {
                    if let Some(i) = value.try_coerce_to_i64() {
                        self.line.map_icon_priority = i;
                    } else {
                        self.errors.push(CompError::InvalidLinePropertyType(
                            prop::ICON_PRIORITY.to_string(),
                        ));
                    }
                }
            }
            prop::COUNTER => {
                if validate_not_array_or_object!(&value, &mut self.errors, prop::COUNTER.to_string()) {
                    let text = value.coerce_into_string();
                    if !text.is_empty() {
                        let mut blocks = lang::parse_rich(&text).into_iter();
                        if let Some(first) = blocks.next() {
                            self.line.counter_text = Some(first);
                        }
                        if blocks.next().is_some() {
                            self.errors.push(CompError::TooManyTagsInCounter);
                        }
                    }
                }
            }
            prop::COLOR => {
                if validate_not_array_or_object!(&value, &mut self.errors, prop::COLOR.to_string()) {
                    if !value.coerce_truthy() {
                        self.line.line_color = None;
                    } else {
                        self.line.line_color = Some(value.coerce_into_string());
                    }
                }
            }
            prop::MOVEMENTS => {
                let array = match value.try_into_array() {
                    Err(_) => {
                        self.errors.push(CompError::InvalidLinePropertyType(
                            prop::MOVEMENTS.to_string(),
                        ));
                        return;
                    }
                    Ok(array) => array,
                };
                // // need to track the coordinate of the final position with a stack
                // let mut ref_stack = vec![];
                for (i, v) in array.into_iter().enumerate() {
                    self.compile_movement(&format!("{p}[{i}]", p = prop::MOVEMENTS), v);
                }
                //     {
                //                 match &m {
                //                     CompMovement::Push => {
                //                         if let Some(i) = ref_stack.last() {
                //                             ref_stack.push(*i);
                //                         }
                //                     }
                //                     CompMovement::Pop => {
                //                         ref_stack.pop();
                //                     }
                //                     _ => match ref_stack.last_mut() {
                //                         Some(i) => *i = output.movements.len(),
                //                         None => ref_stack.push(output.movements.len()),
                //                     },
                //                 }
                //                 output.movements.push(m);
                //             }
                //         }
                //         if let Some(i) = ref_stack.last() {
                //             if let CompMovement::To { to, .. } = &output.movements[*i] {
                //                 output.map_coord = to.clone();
                //                 self.coord = to.clone();
                //             } else {
                //                 unreachable!();
                //             }
                //         }
                // match value {
                //     Value::Array(array) => {
                //     }
                // }
            }
            prop::MARKERS => match value.try_into_array() {
                Ok(array) => {
                    for (i, v) in array.into_iter().enumerate() {
                        self.compile_marker(&format!("{p}[{i}]", p = prop::MARKERS), v);
                    }
                }
                _ => self.errors.push(CompError::InvalidLinePropertyType(
                    prop::MARKERS.to_string(),
                )),
            },
            prop::BANNER => match value.try_coerce_to_bool() {
                Some(value) => self.line.is_banner = value,
                None => {
                    self.errors
                        .push(CompError::InvalidLinePropertyType(prop::BANNER.to_string()));
                }
            },
            _ => {
                self.line.properties.insert(key, value.into());
            }
        }
    }
}
