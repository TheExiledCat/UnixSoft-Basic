use crate::usbcompiler::{
    ast::ast::{AstNode, Expression, ScopeNode, Statement},
    errors::error::ParserError,
    tokenizer::lexer::{Span, Token, TokenKind},
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

type ExpressionHandler = fn(&mut Parser, Token) -> Result<Option<Expression>, ParserError>;
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
            if pos >= amount {
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
    fn get_expression_handlers(&self) -> Vec<ExpressionHandler> {
        return vec![Self::handle_single_constant];
    }
    fn handle_single_constant(&mut self, token: Token) -> Result<Option<Expression>, ParserError> {
        if let TokenKind::Number(num) = &token.kind {
            if let Some(op) = self.peek(1) {
                if let TokenKind::Operator(_) = op.kind {
                    return Ok(None);
                }
            }
            let val: i64 = num.parse().unwrap();
        }
        return Ok(None);
    }
    fn parse_expression(&mut self) -> Result<Expression, Vec<ParserError>> {
        let mut errors: Vec<ParserError> = Vec::new();
        let mut expression: Option<Expression> = None;
        let span: Span;
        let handlers = self.get_expression_handlers();
        if let Some(token) = self.current_token.clone() {
            span = token.position_span.clone();
            for handler in handlers {
                let handled = handler(self, token.clone());
                match handled {
                    Ok(e) => {
                        if let Some(_expression) = e {
                            expression = Some(_expression);
                            break;
                        }
                    }
                    Err(e) => errors.push(e),
                }
            }
            if let None = expression {
                errors.push(ParserError::UnexpectedToken { span, token });
            }
        }
        if errors.len() > 0 {
            return Err(errors);
        }

        self.advance();
        return Ok(expression.unwrap());
    }
    fn parse_statement(&mut self) -> Result<Expression, Vec<ParserError>> {
        todo!();
    }
}
