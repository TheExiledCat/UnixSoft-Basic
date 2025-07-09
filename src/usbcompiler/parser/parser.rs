use crate::usbcompiler::{
    ast::ast::{AstNode, Expression, ScopeNode, Statement},
    errors::error::ParserError,
    tokenizer::lexer::Token,
};

pub struct Parser {
    input: Vec<Token>,
    position: usize,
    evaluation_context: EvaluationContext,
    current_token: Option<Token>,
}
pub enum EvaluationContext {
    Imported,
    FileLevel,
    Nested,
}
type AstHandler = fn(&mut Parser, char) -> Result<Option<AstNode>, ParserError>;

impl Parser {
    pub fn new(input: Vec<Token>, evaluation_context: EvaluationContext) -> Self {
        let mut parser = Self {
            input,
            evaluation_context,
            position: 0,
            current_token: None,
        };

        parser.advance();
        return parser;
    }

    fn peek(&self, offset: usize) -> Option<Token> {
        return self.input.get(self.position + offset).cloned();
    }
    fn consume(&mut self, amount: usize) -> Vec<Token> {
        let mut pos = 0;
        let mut tokens = Vec::new();
        while let Some(t) = self.current_token.clone() {
            if (pos >= amount) {
                break;
            }
            self.advance();
            tokens.push(t);
            pos += 1;
        }
        return tokens;
    }
    fn advance(&mut self) {
        if self.position < self.input.len() {
            let t = self.input[self.position].clone();
            self.current_token = Some(t);
            self.position += 1;
        } else {
            self.current_token = None;
        }
    }
    pub fn parse(&mut self) -> Result<AstNode, Vec<ParserError>> {
        let root = match self.evaluation_context {
            EvaluationContext::Imported | EvaluationContext::FileLevel => {
                AstNode::STATEMENT(Statement::SCOPE(ScopeNode {
                    statements: Vec::new(),
                }))
            }
            EvaluationContext::Nested => todo!(),
        };
        return Ok(root);
    }
    fn parse_expression(&mut self) -> Result<Expression, Vec<ParserError>> {
        todo!();
    }
    fn parse_statement(&mut self) -> Result<Expression, Vec<ParserError>> {
        todo!();
    }
}
