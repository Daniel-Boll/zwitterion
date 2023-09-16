use std::{borrow::BorrowMut, io::Write};

use rinha::ast::{Binary, BinaryOp};

use super::{utils, Interpreter, Value};

impl<W: Write> Interpreter<W> {
  pub(crate) fn eval_binary(&mut self, binary: Binary) -> Value {
    match binary.op {
      BinaryOp::Add => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Int(left + right),
          (Value::Int(left), Value::Str(right)) => Value::Str(format!("{}{}", left, right)),
          (Value::Str(left), Value::Int(right)) => Value::Str(format!("{}{}", left, right)),
          (Value::Str(left), Value::Str(right)) => Value::Str(format!("{}{}", left, right)),
          _ => utils::panic(
            "Expected int or string".to_string(),
            self.writer.borrow_mut(),
          ),
        }
      }
      BinaryOp::Sub => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Int(left - right),
          _ => utils::panic("Expected int".to_string(), self.writer.borrow_mut()),
        }
      }
      BinaryOp::Mul => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Int(left * right),
          _ => utils::panic("Expected int".to_string(), self.writer.borrow_mut()),
        }
      }
      BinaryOp::Div => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Int(left / right),
          _ => utils::panic("Expected int".to_string(), self.writer.borrow_mut()),
        }
      }
      BinaryOp::Rem => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Int(left % right),
          _ => utils::panic("Expected int".to_string(), self.writer.borrow_mut()),
        }
      }
      BinaryOp::Eq => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Bool(left == right),
          (Value::Str(left), Value::Str(right)) => Value::Bool(left == right),
          (Value::Bool(left), Value::Bool(right)) => Value::Bool(left == right),
          _ => utils::panic(
            "Expected int, string or bool. Types should match.".to_string(),
            self.writer.borrow_mut(),
          ),
        }
      }
      BinaryOp::Neq => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Bool(left != right),
          (Value::Str(left), Value::Str(right)) => Value::Bool(left != right),
          (Value::Bool(left), Value::Bool(right)) => Value::Bool(left != right),
          _ => utils::panic(
            "Expected int, string or bool. Types should match.".to_string(),
            self.writer.borrow_mut(),
          ),
        }
      }
      BinaryOp::Lt => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Bool(left < right),
          _ => utils::panic("Expected int".to_string(), self.writer.borrow_mut()),
        }
      }
      BinaryOp::Gt => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Bool(left > right),
          _ => utils::panic("Expected int".to_string(), self.writer.borrow_mut()),
        }
      }
      BinaryOp::Lte => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Bool(left <= right),
          _ => utils::panic("Expected int".to_string(), self.writer.borrow_mut()),
        }
      }
      BinaryOp::Gte => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Int(left), Value::Int(right)) => Value::Bool(left >= right),
          _ => utils::panic("Expected int".to_string(), self.writer.borrow_mut()),
        }
      }
      BinaryOp::And => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Bool(left), Value::Bool(right)) => Value::Bool(left && right),
          _ => utils::panic("Expected bool".to_string(), self.writer.borrow_mut()),
        }
      }
      BinaryOp::Or => {
        let left = self.eval(*binary.lhs);
        let right = self.eval(*binary.rhs);

        match (left, right) {
          (Value::Bool(left), Value::Bool(right)) => Value::Bool(left || right),
          _ => utils::panic("Expected bool".to_string(), self.writer.borrow_mut()),
        }
      }
    }
  }
}
