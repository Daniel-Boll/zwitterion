use std::collections::HashMap;

use super::Value;

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

  pub fn from_env(env: &Env) -> Self {
    Self {
      values: env.values.clone(),
    }
  }
}

impl Default for Env {
  fn default() -> Self {
    Self::new()
  }
}
