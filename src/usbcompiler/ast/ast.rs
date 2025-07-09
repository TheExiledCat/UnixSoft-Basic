#![allow(non_camel_case_types)]

// === Core Types ===

#[derive(Debug)]
pub enum DataType {
    INT,
    FLOAT,
    STRING,
    BOOL,
}

#[derive(Debug)]
pub struct IdentifierNode {
    pub name: String,
}

#[derive(Debug)]
pub struct ConstantNode {
    pub data_type: DataType,
    pub value: String,
}

// === Expressions ===

#[derive(Debug)]
pub enum BinaryOpKind {
    ADD,
    SUB,
    MUL,
    DIV,
    EQ,
    NEQ,
    LT,
    GT,
    LTE,
    GTE,
    AND,
    OR,
}

#[derive(Debug)]
pub enum UnaryOpKind {
    NEG,
    NOT,
}

#[derive(Debug)]
pub struct BinaryOpNode {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operation: BinaryOpKind,
}

#[derive(Debug)]
pub struct UnaryOpNode {
    pub target: Box<Expression>,
    pub operation: UnaryOpKind,
}

#[derive(Debug)]
pub struct FunctionCallNode {
    pub name: String,
    pub arguments: Vec<Expression>,
}

#[derive(Debug)]
pub enum Expression {
    IDENTIFIER(IdentifierNode),
    CONSTANT(ConstantNode),
    UNARY_OP(UnaryOpNode),
    BINARY_OP(BinaryOpNode),
    FUNC_CALL(FunctionCallNode),
}

// === Statements ===

#[derive(Debug)]
pub struct AssignmentNode {
    pub target: IdentifierNode,
    pub value: Expression,
}

#[derive(Debug)]
pub struct VariableDeclarationNode {
    pub variable_name: IdentifierNode,
    pub data_type: Option<DataType>,
    pub is_constant: bool,
    pub initial_value: Option<Box<Expression>>,
}

#[derive(Debug)]
pub struct IfStatementNode {
    pub condition: Box<Expression>,
    pub action: Box<Statement>,
    pub else_action: Option<Box<Statement>>,
}

#[derive(Debug)]
pub struct ScopeNode {
    pub statements: Vec<AstNode>,
}

#[derive(Debug)]
pub struct ReturnNode {
    return_expression: Option<Box<Expression>>,
}

#[derive(Debug)]
pub enum Statement {
    ASSIGN(AssignmentNode),
    IF(IfStatementNode),
    SCOPE(ScopeNode),
    VAR_DECL(VariableDeclarationNode),
    FUNC_CALL(FunctionCallNode),
    RETURN(ReturnNode),
}

// === Root Node ===

#[derive(Debug)]
pub enum AstNode {
    EXPRESSION(Expression),
    STATEMENT(Statement),
}
