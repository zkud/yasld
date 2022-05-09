use super::ASTExpression;
use crate::lexer::Token;
use std::collections::HashSet;
use std::collections::LinkedList;

pub struct ExpressionIter {
  declared_variables: HashSet<String>,
  cursor: usize,
  tokens: Vec<Token>,
}

impl ExpressionIter {
  pub fn new(raw_tokens: &LinkedList<Token>) -> Self {
    Self {
      declared_variables: HashSet::new(),
      tokens: ExpressionIter::strip_tokens(raw_tokens),
      cursor: 0,
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
    if let Some(expression) = self.pop_ternary_function() {
      return expression;
    }

    if let Some(expression) = self.pop_variable_operation() {
      return expression;
    }

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
  fn pop_ternary_function(&mut self) -> Option<ASTExpression> {
    match self.current_token() {
      Token::For => {
        self.move_cursor();
        match self.current_token() {
          Token::Name(index) if !self.is_declared_variable(&index) => {
            self.move_cursor();

            let range = { Box::new(self.pop_expression()) };

            self.declared_variables.insert(index.clone());
            let body = { Box::new(self.pop_expression()) };
            self.declared_variables.remove(&index);

            Some(ASTExpression::ForDefinition { index, range, body })
          }
          _ => panic!("Incorrect index name"),
        }
      }
      Token::If => {
        self.move_cursor();

        let condition = Box::new(self.pop_expression());
        let then_clause = Box::new(self.pop_expression());
        let else_clause = Box::new(self.pop_expression());

        Some(ASTExpression::IFDefinition {
          condition,
          then_clause,
          else_clause,
        })
      }
      _ => None,
    }
  }

  #[inline]
  fn pop_variable_operation(&mut self) -> Option<ASTExpression> {
    match self.current_token() {
      Token::Set => {
        self.move_cursor();
        match self.current_token() {
          Token::Name(variable) if self.is_declared_variable(&variable) => {
            self.move_cursor();
            Some(ASTExpression::SetDefinition {
              name: variable.clone(),
              new_value: Box::new(self.pop_expression()),
            })
          }
          _ => panic!("Incorrect variable name"),
        }
      }
      Token::Var => {
        self.move_cursor();
        match self.current_token() {
          Token::Name(variable) if !self.is_declared_variable(&variable) => {
            self.move_cursor();
            self.declared_variables.insert(variable.clone());
            Some(ASTExpression::VarDefinition {
              name: variable.clone(),
              value: Box::new(self.pop_expression()),
            })
          }
          _ => panic!("Incorrect variable name"),
        }
      }
      _ => None,
    }
  }

  #[inline]
  fn pop_binary_function(&mut self) -> Option<ASTExpression> {
    let current_token = self.current_token();

    let (left, right) = match current_token {
      Token::Bigger
      | Token::Less
      | Token::Eq
      | Token::Plus
      | Token::Minus
      | Token::Divide
      | Token::Multiply
      | Token::BinaryAnd
      | Token::BinaryOr
      | Token::Range => self.reduce_binary_function(),
      _ => return None,
    };

    Some(match current_token {
      Token::Bigger => ASTExpression::BiggerExpression { left, right },
      Token::Less => ASTExpression::LessExpression { left, right },
      Token::Eq => ASTExpression::EqExpression { left, right },
      Token::Divide => ASTExpression::DivideExpression { left, right },
      Token::Multiply => ASTExpression::MultiplyExpression { left, right },
      Token::Minus => ASTExpression::MinusExpression { left, right },
      Token::Plus => ASTExpression::PlusExpression { left, right },
      Token::BinaryAnd => ASTExpression::BinaryAndExpression { left, right },
      Token::BinaryOr => ASTExpression::BinaryOrExpression { left, right },
      Token::Range => ASTExpression::RangeDefinition {
        start: left,
        end: right,
      },
      _ => return None,
    })
  }

  #[inline]
  fn reduce_binary_function(&mut self) -> (Box<ASTExpression>, Box<ASTExpression>) {
    self.move_cursor();

    let left = self.pop_expression();
    let right = self.pop_expression();

    (Box::new(left), Box::new(right))
  }

  #[inline]
  fn pop_unary_function(&mut self) -> Option<ASTExpression> {
    match self.current_token() {
      Token::OpenBracket => {
        self.move_cursor();

        let result = self.pop_expression();

        assert!(matches!(self.current_token(), Token::CloseBracket));
        self.move_cursor();

        Some(result)
      }
      Token::BinaryNeg => {
        self.move_cursor();
        Some(ASTExpression::BinaryNegExpression(Box::new(
          self.pop_expression(),
        )))
      }
      _ => None,
    }
  }

  #[inline]
  fn pop_literal(&mut self) -> Option<ASTExpression> {
    match self.current_token() {
      Token::Number(value) => {
        self.move_cursor();
        Some(ASTExpression::NumberLiteral(value.clone()))
      }
      Token::Bool(value) => {
        self.move_cursor();
        Some(ASTExpression::BoolLiteral(value.clone()))
      }
      Token::Name(name) if self.is_declared_variable(&name) => {
        self.move_cursor();
        Some(ASTExpression::NameReference(name.clone()))
      }
      _ => None,
    }
  }

  #[inline]
  fn is_declared_variable(&self, name: &str) -> bool {
    self.declared_variables.contains(name)
  }

  #[inline]
  fn current_token(&self) -> Token {
    self.tokens[self.cursor].clone()
  }

  #[inline]
  fn move_cursor(&mut self) {
    self.cursor += 1;
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
