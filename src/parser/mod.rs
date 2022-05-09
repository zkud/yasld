mod ast;
mod expression_iter;

use crate::lexer::Token;
pub use ast::ASTExpression;
use std::collections::LinkedList;

pub fn parse(tokens: &LinkedList<Token>) -> LinkedList<ASTExpression> {
  let parser = expression_iter::ExpressionIter::new(tokens);
  parser.collect()
}
