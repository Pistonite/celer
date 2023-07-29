use super::TempStrBlock;
use super::grammar::{pt, Ctx, Tok};

/// Parse parse tree roots to TempStrBlocks
pub fn from_pts(pts: Vec<pt::Block>) -> Vec<TempStrBlock> {
    let mut output = vec![];
    for pt in pts {
        match pt {
            pt::Block::Dollar(mut pt) => {
                let block = pt.take_unchecked();
                match &block {
                    TempStrBlock::Lit(block_str) => {
                        match output.last_mut() {
                            None | Some(TempStrBlock::Var(_)) => {
                                output.push(block);
                            }
                            Some(TempStrBlock::Lit(s)) => {
                                s.push_str(&block_str);
                            }
                        }
                    },
                    TempStrBlock::Var(_) => {
                        output.push(block);
                    }
                }
            }
            pt::Block::NonDollar(mut pt) => match output.last_mut() {
                None | Some(TempStrBlock::Var(_)) => {
                    output.push(TempStrBlock::Lit(pt.take_unchecked()));
                }
                Some(TempStrBlock::Lit(s)) => {
                    s.push_str(&pt.take_unchecked());
                }
            },
        }
    }
    output
}

/// Parse tree hook for the Dollar node
pub fn parse_dollar(pt: &pt::Dollar, ctx: &mut Ctx) -> Option<TempStrBlock> {
    match pt.m_tail.as_ref() {
        None => {
            // just a dollar sign
            Some(TempStrBlock::Lit("$".to_string()))
        },
        Some(pt_tail) => {
            // set the semantic of first dollar sign to be variable
            ctx.tbs.set(&pt.ast.m_0, Tok::SVariable);
            match pt_tail.as_ref() {
                pt::DollarTail::Escape(_) => {
                    // double dollar sign
                    Some(TempStrBlock::Lit("$".to_string()))
                },
                pt::DollarTail::Variable(pt_variable) => {
                    match pt_variable.m_arg.parse::<usize>() {
                        Ok(arg) => {
                            // variable
                            Some(TempStrBlock::Var(arg))
                        },
                        Err(_) => {
                            // cannot parse the number as valid usize
                            // treat as string
                            Some(TempStrBlock::Lit(format!("$({})", pt_variable.m_arg)))
                        }
                    }
                }
            }
        }
    }
}

/// Parse tree hook for the NonDollar node
pub fn parse_non_dollar(pt: &pt::NonDollar, _: &mut Ctx) -> Option<String> {
    match pt {
        pt::NonDollar::Text(pt) => {
            Some(pt.m_t.clone())
        },
        pt::NonDollar::Symbol(pt) => {
            Some(pt.m_t.clone())
        },
        pt::NonDollar::Number(pt) => {
            Some(pt.m_t.clone())
        },
    }
}

