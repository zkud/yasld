use regex;
use std::collections::LinkedList;

#[derive(Debug, Clone)]
pub enum Token {
  OpenBracket,
  CloseBracket,
  Name(String),
  Range,
  Var,
  Number(String),
  Set,
  Bool(bool),
  If,
  For,
  Plus,
  Minus,
  Multiply,
  Divide,
  Eq,
  Bigger,
  Less,
  BinaryOr,
  BinaryAnd,
  BinaryNeg,
  Space,
}

pub fn parse(program: &str) -> LinkedList<Token> {
  let mut tokens: LinkedList<Token> = LinkedList::new();

  let mut cursor: usize = 0;
  while cursor < program.len() {
    let (new_token, new_cursor) = extract_token(program, cursor);

    tokens.push_back(new_token);
    cursor = new_cursor;
  }

  tokens
}

#[inline]
fn extract_token(program: &str, cursor: usize) -> (Token, usize) {
  match &program[cursor..] {
    open_bracket if open_bracket.starts_with("(") => (Token::OpenBracket, cursor + 1),
    close_bracket if close_bracket.starts_with(")") => (Token::CloseBracket, cursor + 1),
    space if space.starts_with(" ") || space.starts_with("\n") || space.starts_with("\t") => {
      (Token::Space, cursor + 1)
    }
    if_def if if_matches(if_def) => (Token::If, cursor + 2),
    var_def if var_matches(var_def) => (Token::Var, cursor + 3),
    set_def if set_matches(set_def) => (Token::Set, cursor + 3),
    range_def if range_matches(range_def) => (Token::Range, cursor + 5),
    number if number_matches(number) => {
      let number: String = get_number_substr(number);
      let number_lenght = number.len();
      (Token::Number(number), cursor + number_lenght)
    }
    true_literal if true_matches(true_literal) => (Token::Bool(true), cursor + 4),
    false_literal if false_matches(false_literal) => (Token::Bool(false), cursor + 5),
    for_def if for_matches(for_def) => (Token::For, cursor + 3),
    plus if plus.starts_with("+") => (Token::Plus, cursor + 1),
    minus if minus.starts_with("+") => (Token::Minus, cursor + 1),
    divide if divide.starts_with("+") => (Token::Divide, cursor + 1),
    multiply if multiply.starts_with("+") => (Token::Multiply, cursor + 1),
    binary_and if binary_and.starts_with("&&") => (Token::BinaryAnd, cursor + 2),
    binary_or if binary_or.starts_with("||") => (Token::BinaryOr, cursor + 2),
    binary_neg if binary_neg.starts_with("!") => (Token::BinaryNeg, cursor + 1),
    eq if eq.starts_with("==") => (Token::Eq, cursor + 2),
    bigger if bigger.starts_with(">") => (Token::Bigger, cursor + 1),
    less if less.starts_with("<") => (Token::Less, cursor + 1),
    name if name_matches(name) => {
      let name: String = get_name_substr(name);
      let name_lenght = name.len();
      (Token::Name(name), cursor + name_lenght)
    }
    _ => panic!("Unexpected token"),
  }
}

#[inline]
fn if_matches(s: &str) -> bool {
  s.starts_with("if ") || s.starts_with("if\n") || s.starts_with("if\t")
}

#[inline]
fn var_matches(s: &str) -> bool {
  s.starts_with("var ") || s.starts_with("var\n") || s.starts_with("var\t")
}

#[inline]
fn set_matches(s: &str) -> bool {
  s.starts_with("set ") || s.starts_with("set\n") || s.starts_with("set\t")
}

#[inline]
fn range_matches(s: &str) -> bool {
  s.starts_with("range ") || s.starts_with("range\n") || s.starts_with("range\t")
}

#[inline]
fn number_matches(s: &str) -> bool {
  let number_regex = regex::Regex::new(r"^[-+]?[0-9]*.?[0-9]+([eE][-+]?[0-9]+)?[.]*").unwrap();
  number_regex.is_match(s)
}

#[inline]
fn get_number_substr(s: &str) -> String {
  let number_regex = regex::Regex::new(r"^[-+]?[0-9]*.?[0-9]+([eE][-+]?[0-9]+)?[.]*").unwrap();
  let positions = number_regex.find(s).unwrap();
  String::from(&s[positions.start()..positions.end()])
}

#[inline]
fn true_matches(s: &str) -> bool {
  s.starts_with("true)")
    || s.starts_with("true ")
    || s.starts_with("true\n")
    || s.starts_with("true\t")
    || s.starts_with("true||")
    || s.starts_with("true&&")
}

#[inline]
fn false_matches(s: &str) -> bool {
  s.starts_with("false)")
    || s.starts_with("false ")
    || s.starts_with("false\n")
    || s.starts_with("false\t")
    || s.starts_with("false||")
    || s.starts_with("false&&")
}

#[inline]
fn for_matches(s: &str) -> bool {
  s.starts_with("for ") || s.starts_with("for\n") || s.starts_with("for\t")
}

#[inline]
fn name_matches(s: &str) -> bool {
  let name_regex = regex::Regex::new(r"^[\w][\w0-9_]*").unwrap();
  name_regex.is_match(s)
}

#[inline]
fn get_name_substr(s: &str) -> String {
  let name_regex = regex::Regex::new(r"^[\w][\w0-9_]*").unwrap();
  let positions = name_regex.find(s).unwrap();
  String::from(&s[positions.start()..positions.end()])
}
