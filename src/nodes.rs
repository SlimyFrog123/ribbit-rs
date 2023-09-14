use crate::token::Token;

enum Node {
    Identifier(IdentifierNode),
    Variable(VariableNode),
    Initialize(InitializeNode),
    Assign(AssignNode),
    FunctionCall(FunctionCallNode),
    FunctionDefinition(FunctionDefinitionNode),
    Binary(BinaryNode),
    Unary(UnaryNode)
}

enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
    Power
}

enum UnaryOperator {
    Negate,
    Not
}

struct IdentifierNode {
    token: Token
}

struct VariableNode {
    name: IdentifierNode
}

struct InitializeNode {
    name: IdentifierNode,
    variable_type: IdentifierNode,
    value: Option<Node>
}

struct AssignNode {
    name: IdentifierNode,
    value: Node
}

struct FunctionCallNode {
    function: IdentifierNode,
    parameters: Vec<Node>
}

struct FunctionDefinitionNode {
    name: IdentifierNode,
    parameters: Vec<InitializeNode>,
    body: Vec<Node>
}

struct BinaryNode {
    operator: BinaryOperator,
    left: Node,
    right: Node
}

struct UnaryNode {
    operator: UnaryOperator,
    operand: Node
}