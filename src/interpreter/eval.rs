use rinha::ast::Term;
use std::io::Write;

use super::{binary::eval_binary, env, utils, AnonymousFunction, Value};

pub fn eval<W: Write>(term: Term, env: &mut env::Env, writer: &mut W) -> Value {
  match term {
    Term::Error(error) => {
      utils::panic(format!("Error: {:#?}", error), writer);
    }
    Term::Int(int_term) => Value::Int(int_term.value.into()),
    Term::Str(str_term) => Value::Str(str_term.value),
    Term::Let(term) => {
      let value = eval(*term.value.clone(), env, writer);

      env.set(&term.name.text, value.clone());

      eval(*term.next, env, writer)
    }
    Term::Call(call_term) => {
      let closure = eval(*call_term.callee, env, writer);

      match closure {
        Value::Closure(function) => {
          let mut function_env = function.env.clone();

          if function.parameters.len() != call_term.arguments.len() {
            utils::panic(
              format!(
                "Expected {} arguments, got {}",
                function.parameters.len(),
                call_term.arguments.len()
              ),
              writer,
            );
          }

          for (parameter, argument) in function.parameters.iter().zip(call_term.arguments) {
            let value = eval(argument, env, writer);

            function_env.set(&parameter.text, value);
          }

          eval(*function.body, &mut function_env, writer)
        }
        _ => utils::panic("Expected closure".to_string(), writer),
      }
    }
    Term::Binary(binary_term) => eval_binary(binary_term, env, writer),
    Term::Function(function_term) => Value::Closure(AnonymousFunction {
      parameters: function_term.parameters,
      body: function_term.value,
      env: env.clone(),
    }),
    Term::If(if_term) => {
      let condition = eval(*if_term.condition, env, writer);

      match condition {
        Value::Bool(true) => eval(*if_term.then, env, writer),
        Value::Bool(false) => eval(*if_term.otherwise, env, writer),
        _ => utils::panic("Expected bool".to_string(), writer),
      }
    }
    Term::Print(print_term) => {
      let value = eval(*print_term.value, env, writer);
      writer.write_all(format!("{}\n", value).as_bytes()).unwrap();
      value
    }
    Term::First(first_term) => {
      let tuple = eval(*first_term.value, env, writer);

      match tuple {
        Value::Tuple(first, _) => *first,
        _ => utils::panic("Expected tuple".to_string(), writer),
      }
    }
    Term::Second(second_term) => {
      let tuple = eval(*second_term.value, env, writer);

      match tuple {
        Value::Tuple(_, second) => *second,
        _ => utils::panic("Expected tuple".to_string(), writer),
      }
    }
    Term::Bool(bool_term) => Value::Bool(bool_term.value),
    Term::Tuple(tuple_term) => {
      let first = eval(*tuple_term.first, env, writer);
      let second = eval(*tuple_term.second, env, writer);

      Value::Tuple(Box::new(first), Box::new(second))
    }
    Term::Var(var_term) => {
      let value = env
        .get(&var_term.text)
        .unwrap_or_else(|| utils::panic(format!("Variable {} not found", var_term.text), writer));

      value.clone()
    }
  }
}
