#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
/*
  Generated with regen-lang
*/
regen::sdk!(
  context: ();
  target: Preset;
  tokens: [
    TText,
    TEscape,
    TSymbol,
  ];
  rules: [
    lex::Rule::Regex(Regex::new(r"^[:,<>]").unwrap(), lex::Target::Keep(Tok::TSymbol)),
    lex::Rule::Regex(Regex::new(r"^[^:,<>\\]+").unwrap(), lex::Target::Keep(Tok::TText)),
    lex::Rule::Regex(Regex::new(r"^\\[\\,>]?").unwrap(), lex::Target::Keep(Tok::TEscape)),
  ];
  semantics: [
    SNamespace,
    SArg,
  ];
);
pub mod ast {
  use super::*;
  #[derive(Debug)] pub struct Arg {
    pub m_t: Token,
  }
  #[derive(Debug)] pub enum ArgBlock {
    Arg(Box<Arg>),
    ArgEscape(Box<ArgEscape>),
    ArgSymbol(Box<ArgSymbol>),
  }
  #[derive(Debug)] pub struct ArgEscape {
    pub m_t: Token,
  }
  #[derive(Debug)] pub struct ArgListTail {
    pub m_0: Token,
    pub m_arg: Option<Box<ArgText>>,
  }
  #[derive(Debug)] pub struct ArgSymbol {
    pub m_t_0: Token,
  }
  #[derive(Debug)] pub struct ArgText {
    pub m_blocks: Vec<ArgBlock>,
  }
  #[derive(Debug)] pub struct Args {
    pub m_0: Token,
    pub m_first: Option<Box<ArgText>>,
    pub m_rest: Vec<ArgListTail>,
    pub m_3: Token,
  }
  #[derive(Debug)] pub struct Preset {
    pub m_namespace: Token,
    pub m_sub_namespaces: Vec<SubNamespace>,
    pub m_args: Option<Box<Args>>,
  }
  #[derive(Debug)] pub struct SubNamespace {
    pub m_0: Token,
    pub m_1: Token,
    pub m_name: Token,
  }
}
pub mod pt {
  use super::*;
  #[derive(Debug)] pub struct Arg<'p> {
    pub ast: &'p ast::Arg,
    pub m_t: String,
  }
  #[derive(Debug)] pub enum ArgBlock<'p> { 
    Arg(Box<pt::Arg<'p>>),
    ArgEscape(Box<pt::ArgEscape<'p>>),
    ArgSymbol(Box<pt::ArgSymbol<'p>>),
  }
  #[derive(Debug)] pub struct ArgEscape<'p> {
    pub ast: &'p ast::ArgEscape,
    pub m_t: String,
  }
  #[derive(Debug)] pub struct ArgListTail<'p> {
    pub ast: &'p ast::ArgListTail,
    pub m_arg: Option<Box<pt::ArgText<'p>>>,
  }
  #[derive(Debug)] pub struct ArgSymbol<'p> {
    pub ast: &'p ast::ArgSymbol,
  }
  #[derive(Debug)] pub struct ArgText<'p> {
    pub ast: &'p ast::ArgText,
    pub m_blocks: Vec<pt::ArgBlock<'p>>,
  }
  #[derive(Debug)] pub struct Args<'p> {
    pub ast: &'p ast::Args,
    pub m_first: Option<Box<pt::ArgText<'p>>>,
    pub m_rest: Vec<pt::ArgListTail<'p>>,
  }
  #[derive(Debug)] pub struct Preset<'p> {
    pub ast: &'p ast::Preset,
    pub m_namespace: String,
    pub m_sub_namespaces: Vec<pt::SubNamespace<'p>>,
    pub m_args: Option<Box<pt::Args<'p>>>,
  }
  #[derive(Debug)] pub struct SubNamespace<'p> {
    pub ast: &'p ast::SubNamespace,
    pub m_name: String,
  }
}
impl ast::Arg {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_t: required!(ts, token!(TText::parse(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    si.set(&self.m_t, _ovr.as_ref().cloned().unwrap_or(Tok::SArg));
  }
}
impl<'p> pt::Arg<'p> {
  fn from_ast(ast: &'p ast::Arg, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_t: ast.m_t.value.clone(),
    }
  }
}
regen::impl_union!(from_ast, ArgBlock, {
  Arg,
  ArgEscape,
  ArgSymbol,
});
impl ast::ArgEscape {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_t: required!(ts, token!(TEscape::parse(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    if let Some(o) = _ovr { si.set(&self.m_t, o.clone()); }
  }
}
impl<'p> pt::ArgEscape<'p> {
  fn from_ast(ast: &'p ast::ArgEscape, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_t: ast.m_t.value.clone(),
    }
  }
}
impl ast::ArgListTail {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_0: required!(ts, token!(TSymbol::","(ts)))?,
      m_arg: (optional!(ts, ast::ArgText::parse(ts))).map(Box::new),
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    if let Some(o) = _ovr { si.set(&self.m_0, o.clone()); }
    if let Some(m) = &self.m_arg { m.apply_semantic(si, _ovr); }
  }
}
impl<'p> pt::ArgListTail<'p> {
  fn from_ast(ast: &'p ast::ArgListTail, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_arg: ast.m_arg.as_ref().map(|x| Box::new(pt::ArgText::from_ast(x, _ctx))),
    }
  }
}
impl ast::ArgSymbol {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_t_0: required!(ts, token!(TSymbol::":"(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    si.set(&self.m_t_0, _ovr.as_ref().cloned().unwrap_or(Tok::SArg));
  }
}
impl<'p> pt::ArgSymbol<'p> {
  fn from_ast(ast: &'p ast::ArgSymbol, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
    }
  }
}
impl ast::ArgText {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_blocks: { let mut v = vec![required!(ts, ast::ArgBlock::parse(ts))?]; list!(ts, v, ast::ArgBlock::parse(ts)) },
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    for m in &self.m_blocks { m.apply_semantic(si, _ovr); }
  }
}
impl<'p> pt::ArgText<'p> {
  fn from_ast(ast: &'p ast::ArgText, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_blocks: ast.m_blocks.iter().map(|x| pt::ArgBlock::from_ast(x, _ctx)).collect::<Vec<_>>(),
    }
  }
}
impl ast::Args {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_0: required!(ts, token!(TSymbol::"<"(ts)))?,
      m_first: (optional!(ts, ast::ArgText::parse(ts))).map(Box::new),
      m_rest: { let mut v = vec![]; list!(ts, v, ast::ArgListTail::parse(ts)) },
      m_3: required!(ts, token!(TSymbol::">"(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    if let Some(o) = _ovr { si.set(&self.m_0, o.clone()); }
    if let Some(m) = &self.m_first { m.apply_semantic(si, _ovr); }
    for m in &self.m_rest { m.apply_semantic(si, _ovr); }
    if let Some(o) = _ovr { si.set(&self.m_3, o.clone()); }
  }
}
impl<'p> pt::Args<'p> {
  fn from_ast(ast: &'p ast::Args, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_first: ast.m_first.as_ref().map(|x| Box::new(pt::ArgText::from_ast(x, _ctx))),
      m_rest: ast.m_rest.iter().map(|x| pt::ArgListTail::from_ast(x, _ctx)).collect::<Vec<_>>(),
    }
  }
}
impl ast::Preset {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_namespace: required!(ts, token!(TText::parse(ts)))?,
      m_sub_namespaces: { let mut v = vec![]; list!(ts, v, ast::SubNamespace::parse(ts)) },
      m_args: (optional!(ts, ast::Args::parse(ts))).map(Box::new),
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    si.set(&self.m_namespace, _ovr.as_ref().cloned().unwrap_or(Tok::SNamespace));
    for m in &self.m_sub_namespaces { m.apply_semantic(si, _ovr); }
    if let Some(m) = &self.m_args { m.apply_semantic(si, _ovr); }
  }
}
impl<'p> pt::Preset<'p> {
  fn from_ast(ast: &'p ast::Preset, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_namespace: ast.m_namespace.value.clone(),
      m_sub_namespaces: ast.m_sub_namespaces.iter().map(|x| pt::SubNamespace::from_ast(x, _ctx)).collect::<Vec<_>>(),
      m_args: ast.m_args.as_ref().map(|x| Box::new(pt::Args::from_ast(x, _ctx))),
    }
  }
}
impl ast::SubNamespace {
  pub fn parse(ts: &mut TokenStream<Tok>) -> Option<Self> {
    Some(Self {
      m_0: required!(ts, token!(TSymbol::":"(ts)))?,
      m_1: required!(ts, token!(TSymbol::":"(ts)))?,
      m_name: required!(ts, token!(TText::parse(ts)))?,
    })
  }
  pub fn apply_semantic(&self, si: &mut TokenBlocks<Tok>, _ovr: &Option<Tok>) {
    if let Some(o) = _ovr { si.set(&self.m_0, o.clone()); }
    if let Some(o) = _ovr { si.set(&self.m_1, o.clone()); }
    si.set(&self.m_name, _ovr.as_ref().cloned().unwrap_or(Tok::SNamespace));
  }
}
impl<'p> pt::SubNamespace<'p> {
  fn from_ast(ast: &'p ast::SubNamespace, _ctx: &mut Ctx) -> Self {
    Self {
      ast,
      m_name: ast.m_name.value.clone(),
    }
  }
}
