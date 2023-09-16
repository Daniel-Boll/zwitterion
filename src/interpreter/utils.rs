use std::io::Write;

use super::{AnonymousFunction, Value};

impl std::fmt::Debug for AnonymousFunction {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "<#closure>")
  }
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Value::Int(int) => write!(f, "{}", int),
      Value::Str(string) => write!(f, "{}", string),
      Value::Bool(bool) => write!(f, "{}", bool),
      Value::Tuple(first, second) => write!(f, "({}, {})", first, second),
      Value::Closure(_) => write!(f, "<#closure>"),
    }
  }
}

pub(crate) fn panic<W: Write>(message: String, writer: &mut W) -> ! {
  writer
    .write_all(format!("{}\n", message).as_bytes())
    .unwrap();
  panic!();
}
