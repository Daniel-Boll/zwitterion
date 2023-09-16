use std::collections::HashMap;

use super::Value;

#[derive(Debug, Clone)]
pub struct Env {
  pub scopes: Vec<HashMap<String, Value>>,
}

impl Env {
  /// Create a new environment with a single scope.
  /// This scope is the global scope.
  pub fn new() -> Self {
    Self {
      scopes: vec![HashMap::new()],
    }
  }

  /// Get the value of a variable from the environment.
  /// This function will search the environment from the most recent scope to the least recent scope.
  /// If the variable is not found, None is returned.
  pub fn get(&self, name: &str) -> Option<&Value> {
    for scope in self.scopes.iter().rev() {
      if let Some(value) = scope.get(name) {
        return Some(value);
      }
    }

    None
  }

  /// Set the value of a variable in the environment at the most recent scope.
  pub fn set(&mut self, name: &str, value: Value) {
    if let Some(scope) = self.scopes.last_mut() {
      scope.insert(name.to_string(), value);
    }
  }

  /// Create a new scope in the environment.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use rinha::interpreter::Env;
  ///
  /// let mut env = Env::new();
  ///
  /// env.push_scope();
  /// ```
  pub fn push_scope(&mut self) {
    self.scopes.push(HashMap::new());
  }

  /// Remove the most recent scope from the environment.
  ///
  /// # Panics
  ///
  /// This function will panic if there is only one scope in the environment.
  ///
  /// # Examples
  ///
  /// Basic usage:
  ///
  /// ```
  /// use rinha::interpreter::Env;
  ///
  /// let mut env = Env::new();
  ///
  /// env.push_scope();
  /// env.pop_scope(); // Works fine
  /// ```
  ///
  /// This will panic:
  ///
  /// ```should_panic
  /// use rinha::interpreter::Env;
  ///
  /// let mut env = Env::new();
  ///
  /// env.pop_scope(); // This will panic on trying to pop the global scope
  /// ```
  pub fn pop_scope(&mut self) {
    if self.scopes.len() == 1 {
      panic!("Cannot pop global scope");
    }

    self.scopes.pop();
  }
}

impl Default for Env {
  fn default() -> Self {
    Self::new()
  }
}

impl std::fmt::Display for Env {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (i, scope) in self.scopes.iter().enumerate() {
      writeln!(f, "Scope {}:", i)?;

      for (name, value) in scope.iter() {
        writeln!(f, "  {} = {}", name, value)?;
      }
    }

    Ok(())
  }
}
