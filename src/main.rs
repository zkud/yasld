mod generator;
mod lexer;
mod parser;

use std::env;
use std::fs;
use std::io::prelude::*;

fn main() {
  let args: Vec<String> = env::args().collect();

  let filename = &args[1];
  println!("Reading {}...", filename);
  let content = fs::read_to_string(filename).expect("cannot read file");

  println!("Parsing {}...", filename);
  let parsed_tokens = lexer::parse(&content);
  let ast_stmns = parser::parse(&parsed_tokens);

  println!("Generating code for {}...", filename);
  let code = generator::generate_code(&ast_stmns);

  let mut code_file = fs::File::create(format!("{}.js", filename)).unwrap();
  code_file.write_all(code.as_bytes()).unwrap();
}
