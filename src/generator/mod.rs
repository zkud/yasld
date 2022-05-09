use crate::parser::ASTExpression;

use std::collections::LinkedList;

const GENERATED_HEADER: &str = "
// This is automatically generated code from yasld v 0.0.1
// Please check the project https://github.com/zkud/yasld
function* range(start, end) {
    while (start < end) {
        yield start;
        start += 1;
    }
}";

pub fn generate_code(statements: &LinkedList<ASTExpression>) -> String {
  GENERATED_HEADER.to_owned()
}
