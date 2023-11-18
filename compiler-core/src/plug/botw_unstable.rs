//! Temporary solution for botw specific plugin features before JS plugin engine can be implemented

use serde_json::Value;

use crate::types::{GameCoord, DocRichText, DocDiagnostic, DocRichTextBlock};
use crate::comp::{CompLine, CompDoc, CompMovement};
use crate::json::Coerce;
use crate::{lang, CompilerMetadata};
use crate::macros::async_trait;

use super::{PluginRuntime, operation, PlugResult};

const FURY: &str = "fury";
const GALE: &str = "gale";
const FURY_PLUS: &str = "fury-plus";
const GALE_PLUS: &str = "gale-plus";
const TIME_OVERRIDE: &str = "time-override";
const ESTIMATE_RECHARGE: &str = "estimate-recharge";
const MULTIPLIER: &str = "multiplier";
const DIR: &str = "dir";

const MAX_USE: u32 = 3;
/// Gale recharges in 6 minutes without buffs
const GALE_RECHARGE_SECONDS: u32 = 360;
/// Fury recharges in 12 minutes without buffs
const FURY_RECHARGE_SECONDS: u32 = 720;

/// Castle area is a sphere with radius 350
const CASTLE_X: f64 = -254.0;
const CASTLE_Y: f64 = 194.78;
const CASTLE_Z: f64 = -946.37;
const CASTLE_RADIUS: f64 = 350.0;
fn is_in_castle(coord: &GameCoord) -> bool {
    let dx = coord.0 - CASTLE_X;
    let dy = coord.1 - CASTLE_Y;
    let dz = coord.2 - CASTLE_Z;
    let distsq = dx * dx + dy * dy + dz * dz;
    distsq < CASTLE_RADIUS * CASTLE_RADIUS
}

pub struct BotwAbilityUnstablePlugin {
    /// If recharge time should be estimated
    estimate_recharge: bool,
    /// Multiplier for the time estimate
    multiplier: f64,
    /// Currently in castle
    in_castle: bool,
    /// Time left to recharge gale
    gale_recharge_left: u32,
    /// Time left to recharge fury
    fury_recharge_left: u32,
    /// Number of uses left for gale
    gale_uses_left: u32,
    /// Number of uses left for fury
    fury_uses_left: u32,
    /// If gale plus is obtained
    gale_plus: bool,
    /// If fury plus is obtained
    fury_plus: bool,

}

impl BotwAbilityUnstablePlugin {
    pub fn from_props(props: &Value) -> Self {
        let mut plugin = BotwAbilityUnstablePlugin {
            estimate_recharge: false,
            multiplier: 1.0,
            in_castle: false,
            gale_recharge_left: GALE_RECHARGE_SECONDS,
            fury_recharge_left: FURY_RECHARGE_SECONDS,
            gale_uses_left: MAX_USE,
            fury_uses_left: MAX_USE,
            gale_plus: false,
            fury_plus: false,
        };
        if let Some(m) = props.as_object() {
            if let Some(x) = m.get(ESTIMATE_RECHARGE) {
                plugin.estimate_recharge = x.coerce_truthy();
            }
            if let Some(x) = m.get(MULTIPLIER) {
                if let Some(x) = x.try_coerce_to_f64() {
                    plugin.multiplier = x;
                }
            }
        }
        plugin
    }
    fn set_in_castle(&mut self, in_castle: bool) {
        if self.in_castle == in_castle {
            return;
        }
        if self.in_castle {
            // exit castle, recharge times are 3 times longer
            self.gale_recharge_left *= 3;
            self.fury_recharge_left *= 3;
        } else {
            // enter castle, recharge times are 3 times shorter
            self.gale_recharge_left /= 3;
            self.fury_recharge_left /= 3;
        }
    }

    fn update_recharge(&mut self, seconds: u32) {
        if self.gale_uses_left <= 0 {
            if self.estimate_recharge {
                self.gale_recharge_left -= seconds;
                if self.gale_recharge_left <= 0 {
                    let was_in_castle = self.in_castle;
                    if was_in_castle {
                        self.set_in_castle(false);
                    }
                    self.gale_uses_left = MAX_USE;
                    self.gale_recharge_left = GALE_RECHARGE_SECONDS;
                    if self.gale_plus {
                        self.gale_recharge_left /= 3;
                    }
                    self.gale_recharge_left = (self.gale_recharge_left as f64 * self.multiplier) as u32;
                    if was_in_castle {
                        self.set_in_castle(true);
                    }
                }
            } else {
                self.gale_uses_left = MAX_USE;
            }
        }
        if self.fury_uses_left <= 0 {
            if self.estimate_recharge {
                self.fury_recharge_left -= seconds;
                if self.fury_recharge_left <= 0 {
                    let was_in_castle = self.in_castle;
                    if was_in_castle {
                        self.set_in_castle(false);
                    }
                    self.fury_uses_left = MAX_USE;
                    self.fury_recharge_left = FURY_RECHARGE_SECONDS;
                    if self.fury_plus {
                        self.fury_recharge_left /= 3;
                    }
                    self.fury_recharge_left = (self.fury_recharge_left as f64 * self.multiplier) as u32;
                    if was_in_castle {
                        self.set_in_castle(true);
                    }
                }
            } else {
                self.fury_uses_left = MAX_USE;
            }
        }
    }


    fn process_line(&mut self, line: &mut CompLine) {
        // consume the property regardless of if we are using it
        let time_override = line.properties.remove(TIME_OVERRIDE);
        if let Some(x) = line.properties.remove(GALE_PLUS) {
            if !self.gale_plus && x.coerce_truthy() {
                self.gale_recharge_left /= 3;
                self.gale_plus = true;
            }
        }
        if let Some(x) = line.properties.remove(FURY_PLUS) {
            if !self.fury_plus && x.coerce_truthy() {
                self.fury_recharge_left /= 3;
                self.fury_plus = true;
            }
        }
        let gale_override = match line.properties.remove(GALE) {
            Some(x) => match x.try_coerce_to_u32() {
                None => {
                    line.diagnostics.push(DocDiagnostic {
                        msg: lang::parse_poor("`gale` must be a non-negative integer"),
                        msg_type: "error".to_string(),
                        source: "plugin/botw-ability-unstable".to_string()
                    });
                    None
                }
                x => x
            },
            _ => None
        };
        let fury_override = match line.properties.remove(FURY) {
            Some(x) => match x.try_coerce_to_u32() {
                None => {
                    line.diagnostics.push(DocDiagnostic {
                        msg: lang::parse_poor("`fury` must be a non-negative integer"),
                        msg_type: "error".to_string(),
                        source: "plugin/botw-ability-unstable".to_string()
                    });
                    None
                }
                x => x
            },
            _ => None
        };
        if self.estimate_recharge {
            let time = match time_override {
                Some(x) => match x.try_coerce_to_u32() {
                    None => {
                        line.diagnostics.push(DocDiagnostic {
                            msg: lang::parse_poor("`time-override` must be a non-negative integer"),
                            msg_type: "error".to_string(),
                            source: "plugin/botw-ability-unstable".to_string()
                        });
                        None
                    }
                    x => x
                },
                _ => None
            };
            let time = time.unwrap_or_else(|| estimate_time(&line.text));
            self.update_recharge(time);
        } else {
            self.update_recharge(0);
        }

        let mut new_in_castle = false;
        for movement in &line.movements {
            if let CompMovement::To{to, ..} = movement {
                new_in_castle = is_in_castle(to);
                if new_in_castle {
                    break;
                }
            }
        }
        self.set_in_castle(new_in_castle);

        operation::for_each_rich_text_except_counter!(block in line {
            self.process_block(block, &gale_override, &fury_override, &mut line.diagnostics);
        });
    }

    fn process_block(&mut self, block: &mut DocRichTextBlock, gale_override: &Option<u32>, fury_override: &Option<u32>, diagnostics: &mut Vec<DocDiagnostic>) {
        match &block.tag {
            Some(x) if x == GALE => {
                let count = get_ability_use(&block.text, gale_override, diagnostics);
                if let Some(count) = count {
                    let text = get_use_ability_string("GALE", &mut self.gale_uses_left, count);
                    if let Some(text) = text {
                        block.text = text;
                    } else {
                        block.text = "GALE ?".to_string();
                        add_usage_warning(GALE, self.gale_uses_left, count, self.gale_recharge_left, diagnostics);
                    }
                }
            },
            Some(x) if x == FURY => {
                let count = get_ability_use(&block.text, fury_override, diagnostics);
                if let Some(count) = count {
                    let text = get_use_ability_string("FURY", &mut self.fury_uses_left, count);
                    if let Some(text) = text {
                        block.text = text;
                    } else {
                        block.text = "FURY ?".to_string();
                        add_usage_warning(GALE, self.gale_uses_left, count, self.gale_recharge_left, diagnostics);
                    }
                }
            },
            _ => {}
        }
    }

}
fn add_usage_warning(ability: &str, current: u32, need: u32, time_need: u32, diagnostics: &mut Vec<DocDiagnostic>) {
    if current == 0 {
        diagnostics.push(DocDiagnostic {
            msg: lang::parse_poor(&format!("{ability} may not be recharged yet. May need {time_need} more seconds to recharge. Note that this is an estimate and may not be accurate.")),
            msg_type: "warning".to_string(),
            source: "plugin/botw-ability-unstable".to_string()
        });
    } else {
        diagnostics.push(DocDiagnostic {
            msg: lang::parse_poor(&format!("not enough {ability}! Need to use {need}, but only {current} left.")),
            msg_type: "warning".to_string(),
            source: "plugin/botw-ability-unstable".to_string()
        });
    }
}


fn get_ability_use(text: &str, count_override: &Option<u32>, diagnostics: &mut Vec<DocDiagnostic>) -> Option<u32> {
    let count = if text == "" {
        match count_override {
            Some(x) => *x,
            _ => {
                diagnostics.push(DocDiagnostic { 
                    msg: lang::parse_poor("ability use count must be specified in the tag or as a property!"),
                    msg_type: "error".to_string(),
                    source: "plugin/botw-ability-unstable".to_string()
                });
                    return None;
            }
        }
    } else {
        match text.parse::<u32>() {
            Ok(x) => x,
            Err(_) => {
                diagnostics.push(DocDiagnostic { 
                    msg: lang::parse_poor("ability use count must be a non-negative integer!"),
                    msg_type: "error".to_string(),
                    source: "plugin/botw-ability-unstable".to_string()
                });
                return None;
            }
        }
    };
    if count > MAX_USE {
        diagnostics.push(DocDiagnostic { 
            msg: lang::parse_poor("ability use count must be between 0 and 3!"),
            msg_type: "error".to_string(),
            source: "plugin/botw-ability-unstable".to_string()
        });
        return None;
    }
    Some(count)
}


/// Get ability string.
///
/// For example:
/// - prefix is "FURY", uses_left is 2, need is 1, then the result is "FURY 2".
/// - prefix is "GALE", uses_left is 2, need is 2, then the result is "GALE 2-3".
///
/// Return None if need is more than uses_left.
fn get_use_ability_string(prefix: &str, uses_left: &mut u32, need: u32) -> Option<String> {
    let current = *uses_left;
    if current < need {
        return None;
    }
    let text = match current {
        1 => {
            // current 1, need 1
            format!("{prefix} 3")
        }
        2 => {
            // current 2
            match need {
                1 => {
                    // need 1
                    format!("{prefix} 2")
                }
                _ => {
                    // need 2
                    format!("{prefix} 2-3")
                }
            }
        }
        _ => {
            // current 3
            format!("{prefix} 1-{need}")
        }
    };
    *uses_left -= need;
    Some(text)
}

fn estimate_time(text: &DocRichText) -> u32 {
    let mut movement_count = 0;
    for block in text.iter() {
        if let Some(t) = &block.tag {
            if t == DIR {
                movement_count += 1;
            }
        }
    }
    return movement_count * 14 + 6; // (approximately) same timing as old celer
}

#[async_trait(?Send)]
impl PluginRuntime for BotwAbilityUnstablePlugin {
    async fn on_compile(&mut self, _: &CompilerMetadata, comp_doc: &mut CompDoc) -> PlugResult<()> {
        operation::for_each_line!(line in comp_doc {
            self.process_line(&mut line);
            line
        });
        Ok(())
    }
}