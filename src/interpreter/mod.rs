use std::io::Write;

use rinha::{ast, parser};

pub mod binary;
pub mod env;
pub mod eval;
pub mod utils;

#[derive(Debug, Clone)]
pub enum Value {
  Int(i64),
  Str(String),
  Bool(bool),
  Tuple(Box<Value>, Box<Value>),
  Closure(AnonymousFunction),
}

#[derive(Clone)]
pub struct AnonymousFunction {
  pub parameters: Vec<parser::Var>,
  pub body: Box<ast::Term>,
  pub env: env::Env,
}

impl AnonymousFunction {
  pub fn new(parameters: Vec<parser::Var>, body: Box<ast::Term>, env: env::Env) -> Self {
    Self {
      parameters,
      body,
      env,
    }
  }
}

pub fn from_ast<W: Write>(term: ast::Term, mut writer: W) {
  let mut env = env::Env::default();

  eval::eval(term, &mut env, &mut writer);
}
