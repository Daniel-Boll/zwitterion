use std::io::Write;

use rinha::ast::{Binary, BinaryOp};

use super::{env::Env, eval::eval, utils, Value};

pub(crate) fn eval_binary<W: Write>(binary: Binary, env: &mut Env, writer: &mut W) -> Value {
  match binary.op {
    BinaryOp::Add => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left + right),
        (Value::Int(left), Value::Str(right)) => Value::Str(format!("{}{}", left, right)),
        (Value::Str(left), Value::Int(right)) => Value::Str(format!("{}{}", left, right)),
        (Value::Str(left), Value::Str(right)) => Value::Str(format!("{}{}", left, right)),
        _ => utils::panic("Expected int or string".to_string(), writer),
      }
    }
    BinaryOp::Sub => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left - right),
        _ => utils::panic("Expected int".to_string(), writer),
      }
    }
    BinaryOp::Mul => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left * right),
        _ => utils::panic("Expected int".to_string(), writer),
      }
    }
    BinaryOp::Div => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left / right),
        _ => utils::panic("Expected int".to_string(), writer),
      }
    }
    BinaryOp::Rem => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left % right),
        _ => utils::panic("Expected int".to_string(), writer),
      }
    }
    BinaryOp::Eq => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left == right),
        (Value::Str(left), Value::Str(right)) => Value::Bool(left == right),
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left == right),
        _ => utils::panic(
          "Expected int, string or bool. Types should match.".to_string(),
          writer,
        ),
      }
    }
    BinaryOp::Neq => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left != right),
        (Value::Str(left), Value::Str(right)) => Value::Bool(left != right),
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left != right),
        _ => utils::panic(
          "Expected int, string or bool. Types should match.".to_string(),
          writer,
        ),
      }
    }
    BinaryOp::Lt => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left < right),
        _ => utils::panic("Expected int".to_string(), writer),
      }
    }
    BinaryOp::Gt => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left > right),
        _ => utils::panic("Expected int".to_string(), writer),
      }
    }
    BinaryOp::Lte => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left <= right),
        _ => utils::panic("Expected int".to_string(), writer),
      }
    }
    BinaryOp::Gte => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left >= right),
        _ => utils::panic("Expected int".to_string(), writer),
      }
    }
    BinaryOp::And => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left && right),
        _ => utils::panic("Expected bool".to_string(), writer),
      }
    }
    BinaryOp::Or => {
      let left = eval(*binary.lhs, env, writer);
      let right = eval(*binary.rhs, env, writer);

      match (left, right) {
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left || right),
        _ => utils::panic("Expected bool".to_string(), writer),
      }
    }
  }
}
