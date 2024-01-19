#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
/*
  Generated with regen-lang
*/
// /* ./grammar.rs.pp */
use super::{parse_dollar, TempStrBlock};

// /* ./grammar.rs.pp */
regen::sdk!(
  context: ();
  target: Block;
  tokens: [
    TText,
    TSymbol,
    TNumber,
  ];
  rules: [
    lex::Rule::Regex(Regex::new(r"^[$()]").unwrap(), lex::Target::Keep(Tok::TSymbol)),
    lex::Rule::Regex(Regex::new(r"^[^$()0-9][^$]*").unwrap(), lex::Target::Keep(Tok::TText)),
    lex::Rule::Regex(Regex::new(r"^[0-9]+").unwrap(), lex::Target::Keep(Tok::TNumber)),
  ];
  semantics: [
    SLiteral,
    SVariable,
  ];
);
pub mod ast {
  use super::*;
  #[derive(Debug)] pub enum Block {
    Dollar(Box<Dollar>),
    NonDollar(Box<NonDollar>),
  }
  #[derive(Debug)] pub struct Dollar {
    pub m_0: Token,
    pub m_tail: Option<Box<DollarTail>>,
  }
  #[derive(Debug)] pub enum DollarTail {
    Escape(Box<Escape>),
    Variable(Box<Variable>),
  }
  #[derive(Debug)] pub struct Escape {
    pub m_0: Token,
  }
  #[derive(Debug)] pub enum NonDollar {
    Text(Box<Text>),
    Number(Box<Number>),
    Symbol(Box<Symbol>),
  }
  #[derive(Debug)] pub struct Number {
    pub m_t: Token,
  }
  #[derive(Debug)] pub struct Symbol {
    pub m_t: Token,
  }
  #[derive(Debug)] pub struct Text {
    pub m_t: Token,
  }
  #[derive(Debug)] pub struct Variable {
    pub m_0: Token,
    pub m_arg: Token,
    pub m_2: Token,
  }
}
pub mod pt {
  use super::*;
  #[derive(Debug)] pub enum Block<'p> { 
    Dollar(Box<ParseHook<TempStrBlock, pt::Dollar<'p>>>),
    NonDollar(Box<pt::NonDollar<'p>>),
  }
  #[derive(Debug)] pub struct Dollar<'p> {
    pub ast: &'p ast::Dollar,
    pub m_tail: Option<Box<pt::DollarTail<'p>>>,
  }
  #[derive(Debug)] pub enum DollarTail<'p> { 
    Escape(Box<pt::Escape<'p>>),
    Variable(Box<pt::Variable<'p>>),
  }
  #[derive(Debug)] pub struct Escape<'p> {
    pub ast: &'p ast::Escape,
  }
  #[derive(Debug)] pub enum NonDollar<'p> { 
    Text(Box<pt::Text<'p>>),
    Number(Box<pt::Number<'p>>),
    Symbol(Box<pt::Symbol<'p>>),
  }
  #[derive(Debug)] pub struct Number<'p> {
    pub ast: &'p ast::Number,
    pub m_t: String,
  }
  #[derive(Debug)] pub struct Symbol<'p> {
    pub ast: &'p ast::Symbol,
    pub m_t: String,
  }
  #[derive(Debug)] pub struct Text<'p> {
    pub ast: &'p ast::Text,
    pub m_t: String,
  }
  #[derive(Debug)] pub struct Variable<'p> {
    pub ast: &'p ast::Variable,
    pub m_arg: String,
  }
}
regen::impl_union!(from_ast, Block, {
  Dollar,
  NonDollar,
});
impl ast::Dollar {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_0: required!(ts, token!(TSymbol::"$"(ts)))?,
      m_tail: (optional!(ts, ast::DollarTail::parse(ts))).map(Box::new),
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    if let Some(o) = _ovr { si.set(&self.m_0, o.clone()); }
    if let Some(m) = &self.m_tail { m.apply_semantic(si, _ovr); }
  }
}
impl<'p> pt::Dollar<'p> {
  fn from_ast_internal(ast: &'p ast::Dollar, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_tail: ast.m_tail.as_ref().map(|x| Box::new(pt::DollarTail::from_ast(x, _ctx))),
    }
  }
  #[inline] #[allow(clippy::unnecessary_mut_passed)] fn from_ast(ast: &'p ast::Dollar, ctx: &mut Ctx) -> ParseHook<TempStrBlock, pt::Dollar<'p>> {
    let mut pt = Self::from_ast_internal(ast, ctx);
    ParseHook { val: parse_dollar(&mut pt, ctx), pt }
  }
}
regen::impl_union!(from_ast, DollarTail, {
  Escape,
  Variable,
});
impl ast::Escape {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_0: required!(ts, token!(TSymbol::"$"(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    si.set(&self.m_0, _ovr.as_ref().cloned().unwrap_or(Tok::SVariable));
  }
}
impl<'p> pt::Escape<'p> {
  fn from_ast(ast: &'p ast::Escape, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
    }
  }
}
regen::impl_union!(from_ast, NonDollar, {
  Text,
  Number,
  Symbol,
});
impl ast::Number {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_t: required!(ts, token!(TNumber::parse(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    si.set(&self.m_t, _ovr.as_ref().cloned().unwrap_or(Tok::SLiteral));
  }
}
impl<'p> pt::Number<'p> {
  fn from_ast(ast: &'p ast::Number, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_t: ast.m_t.value.clone(),
    }
  }
}
impl ast::Symbol {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_t: required!(ts, token!(TSymbol::parse(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    si.set(&self.m_t, _ovr.as_ref().cloned().unwrap_or(Tok::SLiteral));
  }
}
impl<'p> pt::Symbol<'p> {
  fn from_ast(ast: &'p ast::Symbol, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_t: ast.m_t.value.clone(),
    }
  }
}
impl ast::Text {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_t: required!(ts, token!(TText::parse(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    si.set(&self.m_t, _ovr.as_ref().cloned().unwrap_or(Tok::SLiteral));
  }
}
impl<'p> pt::Text<'p> {
  fn from_ast(ast: &'p ast::Text, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_t: ast.m_t.value.clone(),
    }
  }
}
impl ast::Variable {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_0: required!(ts, token!(TSymbol::"("(ts)))?,
      m_arg: required!(ts, token!(TNumber::parse(ts)))?,
      m_2: required!(ts, token!(TSymbol::")"(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    si.set(&self.m_0, _ovr.as_ref().cloned().unwrap_or(Tok::SVariable));
    si.set(&self.m_arg, _ovr.as_ref().cloned().unwrap_or(Tok::SVariable));
    si.set(&self.m_2, _ovr.as_ref().cloned().unwrap_or(Tok::SVariable));
  }
}
impl<'p> pt::Variable<'p> {
  fn from_ast(ast: &'p ast::Variable, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_arg: ast.m_arg.value.clone(),
    }
  }
}
