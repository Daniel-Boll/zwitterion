use std::collections::HashMap;

use rinha::{
  ast::{Binary, BinaryOp, Term},
  parser::Var,
};

#[derive(Debug, Clone)]
pub enum Value {
  Int(i64),
  Str(String),
  Bool(bool),
  Tuple(Box<Value>, Box<Value>),
  Closure(AnonymousFunction),
}

#[derive(Debug, Clone)]
pub struct AnonymousFunction {
  pub parameters: Vec<Var>,
  pub body: Box<Term>,
  pub env: Env,
}

#[derive(Debug, Clone)]
pub struct Env {
  pub values: HashMap<String, Value>,
}

impl Env {
  pub fn new() -> Self {
    Self {
      values: HashMap::new(),
    }
  }

  pub fn get(&self, name: &str) -> Option<&Value> {
    self.values.get(name)
  }

  pub fn set(&mut self, name: &str, value: Value) {
    self.values.insert(name.to_string(), value);
  }
}

impl Default for Env {
  fn default() -> Self {
    Self::new()
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

pub fn interpret_from_ast(term: Term) {
  let mut env = Env::default();

  interpret(term, &mut env);
}

pub fn interpret(term: Term, env: &mut Env) -> Value {
  match term {
    Term::Error(error) => {
      panic!("Error: {:#?}", error);
    }
    Term::Int(int_term) => Value::Int(int_term.value.into()),
    Term::Str(str_term) => Value::Str(str_term.value),
    Term::Let(term) => {
      let value = interpret(*term.value.clone(), env);

      env.set(&term.name.text, value.clone());

      interpret(*term.next, env)
    }
    Term::Call(call_term) => {
      let closure = interpret(*call_term.callee, env);

      match closure {
        Value::Closure(function) => {
          let mut function_env = function.env.clone();

          if function.parameters.len() != call_term.arguments.len() {
            panic!(
              "Expected {} arguments, got {}",
              function.parameters.len(),
              call_term.arguments.len()
            );
          }

          for (parameter, argument) in function.parameters.iter().zip(call_term.arguments) {
            let value = interpret(argument, env);

            function_env.set(&parameter.text, value);
          }

          interpret(*function.body, &mut function_env)
        }
        _ => panic!("Expected a closure"),
      }
    }
    Term::Binary(binary_term) => interpret_binary(binary_term, env),
    Term::Function(function_term) => Value::Closure(AnonymousFunction {
      parameters: function_term.parameters,
      body: function_term.value,
      env: env.clone(),
    }),
    Term::If(if_term) => {
      let condition = interpret(*if_term.condition, env);

      match condition {
        Value::Bool(true) => interpret(*if_term.then, env),
        Value::Bool(false) => interpret(*if_term.otherwise, env),
        _ => panic!("Expected bool"),
      }
    }
    Term::Print(print_term) => {
      let value = interpret(*print_term.value, env);
      println!("{}", value);
      value
    }
    Term::First(first_term) => {
      let tuple = interpret(*first_term.value, env);

      match tuple {
        Value::Tuple(first, _) => *first,
        _ => panic!("Expected tuple"),
      }
    }
    Term::Second(second_term) => {
      let tuple = interpret(*second_term.value, env);

      match tuple {
        Value::Tuple(_, second) => *second,
        _ => panic!("Expected tuple"),
      }
    }
    Term::Bool(bool_term) => Value::Bool(bool_term.value),
    Term::Tuple(tuple_term) => {
      let first = interpret(*tuple_term.first, env);
      let second = interpret(*tuple_term.second, env);

      Value::Tuple(Box::new(first), Box::new(second))
    }
    Term::Var(var_term) => {
      let value = env
        .get(&var_term.text)
        .unwrap_or_else(|| panic!("Variable {} not found", var_term.text));

      value.clone()
    }
  }
}

fn interpret_binary(binary: Binary, env: &mut Env) -> Value {
  match binary.op {
    BinaryOp::Add => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left + right),
        (Value::Int(left), Value::Str(right)) => Value::Str(format!("{}{}", left, right)),
        (Value::Str(left), Value::Int(right)) => Value::Str(format!("{}{}", left, right)),
        (Value::Str(left), Value::Str(right)) => Value::Str(format!("{}{}", left, right)),
        _ => panic!("Expected int or string"),
      }
    }
    BinaryOp::Sub => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left - right),
        _ => panic!("Expected int"),
      }
    }
    BinaryOp::Mul => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left * right),
        _ => panic!("Expected int"),
      }
    }
    BinaryOp::Div => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left / right),
        _ => panic!("Expected int"),
      }
    }
    BinaryOp::Rem => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Int(left % right),
        _ => panic!("Expected int"),
      }
    }
    BinaryOp::Eq => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left == right),
        (Value::Str(left), Value::Str(right)) => Value::Bool(left == right),
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left == right),
        _ => panic!("Expected int, string or bool. Types should match."),
      }
    }
    BinaryOp::Neq => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left != right),
        (Value::Str(left), Value::Str(right)) => Value::Bool(left != right),
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left != right),
        _ => panic!("Expected int, string or bool. Types should match."),
      }
    }
    BinaryOp::Lt => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left < right),
        _ => panic!("Expected int"),
      }
    }
    BinaryOp::Gt => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left > right),
        _ => panic!("Expected int"),
      }
    }
    BinaryOp::Lte => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left <= right),
        _ => panic!("Expected int"),
      }
    }
    BinaryOp::Gte => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Int(left), Value::Int(right)) => Value::Bool(left >= right),
        _ => panic!("Expected int"),
      }
    }
    BinaryOp::And => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left && right),
        _ => panic!("Expected bool"),
      }
    }
    BinaryOp::Or => {
      let left = interpret(*binary.lhs, env);
      let right = interpret(*binary.rhs, env);

      match (left, right) {
        (Value::Bool(left), Value::Bool(right)) => Value::Bool(left || right),
        _ => panic!("Expected bool"),
      }
    }
  }
}
