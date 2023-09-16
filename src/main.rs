use rinha::rinha;
use zwitterion::interpreter;

fn main() {
  let input = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
  let ast = rinha::FileParser::new()
    .parse(&mut vec![], "", &input)
    .unwrap();

  interpreter::from_ast(ast.expression, &mut std::io::stdout());
}
