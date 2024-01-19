#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
/*
  Generated with regen-lang
*/
regen::sdk!(
  context: ();
  target: Block;
  tokens: [
    TSymbol,
    TEscape,
    TIdentifier,
    TSpace,
  ];
  rules: [
    lex::Rule::Regex(Regex::new(r"^\s+").unwrap(), lex::Target::Keep(Tok::TSpace)),
    lex::Rule::Regex(Regex::new(r"^[\.()]").unwrap(), lex::Target::Keep(Tok::TSymbol)),
    lex::Rule::Regex(Regex::new(r"^\\[\\\.)]?").unwrap(), lex::Target::Keep(Tok::TEscape)),
    lex::Rule::Regex(Regex::new(r"^[^\\\.()\s]+").unwrap(), lex::Target::Keep(Tok::TIdentifier)),
  ];
  semantics: [
    SText,
    STag,
    SParen,
    SArg,
  ];
);
pub mod ast {
  use super::*;
  #[derive(Debug)] pub enum Block {
    Text(Box<Text>),
    TagExp(Box<TagExp>),
    Symbol(Box<Symbol>),
    Space(Box<Space>),
  }
  #[derive(Debug)] pub struct Space {
    pub m_t: Token,
  }
  #[derive(Debug)] pub struct Symbol {
    pub m_t: Token,
  }
  #[derive(Debug)] pub struct TagExp {
    pub m_0: Token,
    pub m_tag: Token,
    pub m_2: Token,
    pub m_space: Option<Token>,
    pub m_arg: Vec<UnitInsideTag>,
    pub m_5: Token,
  }
  #[derive(Debug)] pub struct Text {
    pub m_t: Vec<Unit>,
  }
  #[derive(Debug)] pub enum Unit {
    UnitId(Box<UnitId>),
    UnitEscape(Box<UnitEscape>),
  }
  #[derive(Debug)] pub struct UnitDotSymbol {
    pub m_0: Token,
    pub m_s: Option<Token>,
  }
  #[derive(Debug)] pub struct UnitEscape {
    pub m_t: Token,
    pub m_s: Option<Token>,
  }
  #[derive(Debug)] pub struct UnitId {
    pub m_t: Token,
    pub m_s: Option<Token>,
  }
  #[derive(Debug)] pub enum UnitInsideTag {
    Unit(Box<Unit>),
    UnitDotSymbol(Box<UnitDotSymbol>),
    UnitOpenParenSymbol(Box<UnitOpenParenSymbol>),
  }
  #[derive(Debug)] pub struct UnitOpenParenSymbol {
    pub m_0: Token,
    pub m_s: Option<Token>,
  }
}
pub mod pt {
  use super::*;
  #[derive(Debug)] pub enum Block<'p> { 
    Text(Box<pt::Text<'p>>),
    TagExp(Box<pt::TagExp<'p>>),
    Symbol(Box<pt::Symbol<'p>>),
    Space(Box<pt::Space<'p>>),
  }
  #[derive(Debug)] pub struct Space<'p> {
    pub ast: &'p ast::Space,
    pub m_t: String,
  }
  #[derive(Debug)] pub struct Symbol<'p> {
    pub ast: &'p ast::Symbol,
    pub m_t: String,
  }
  #[derive(Debug)] pub struct TagExp<'p> {
    pub ast: &'p ast::TagExp,
    pub m_tag: String,
    pub m_space: Option<String>,
    pub m_arg: Vec<pt::UnitInsideTag<'p>>,
  }
  #[derive(Debug)] pub struct Text<'p> {
    pub ast: &'p ast::Text,
    pub m_t: Vec<pt::Unit<'p>>,
  }
  #[derive(Debug)] pub enum Unit<'p> { 
    UnitId(Box<pt::UnitId<'p>>),
    UnitEscape(Box<pt::UnitEscape<'p>>),
  }
  #[derive(Debug)] pub struct UnitDotSymbol<'p> {
    pub ast: &'p ast::UnitDotSymbol,
    pub m_s: Option<String>,
  }
  #[derive(Debug)] pub struct UnitEscape<'p> {
    pub ast: &'p ast::UnitEscape,
    pub m_t: String,
    pub m_s: Option<String>,
  }
  #[derive(Debug)] pub struct UnitId<'p> {
    pub ast: &'p ast::UnitId,
    pub m_t: String,
    pub m_s: Option<String>,
  }
  #[derive(Debug)] pub enum UnitInsideTag<'p> { 
    Unit(Box<pt::Unit<'p>>),
    UnitDotSymbol(Box<pt::UnitDotSymbol<'p>>),
    UnitOpenParenSymbol(Box<pt::UnitOpenParenSymbol<'p>>),
  }
  #[derive(Debug)] pub struct UnitOpenParenSymbol<'p> {
    pub ast: &'p ast::UnitOpenParenSymbol,
    pub m_s: Option<String>,
  }
}
regen::impl_union!(from_ast, Block, {
  Text,
  TagExp,
  Symbol,
  Space,
});
impl ast::Space {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_t: required!(ts, token!(TSpace::parse(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    if let Some(o) = _ovr { si.set(&self.m_t, o.clone()); }
  }
}
impl<'p> pt::Space<'p> {
  fn from_ast(ast: &'p ast::Space, _ctx: &mut Ctx) -> Self {
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
    si.set(&self.m_t, _ovr.as_ref().cloned().unwrap_or(Tok::SText));
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
impl ast::TagExp {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_0: required!(ts, token!(TSymbol::"."(ts)))?,
      m_tag: required!(ts, token!(TIdentifier::parse(ts)))?,
      m_2: required!(ts, token!(TSymbol::"("(ts)))?,
      m_space: optional!(ts, token!(TSpace::parse(ts))),
      m_arg: { let mut v = vec![]; list!(ts, v, ast::UnitInsideTag::parse(ts)) },
      m_5: required!(ts, token!(TSymbol::")"(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    if let Some(o) = _ovr { si.set(&self.m_0, o.clone()); }
    si.set(&self.m_tag, _ovr.as_ref().cloned().unwrap_or(Tok::STag));
    if let Some(o) = _ovr { si.set(&self.m_2, o.clone()); }
    if let Some(o) = _ovr { if let Some(m) = &self.m_space { si.set(m, o.clone()); } }
    for m in &self.m_arg { m.apply_semantic(si, &Some(Tok::SArg)); }
    if let Some(o) = _ovr { si.set(&self.m_5, o.clone()); }
  }
}
impl<'p> pt::TagExp<'p> {
  fn from_ast(ast: &'p ast::TagExp, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_tag: ast.m_tag.value.clone(),
      m_space: ast.m_space.as_ref().map(|t| t.value.clone()),
      m_arg: ast.m_arg.iter().map(|x| pt::UnitInsideTag::from_ast(x, _ctx)).collect::<Vec<_>>(),
    }
  }
}
impl ast::Text {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_t: { let mut v = vec![required!(ts, ast::Unit::parse(ts))?]; list!(ts, v, ast::Unit::parse(ts)) },
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    for m in &self.m_t { m.apply_semantic(si, &Some(Tok::SText)); }
  }
}
impl<'p> pt::Text<'p> {
  fn from_ast(ast: &'p ast::Text, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_t: ast.m_t.iter().map(|x| pt::Unit::from_ast(x, _ctx)).collect::<Vec<_>>(),
    }
  }
}
regen::impl_union!(from_ast, Unit, {
  UnitId,
  UnitEscape,
});
impl ast::UnitDotSymbol {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_0: required!(ts, token!(TSymbol::"."(ts)))?,
      m_s: optional!(ts, token!(TSpace::parse(ts))),
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    si.set(&self.m_0, _ovr.as_ref().cloned().unwrap_or(Tok::SText));
    if let Some(o) = _ovr { if let Some(m) = &self.m_s { si.set(m, o.clone()); } }
  }
}
impl<'p> pt::UnitDotSymbol<'p> {
  fn from_ast(ast: &'p ast::UnitDotSymbol, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_s: ast.m_s.as_ref().map(|t| t.value.clone()),
    }
  }
}
impl ast::UnitEscape {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_t: required!(ts, token!(TEscape::parse(ts)))?,
      m_s: optional!(ts, token!(TSpace::parse(ts))),
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    if let Some(o) = _ovr { si.set(&self.m_t, o.clone()); }
    if let Some(o) = _ovr { if let Some(m) = &self.m_s { si.set(m, o.clone()); } }
  }
}
impl<'p> pt::UnitEscape<'p> {
  fn from_ast(ast: &'p ast::UnitEscape, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_t: ast.m_t.value.clone(),
      m_s: ast.m_s.as_ref().map(|t| t.value.clone()),
    }
  }
}
impl ast::UnitId {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_t: required!(ts, token!(TIdentifier::parse(ts)))?,
      m_s: optional!(ts, token!(TSpace::parse(ts))),
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    if let Some(o) = _ovr { si.set(&self.m_t, o.clone()); }
    if let Some(o) = _ovr { if let Some(m) = &self.m_s { si.set(m, o.clone()); } }
  }
}
impl<'p> pt::UnitId<'p> {
  fn from_ast(ast: &'p ast::UnitId, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_t: ast.m_t.value.clone(),
      m_s: ast.m_s.as_ref().map(|t| t.value.clone()),
    }
  }
}
regen::impl_union!(from_ast, UnitInsideTag, {
  Unit,
  UnitDotSymbol,
  UnitOpenParenSymbol,
});
impl ast::UnitOpenParenSymbol {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_0: required!(ts, token!(TSymbol::"("(ts)))?,
      m_s: optional!(ts, token!(TSpace::parse(ts))),
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    si.set(&self.m_0, _ovr.as_ref().cloned().unwrap_or(Tok::SText));
    if let Some(o) = _ovr { if let Some(m) = &self.m_s { si.set(m, o.clone()); } }
  }
}
impl<'p> pt::UnitOpenParenSymbol<'p> {
  fn from_ast(ast: &'p ast::UnitOpenParenSymbol, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_s: ast.m_s.as_ref().map(|t| t.value.clone()),
    }
  }
}
