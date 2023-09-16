use rinha::ast::Term;
use std::{borrow::BorrowMut, io::Write};

use super::{utils, AnonymousFunction, Interpreter, Value};

impl<W: Write> Interpreter<W> {
  pub fn eval(&mut self, term: Term) -> Value {
    match term {
      Term::Error(error) => {
        utils::panic(format!("Error: {:#?}", error), self.writer.borrow_mut());
      }
      Term::Int(int_term) => Value::Int(int_term.value.into()),
      Term::Str(str_term) => Value::Str(str_term.value),
      Term::Let(term) => {
        let value = self.eval(*term.value.clone());

        self.env.set(&term.name.text, value.clone());

        self.eval(*term.next)
      }
      Term::Call(call_term) => {
        let closure = self.eval(*call_term.callee);

        match closure {
          Value::Closure(function) => {
            if function.parameters.len() != call_term.arguments.len() {
              utils::panic(
                format!(
                  "Expected {} arguments, got {}",
                  function.parameters.len(),
                  call_term.arguments.len()
                ),
                self.writer.borrow_mut(),
              );
            }

            self.env.push_scope();

            for (parameter, argument) in function.parameters.iter().zip(call_term.arguments) {
              let value = self.eval(argument);

              self.env.set(&parameter.text, value);
            }

            let ret = self.eval(*function.body);

            self.env.pop_scope();

            ret
          }
          _ => utils::panic("Expected closure".to_string(), self.writer.borrow_mut()),
        }
      }
      Term::Binary(binary_term) => self.eval_binary(binary_term),
      Term::Function(function_term) => Value::Closure(AnonymousFunction {
        parameters: function_term.parameters,
        body: function_term.value,
      }),
      Term::If(if_term) => {
        let condition = self.eval(*if_term.condition);

        match condition {
          Value::Bool(true) => self.eval(*if_term.then),
          Value::Bool(false) => self.eval(*if_term.otherwise),
          _ => utils::panic("Expected bool".to_string(), self.writer.borrow_mut()),
        }
      }
      Term::Print(print_term) => {
        let value = self.eval(*print_term.value);
        self
          .writer
          .write_all(format!("{}\n", value).as_bytes())
          .unwrap();
        value
      }
      Term::First(first_term) => {
        let tuple = self.eval(*first_term.value);

        match tuple {
          Value::Tuple(first, _) => *first,
          _ => utils::panic("Expected tuple".to_string(), self.writer.borrow_mut()),
        }
      }
      Term::Second(second_term) => {
        let tuple = self.eval(*second_term.value);

        match tuple {
          Value::Tuple(_, second) => *second,
          _ => utils::panic("Expected tuple".to_string(), self.writer.borrow_mut()),
        }
      }
      Term::Bool(bool_term) => Value::Bool(bool_term.value),
      Term::Tuple(tuple_term) => {
        let first = self.eval(*tuple_term.first);
        let second = self.eval(*tuple_term.second);

        Value::Tuple(Box::new(first), Box::new(second))
      }
      Term::Var(var_term) => {
        let value = self.env.get(&var_term.text).unwrap_or_else(|| {
          utils::panic(
            format!("Variable {} not found\n{}", var_term.text, self.env),
            self.writer.borrow_mut(),
          )
        });

        value.clone()
      }
    }
  }
}
