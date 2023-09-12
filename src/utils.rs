use rinha::rinha;
use std::panic::{self, AssertUnwindSafe};

use crate::interpreter;

pub fn interpret_to_buffer(code: &str) -> String {
  let mut buffer = Vec::new();

  let _ = panic::catch_unwind(AssertUnwindSafe(|| {
    let mut errors = vec![];
    let ast = rinha::FileParser::new()
      .parse(&mut errors, "inline-code", code)
      .unwrap();

    interpreter::from_ast(ast.expression, &mut buffer);
  }));

  String::from_utf8(buffer).unwrap()
}
