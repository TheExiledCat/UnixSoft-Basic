use crate::usbcompiler::{
    ast::ast::{DataType, Expression, IdentifierNode, ScopeNode},
    tokenizer::lexer::{Span, Token},
};

#[derive(Debug)]
pub enum USBError {
    CompilerError(CompilerError),
}
#[derive(Debug)]
pub enum CompilerError {
    TokenizerError(LexerError),
    ParsingError(ParserError),
    SemanticError(SemanticError),
    DeclarationError(DeclarationError),
    MiscError(MiscellaneousError),
}
#[derive(Debug)]
pub enum LexerError {
    InvalidChar(Span, char),
    InvalidNumberLiteral(Span, String),
    UnterminatedString(Span),
    UnexpectedEof,
    InvalidEscapeChar(Span, char),
}
#[derive(Debug)]
pub enum ParserError {
    UnexpectedToken {
        span: Span,
        token: Token,
    },
    MissingToken {
        span: Span,
        expected: String,
    },
    MismatchedDelimiter {
        span: Span,
    },
    InvalidCode {
        span: Span,
    },
    ExtraCode {
        span: Span,
        extra_tokens: Vec<Token>,
    },
}

#[derive(Debug)]
pub enum SemanticError {
    UndefinedIdentifier {
        token: Token,
        identifier: IdentifierNode,
    },
    DuplicateDeclaration {
        token: Token,
        scope: ScopeNode,
    },
    ShadowedIdentifier {
        token: Token,
        identifier: IdentifierNode,
    },
    IllegalIdentifier {
        token: Token,
        identifier: IdentifierNode,
    },
    TypeMismatch {
        token: Token,
        expected_type: DataType,
    },
    InvalidOperation {
        token: Token,
        expression: Expression,
    },
    InvalidCast {
        token: Token,
        expression: Expression,
        target: DataType,
    },
    UnreachableCode {
        token: Token,
    },
    InvalidLoopBreak {
        token: Token,
    },
}
#[derive(Debug)]
pub enum DeclarationError {
    MultipleDefinitions { token: Token },
    InvalidSignature { token: Token },
}
#[derive(Debug)]
pub enum MiscellaneousError {
    InvalidConstant {
        token: Token,
        expression: Expression,
    },
    InvalidMacro {
        token: Token,
    },
}
