pub enum ASTExpression {
  VarDefinition {
    name: String,
    value: Box<ASTExpression>,
  },
  SetDefinition {
    name: String,
    new_value: Box<ASTExpression>,
  },
  IFDefinition {
    condition: Box<ASTExpression>,
    then_clause: Box<ASTExpression>,
    else_clause: Box<ASTExpression>,
  },
  ForDefinition {
    index: String,
    range: Box<ASTExpression>,
    body: Box<ASTExpression>,
  },
  RangeDefinition {
    start: Box<ASTExpression>,
    end: Box<ASTExpression>,
  },
  PlusExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
  },
  MinusExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
  },
  DivideExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
  },
  MultiplyExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
  },
  BinaryAndExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
  },
  BinaryOrExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
  },
  BinaryNegExpression(Box<ASTExpression>),
  EqExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
  },
  BiggerExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
  },
  LessExpression {
    left: Box<ASTExpression>,
    right: Box<ASTExpression>,
  },
  NameReference(String),
  BoolLiteral(bool),
  NumberLiteral(String),
}
