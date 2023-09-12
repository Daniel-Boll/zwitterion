use rinha::rinha;
use rinha_de_compiler::interpreter;

fn main() {
  let mut errors = vec![];
  let input = std::fs::read_to_string("assets/test.rinha").unwrap();
  let ast = rinha::FileParser::new()
    .parse(&mut errors, "assets/test.rinha", &input)
    .unwrap();

  interpreter::from_ast(ast.expression, &mut std::io::stdout());
}
