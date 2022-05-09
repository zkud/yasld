use super::ASTExpression;
use crate::lexer::Token;
use std::collections::HashMap;
use std::collections::LinkedList;

#[derive(Clone, PartialEq)]
pub enum VariableType {
  Range,
  Number,
  Bool,
}

pub struct ExpressionIter {
  declared_variables: HashMap<String, VariableType>,
  cursor: usize,
  tokens: Vec<Token>,
  operations_required_type: LinkedList<VariableType>,
}

impl ExpressionIter {
  pub fn new(raw_tokens: &LinkedList<Token>) -> Self {
    Self {
      declared_variables: HashMap::new(),
      tokens: ExpressionIter::strip_tokens(raw_tokens),
      cursor: 0,
      operations_required_type: LinkedList::new(),
    }
  }

  #[inline]
  fn strip_tokens(raw_tokens: &LinkedList<Token>) -> Vec<Token> {
    raw_tokens
      .iter()
      .filter(|token| !matches!(token, Token::Space))
      .map(|token| token.to_owned())
      .collect()
  }

  /*
    Reqursive traverse among the tokens stream
  */
  fn pop_expression(&mut self) -> ASTExpression {
    if let Some(expression) = self.pop_binary_function() {
      return expression;
    }

    if let Some(expression) = self.pop_unary_function() {
      return expression;
    }

    if let Some(expression) = self.pop_literal() {
      return expression;
    }

    panic!("Incorrect syntax");
  }

  #[inline]
  fn pop_binary_function(&mut self) -> Option<ASTExpression> {
    let (left, right) = match &self.tokens[self.cursor] {
      Token::Bigger | Token::Less | Token::Eq => {
        self.reduce_binary_function(VariableType::Number, VariableType::Bool)
      }
      Token::Plus | Token::Minus | Token::Divide | Token::Multiply => {
        self.reduce_binary_function(VariableType::Number, VariableType::Number)
      }
      Token::BinaryAnd | Token::BinaryOr => {
        self.reduce_binary_function(VariableType::Bool, VariableType::Bool)
      }
      Token::Name(name) if name == "range" => {
        self.reduce_binary_function(VariableType::Number, VariableType::Range)
      }
      _ => return None,
    };

    Some(match &self.tokens[self.cursor] {
      Token::Bigger => ASTExpression::BiggerExpression { left, right },
      Token::Less => ASTExpression::LessExpression { left, right },
      Token::Eq => ASTExpression::EqExpression { left, right },
      Token::Divide => ASTExpression::DivideExpression { left, right },
      Token::Multiply => ASTExpression::MultiplyExpression { left, right },
      Token::Minus => ASTExpression::MinusExpression { left, right },
      Token::Plus => ASTExpression::PlusExpression { left, right },
      Token::BinaryAnd => ASTExpression::BinaryAndExpression { left, right },
      Token::BinaryOr => ASTExpression::BinaryOrExpression { left, right },
      Token::Name(name) if name == "range" => ASTExpression::RangeDefinition {
        start: left,
        end: right,
      },
      _ => return None,
    })
  }

  #[inline]
  fn reduce_binary_function(
    &mut self,
    required_args_type: VariableType,
    function_type: VariableType,
  ) -> (Box<ASTExpression>, Box<ASTExpression>) {
    self.cursor += 1;

    if !self.operations_required_type.is_empty() {
      assert_eq!(self.operations_required_type.back(), function_type);
    }

    self.operations_required_type.push_back(required_args_type);
    let left = self.pop_expression();
    let right = self.pop_expression();
    self.operations_required_type.pop_back();

    (Box::new(left), Box::new(right))
  }

  #[inline]
  fn pop_unary_function(&mut self) -> Option<ASTExpression> {
    match &self.tokens[self.cursor] {
      Token::OpenBracket => {
        self.cursor += 1;

        let result = self.pop_expression();

        assert!(matches!(self.tokens[self.cursor], Token::CloseBracket));
        self.cursor += 1;

        Some(result)
      }
      Token::BinaryNeg => {
        self.cursor += 1;

        assert_eq!(self.operations_required_type.back(), VariableType::Bool);

        self.operations_required_type.push_back(VariableType::Bool);
        let result = Some(ASTExpression::BinaryNegExpression(Box::new(
          self.pop_expression(),
        )));
        self.operations_required_type.pop_back();

        result
      }
      _ => None,
    }
  }

  #[inline]
  fn pop_literal(&mut self) -> Option<ASTExpression> {
    match &self.tokens[self.cursor] {
      Token::Number(value) => {
        self.cursor += 1;
        Some(ASTExpression::NumberLiteral(value.clone()))
      }
      Token::Bool(value) => {
        self.cursor += 1;
        Some(ASTExpression::BoolLiteral(value.clone()))
      }
      Token::Name(name) if self.is_appropriate_variable(name) => {
        self.cursor += 1;
        Some(ASTExpression::NameReference(name.clone()))
      }
      _ => None,
    }
  }

  #[inline]
  fn is_appropriate_variable(&self, name: &str) -> bool {
    if let Some(variable_type) = self.declared_variables.get(name) {
      if let Some(required_type) = self.operations_required_type.back() {
        return required_type == variable_type;
      } else {
        return true;
      }
    }

    false
  }
}

impl Iterator for ExpressionIter {
  type Item = ASTExpression;

  fn next(&mut self) -> Option<Self::Item> {
    if self.cursor < self.tokens.len() {
      Some(self.pop_expression())
    } else {
      None
    }
  }
}
