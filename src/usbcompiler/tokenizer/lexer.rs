#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Add more token variants as needed
    Number(String),
    StringLiteral(String),
    Identifier(String),
    Keyword(String),
    Operator(String),
    Newline,
    Colon,
    Comma,
    ParenOpen,
    ParenClose,
    EOF,
}

struct Lexer {
    input: Vec<char>,
    position: usize,
    current_char: Option<char>,
}

impl Lexer {
    pub fn new(script: String, start_position: usize) -> Self {
        let mut lexer = Lexer {
            input: script.chars().collect(),
            position: start_position,
            current_char: None,
        };

        lexer.advance();
        return lexer;
    }
    fn peek(&mut self, count: usize) -> Option<char> {
        todo!();
    }
    fn peek_word(&mut self) -> String {
        todo!();
    }
    fn consume(&mut self, count: usize) -> String {
        todo!();
    }
    fn consume_whitespace(&mut self) {
        todo!();
    }
    fn consume_word(&mut self) -> String {
        todo!();
    }
    fn advance(&mut self) {
        if self.position < self.input.len() {
            self.current_char = Some(self.input[self.position]);
            self.position += 1;
        } else {
            self.current_char = None;
        }
    }
    pub fn next_token(&mut self) -> Token {
        todo!();
    }
}

pub const APPLESOFT_KEYWORDS: &'static [&'static str] = &[
    "END", "FOR", "NEXT", "DATA", "INPUT", "DEL", "DIM", "READ", "GR", "TEXT", "PR#", "IN#",
    "CALL", "PLOT", "HLIN", "VLIN", "HGR2", "HGR", "HCOLOR=", "HPLOT", "DRAW", "XDRAW", "HTAB",
    "HOME", "ROT=", "SCALE=", "SHLOAD", "TRACE", "NOTRACE", "NORMAL", "INVERSE", "FLASH", "COLOR=",
    "POP", "VTAB", "HIMEM:", "LOMEM:", "ONERR", "RESUME", "RECALL", "STORE", "SPEED=", "LET",
    "GOTO", "RUN", "IF", "RESTORE", "GOSUB", "RETURN", "REM", "STOP", "ON", "WAIT", "LOAD", "SAVE",
    "DEF FN", "POKE", "PRINT", "CONT", "LIST", "CLEAR", "GET", "NEW", "TO", "FN", "THEN", "AT",
    "STEP",
];

pub const APPLESOFT_FUNCTIONS: &'static [&'static str] = &[
    "SGN", "INT", "ABS", "USR", "FRE", "SCRN", "PDL", "POS", "SQR", "RND", "LOG", "EXP", "COS",
    "SIN", "TAN", "ATN", "PEEK", "LEN", "STR$", "VAL", "ASC", "CHR$", "LEFT$", "RIGHT$", "MID$",
];
pub const APPLESOFT_OPERATORS: &'static [&'static str] =
    &["+", "-", "*", "/", "^", ">", "=", "<", "AND", "OR", "NOT"];
pub const UNIXSOFT_KEYWORDS: &'static [&'static str] = &[];
pub const UNIXSOFT_FUNCTIONS: &'static [&'static str] = &[];
pub const UNIXSOFT_OPERATORS: &'static [&'static str] = &[">=", "<=", "!="];
