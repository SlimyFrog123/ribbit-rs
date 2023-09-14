use crate::statements::Statement;
use crate::token::Token;

#[derive(PartialEq)]
pub enum LiteralValue {
    Integer(i64),
    Float(i64),
    String(String),
    Char(char),
    Boolean(bool)
}

#[derive(PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Power
}

#[derive(PartialEq)]
pub enum UnaryOperator {
    Negate,
    Not
}

#[derive(PartialEq)]
pub enum Expression {
    Literal(LiteralExpression),
    Identifier(IdentifierExpression),
    Binary(BinaryExpression)
}

pub struct LiteralExpression {
    pub value: LiteralValue
}

pub struct IdentifierExpression {
    pub token: Token
}

pub struct BinaryExpression {
    pub operator: BinaryOperator,
    pub left: Box<Expression>,
    pub right: Box<Expression>
}

pub struct UnaryExpression {
    pub operator: UnaryOperator,
    pub operand: Box<Expression>
}

pub struct AssignmentExpression {
    pub identifier: IdentifierExpression,
    pub value: Box<Expression>
}

pub struct FunctionCallExpression {
    pub identifier: IdentifierExpression,
    pub arguments: Vec<Expression>
}

pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub then_branch: Box<Statement>,
    pub else_branch: Box<Statement>
}