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
}

impl AnonymousFunction {
  pub fn new(parameters: Vec<parser::Var>, body: Box<ast::Term>) -> Self {
    Self { parameters, body }
  }
}

pub(crate) struct Interpreter<W: Write> {
  env: env::Env,
  writer: W,
}

impl<W: Write> Interpreter<W> {
  pub(crate) fn new(writer: W) -> Self {
    Self {
      env: env::Env::default(),
      writer,
    }
  }
}

pub fn from_ast<W: Write>(term: ast::Term, writer: W) {
  Interpreter::new(writer).eval(term);
}
