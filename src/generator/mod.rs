use crate::parser::ASTExpression;

use std::collections::LinkedList;

const GENERATED_HEADER: &str = "// This is automatically generated code from yasld v 0.0.1
// Please check the project https://github.com/zkud/yasld
function* range(start, end) {
  while (start < end) {
    yield start;
    start += 1;
  }
}
";

pub fn generate_code(statements: &LinkedList<ASTExpression>) -> String {
  let mut generated_code: String = GENERATED_HEADER.to_owned();

  for statement in statements {
    generated_code = generated_code + &generate_code_for_statement(statement);
  }

  generated_code
}

fn generate_code_for_statement(statement: &ASTExpression) -> String {
  if let Some(code) = generate_code_for_ternary_function(statement) {
    return code;
  }

  if let Some(code) = generate_code_for_var_operation(statement) {
    return code;
  }

  if let Some(code) = generate_code_for_binary_function(statement) {
    return code;
  }

  if let Some(code) = generate_code_for_unary_function(statement) {
    return code;
  }

  if let Some(code) = generate_code_for_literal(statement) {
    return code;
  }

  String::from("")
}

#[inline]
fn generate_code_for_ternary_function(statement: &ASTExpression) -> Option<String> {
  match statement {
    ASTExpression::ForDefinition { index, range, body } => Some(format!(
      "for ({} of {}) {{\n  {}}}\n",
      index,
      generate_code_for_statement(&range),
      generate_code_for_statement(&body),
    )),
    ASTExpression::IFDefinition {
      condition,
      then_clause,
      else_clause,
    } => Some(format!(
      "if ({}) {{\n  {}}} else {{\n  {}}}\n",
      generate_code_for_statement(&condition),
      generate_code_for_statement(&then_clause),
      generate_code_for_statement(&else_clause),
    )),
    _ => None,
  }
}

#[inline]
fn generate_code_for_var_operation(statement: &ASTExpression) -> Option<String> {
  match statement {
    ASTExpression::SetDefinition { name, new_value } => Some(format!(
      "{} = {};\n",
      name,
      generate_code_for_statement(&new_value),
    )),
    ASTExpression::VarDefinition { name, value } => Some(format!(
      "let {} = {};\n",
      name,
      generate_code_for_statement(&value),
    )),
    _ => None,
  }
}

#[inline]
fn generate_code_for_binary_function(statement: &ASTExpression) -> Option<String> {
  match statement {
    ASTExpression::RangeDefinition { start, end } => Some(format!(
      "range({}, {})",
      generate_code_for_statement(&start),
      generate_code_for_statement(&end),
    )),
    ASTExpression::EqExpression { left, right } => Some(format!(
      "({} == {})",
      generate_code_for_statement(&left),
      generate_code_for_statement(&right),
    )),
    ASTExpression::BiggerExpression { left, right } => Some(format!(
      "({} > {})",
      generate_code_for_statement(&left),
      generate_code_for_statement(&right),
    )),
    ASTExpression::LessExpression { left, right } => Some(format!(
      "({} < {})",
      generate_code_for_statement(&left),
      generate_code_for_statement(&right),
    )),
    ASTExpression::PlusExpression { left, right } => Some(format!(
      "({} + {})",
      generate_code_for_statement(&left),
      generate_code_for_statement(&right),
    )),
    ASTExpression::MinusExpression { left, right } => Some(format!(
      "({} - {})",
      generate_code_for_statement(&left),
      generate_code_for_statement(&right),
    )),
    ASTExpression::MultiplyExpression { left, right } => Some(format!(
      "({} * {})",
      generate_code_for_statement(&left),
      generate_code_for_statement(&right),
    )),
    ASTExpression::DivideExpression { left, right } => Some(format!(
      "({} / {})",
      generate_code_for_statement(&left),
      generate_code_for_statement(&right),
    )),
    ASTExpression::BinaryAndExpression { left, right } => Some(format!(
      "({} && {})",
      generate_code_for_statement(&left),
      generate_code_for_statement(&right),
    )),
    ASTExpression::BinaryOrExpression { left, right } => Some(format!(
      "({} || {})",
      generate_code_for_statement(&left),
      generate_code_for_statement(&right),
    )),
    _ => None,
  }
}

#[inline]
fn generate_code_for_unary_function(statement: &ASTExpression) -> Option<String> {
  match statement {
    ASTExpression::BinaryNegExpression(expression) => {
      Some(format!("(!{})", generate_code_for_statement(expression)))
    }
    ASTExpression::ReportDefinition(expression) => {
      Some(format!("console.log({});", generate_code_for_statement(expression)))
    }
    _ => None,
  }
}

#[inline]
fn generate_code_for_literal(statement: &ASTExpression) -> Option<String> {
  match statement {
    ASTExpression::NameReference(name) => Some(name.clone()),
    ASTExpression::NumberLiteral(value) => Some(value.clone()),
    ASTExpression::BoolLiteral(value) => {
      let bool_string = if *value {
        "true".to_owned()
      } else {
        "false".to_owned()
      };
      Some(bool_string)
    }
    _ => None,
  }
}
