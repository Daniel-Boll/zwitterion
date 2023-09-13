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
fn tuple_second_fail() {
  insta::assert_yaml_snapshot!(interpret_to_buffer(r#"print(second(true))"#));
}
