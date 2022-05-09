mod lexer;
mod parser;

use std::env;
use std::fs;

fn main() {
  let args: Vec<String> = env::args().collect();

  let filename = &args[1];
  println!("checking {}...", filename);

  let content = fs::read_to_string(filename).expect("cannot read file");
  println!("the content is:\n{}", content);

  println!("Parsed tokens:");
  let parsed_tokens = lexer::parse(&content);
  for token in &parsed_tokens {
    println!("{:?}", token);
  }

  let ast_stmns = parser::parse(&parsed_tokens);
}
