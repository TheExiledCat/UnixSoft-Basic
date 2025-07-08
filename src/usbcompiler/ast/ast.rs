#![allow(non_camel_case_types)]

// === Core Types ===

pub enum DataType {
    INT,
    FLOAT,
    STRING,
    BOOL,
}

pub struct IdentifierNode {
    pub name: String,
}

pub struct ConstantNode {
    pub data_type: DataType,
    pub value: String,
}

// === Expressions ===

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

pub enum UnaryOpKind {
    NEG,
    NOT,
}

pub struct BinaryOpNode {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
    pub operation: BinaryOpKind,
}

pub struct UnaryOpNode {
    pub target: Box<Expression>,
    pub operation: UnaryOpKind,
}

pub struct FunctionCallNode {
    pub name: String,
    pub arguments: Vec<Expression>,
}

pub enum Expression {
    IDENTIFIER(IdentifierNode),
    CONSTANT(ConstantNode),
    UNARY_OP(UnaryOpNode),
    BINARY_OP(BinaryOpNode),
    FUNC_CALL(FunctionCallNode),
}

// === Statements ===

pub struct AssignmentNode {
    pub target: IdentifierNode,
    pub value: Expression,
}

pub struct VariableDeclarationNode {
    pub variable_name: IdentifierNode,
    pub data_type: Option<DataType>,
    pub is_constant: bool,
    pub initial_value: Option<Box<Expression>>,
}

pub struct IfStatementNode {
    pub condition: Box<Expression>,
    pub action: Box<Statement>,
    pub else_action: Option<Box<Statement>>,
}

pub struct ScopeNode {
    pub statements: Vec<AstNode>,
}

pub struct ReturnNode {
    return_expression: Option<Box<Expression>>,
}

pub enum Statement {
    ASSIGN(AssignmentNode),
    IF(IfStatementNode),
    SCOPE(ScopeNode),
    VAR_DECL(VariableDeclarationNode),
    FUNC_CALL(FunctionCallNode),
    RETURN(ReturnNode),
}

// === Root Node ===

pub enum AstNode {
    EXPRESSION(Expression),
    STATEMENT(Statement),
}
