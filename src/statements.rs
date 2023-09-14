use crate::expressions::{Expression, IdentifierExpression};
use crate::token::Token;

#[derive(PartialEq)]
pub enum Statement {
    Expression(ExpressionStatement),
    VariableDeclaration(VariableDeclarationStatement),
    Assignment(AssignmentStatement),
    Block(BlockStatement),
    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    FunctionDeclaration(FunctionDeclarationStatement),
    Return(ReturnStatement),
    ClassDeclaration(ClassDeclarationStatement),
    Lambda(LambdaExpression)
}

pub struct ExpressionStatement {
    pub expression: Expression
}

pub struct VariableDeclarationStatement {
    pub identifier: IdentifierExpression,
    pub initialization: Option<Expression>
}

pub struct AssignmentStatement {
    pub identifier: IdentifierExpression,
    pub value: Expression
}

pub struct BlockStatement {

}

pub struct IfStatement {
    pub condition: Expression,
    pub then_branch: Vec<Statement>,
    pub else_branch: Vec<Statement>
}

pub struct WhileStatement {
    pub condition: Expression,
    pub body: Vec<Statement>
}

pub struct ForStatement {
    pub variable: Token,
    pub initializer: Expression,
    pub condition: Expression,
    pub increment: Expression,
    pub body: Vec<Statement>
}

pub struct FunctionDeclarationStatement {
    pub name: IdentifierExpression,
    pub parameters: Vec<Token>,
    pub body: Vec<Statement>
}

pub struct ReturnStatement {
    pub value: Option<Expression>
}

pub struct ClassDeclarationStatement {
    pub name: IdentifierExpression,
    pub properties: Vec<VariableDeclarationStatement>,
    pub methods: Vec<FunctionDeclarationStatement>
}

pub struct LambdaExpression {
    pub parameters: Vec<Token>,
    pub body: Vec<Statement>
}