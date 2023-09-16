use zwitterion::utils::interpret_to_buffer;

#[test]
fn print_number() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(r#"print(1)"#));
}

#[test]
fn print_string() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(r#"print("Hello, world")"#));
}

#[test]
fn define_variable() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(
    r#"
  let a = 1;
  print(a)
  "#
  ));
}

#[test]
fn print_twice() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(
    r#"
  let a = 1;
  print(print(a))
  "#
  ));
}

#[test]
fn sum() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(
    r#"
  let sum = fn(a, b) => { a + b };
  print(sum(1, 2))
  "#
  ));

  insta::assert_yaml_snapshot!(interpret_to_buffer(
    r#"
  print((fn(a, b) => { a + b })(1, 2))
  "#
  ));
}

#[test]
fn declare_tuple() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(
    r#"
  let tuple = (1, 2);
  print(tuple)
  "#
  ));
}

#[test]
fn tuple_first() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(
    r#"
  let tuple = (1, 2);
  print(first(tuple))
  "#
  ));
}

#[test]
fn tuple_second() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(
    r#"
  let tuple = (1, 2);
  print(second(tuple))
  "#
  ));
}

#[test]
fn tuple_first_fail() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(r#"print(first(true))"#));
}

#[test]
fn tuple_second_fail() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(r#"print(second(true))"#));
}
