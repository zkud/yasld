mod lexer;
mod parser;
mod generator;

use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();

  let filename = &args[1];
  println!("reading {}...", filename);
  let content = fs::read_to_string(filename).expect("cannot read file");

  println!("Parsing {}...", filename);
  let parsed_tokens = lexer::parse(&content);
  let ast_stmns = parser::parse(&parsed_tokens);

  println!("Generating code for {}...", filename);
  let code = generator::generate_code(&ast_stmns);

  println!("Code is:\n{}", code);
}
