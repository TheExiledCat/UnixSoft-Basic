use crate::stdlib::{ENC_FUNCTIONS, POS_FUNCTIONS};
use crate::usbcompiler::errors::error::LexerError;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    // Add more token variants as needed
    Number(String),
    StringLiteral(String),
    Identifier(String, bool),
    Keyword(String),
    Operator(String),
    Newline,
    Colon,
    Comma,
    ParenOpen,
    ParenClose,
    BrackOpen,
    BrackClose,
    EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    kind: TokenKind,
    position_flat: usize,
    position_span: Span,
}
trait NoEof {
    fn no_eof(&mut self) -> &mut Self;
}
impl Token {}
impl NoEof for Vec<TokenKind> {
    fn no_eof(&mut self) -> &mut Self {
        if let TokenKind::EOF = self.last().unwrap() {
            self.pop();
        }
        return self;
    }
}
#[repr(u8)]
#[derive(PartialEq, Eq, Debug)]
enum CharType {
    None = 0,
    Numeric,
    Alphabetic,
    OperatorSymbol,
    Delimiter,
    Whitespace,
    Quote,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}
impl Span {
    pub fn new(line: usize, column: usize, length: usize) -> Self {
        return Self {
            line,
            column,
            length,
        };
    }
}
pub struct Lexer {
    input: Vec<char>,
    position_flat: usize,
    position_span: Span,
    current_char: Option<char>,
}

type TokenRecognizer = fn(&mut Lexer, char) -> Result<Option<Token>, LexerError>;
impl Lexer {
    ///creates a new USB Lexer given a string to tokenize. input string is expected to start on the start of a line or statement
    pub fn new(script: String) -> Self {
        let mut lexer = Lexer {
            input: script.chars().collect(),
            position_flat: 0,
            current_char: None,
            position_span: Span::new(1, 1, 1),
        };

        lexer.advance();
        return lexer;
    }
    ///peek into the character stream by **count** indices, returns a [`Some`] containing the peeked char if the offset given was not out of bounds. else returns [`None`]
    fn peek(&mut self, count: usize) -> Option<char> {
        return self.input.get(self.position_flat + count).copied();
    }

    ///peek at the next word in the stream, returning it as a copied [`String`] the position of the lexer has to be on a non white space character or this will always return [`None`]
    fn peek_word(&mut self) -> Option<(Span, CharType, String)> {
        let current_char = &self.current_char;

        if let None = current_char {
            return None;
        }

        let c = current_char.unwrap();

        if c.is_whitespace() {
            return None;
        }
        let start_line = self.position_span.line;
        let start_col = self.position_span.column;
        let mut peek_pos = 0;
        let mut last_char_type = Lexer::get_char_type(&c);
        let mut total = String::new();
        total.push(c);

        while let Some(c) = self.peek(peek_pos) {
            let new_char_type = Lexer::get_char_type(&c);
            if last_char_type != CharType::None {
                if last_char_type != new_char_type
                    && !(last_char_type == CharType::Alphabetic
                        && new_char_type == CharType::Numeric)
                {
                    break;
                }
            }

            last_char_type = new_char_type;
            total.push(c);
            peek_pos += 1;
        }

        let first_char = total.as_bytes()[0].clone() as char;
        last_char_type = Lexer::get_char_type(&first_char);

        return Some((
            Span::new(start_line, start_col, total.chars().count()),
            last_char_type,
            total,
        ));
    }

    ///peek at the next none whitespace character in the stream, returning a [`Some`] containing a tuple with the offsett to the next non whitespace char and the char at that position
    fn peek_next_non_whitespace(&mut self) -> Option<(usize, char)> {
        let mut peek_pos = 0;
        while let Some(c) = self.peek(peek_pos) {
            if !&c.is_whitespace() {
                return Some((peek_pos, c.clone()));
            }
            peek_pos += 1;
        }
        return None;
    }

    fn consume(&mut self, count: usize) -> Option<(Span, String)> {
        let mut total = String::new();
        if let None = self.current_char {
            return None;
        }

        let start_line = self.position_span.line;
        let start_column = self.position_span.column;
        for _i in 0..count {
            if let Some(c) = self.current_char {
                total.push(c);
                self.advance();
            } else {
                break;
            }
        }

        return Some((
            Span::new(start_line, start_column, total.chars().count()),
            total,
        ));
    }
    fn consume_whitespace(&mut self) {
        while let Some(c) = self.current_char {
            if c == '\n' || !c.is_whitespace() {
                break;
            }
            self.advance();
        }
    }
    /// consumes the next word in the stream, skipping any white space first and returning the consumed word.
    ///
    /// A word is any of the following:
    /// - a string of alphabetics containing 0 or more numerics positioned after the first character.
    /// - an operator or a compound operator e.g. <= or =
    /// - a delimiter
    /// - a string literal including both the start and end quotes
    fn consume_word(&mut self) -> Option<(Span, CharType, String)> {
        let word = self.peek_word();
        if let Some(w) = word {
            self.skip(w.2.chars().count() - 1);
            return Some(w);
        }
        return None;
    }
    fn consume_string_literal(&mut self) -> Result<(Span, String), LexerError> {
        let mut literal = String::new();
        if let Some(c) = self.current_char {
            if c != '\"' {
                panic!("consume string called on non string");
            }
            literal.push(c);
        }

        let start_line = self.position_span.line;
        let start_col = self.position_span.column;
        self.advance();
        while let Some(c) = self.current_char {
            if c == '\\' {
                //escape string chars, for now, just store the next char and continue
                literal.push(c);
                self.advance();
                if let Some(n) = self.current_char {
                    literal.push(n);
                }
                self.advance();
                continue;
            }
            if c == '\"' {
                literal.push(c);
                break;
            }
            literal.push(c);
            self.advance();
        }

        if let None = self.current_char {
            return Err(LexerError::UnterminatedString(Span::new(
                start_line, start_col, 0,
            )));
        }

        return Ok((
            Span::new(start_line, start_col, literal.chars().count()),
            literal,
        ));
    }
    fn advance(&mut self) {
        if self.position_flat < self.input.len() {
            let c = self.input[self.position_flat];
            self.current_char = Some(c);
            self.position_flat += 1;

            if c == '\n' {
                self.position_span.line += 1;
                self.position_span.column = 1;
            } else {
                self.position_span.column += 1;
            }

            self.position_span.length += 1;
        } else {
            self.current_char = None;
        }
    }

    /// advance the through the stream by `count` steps
    fn skip(&mut self, count: usize) {
        let mut index = 0;
        while let Some(_) = self.current_char {
            if index >= count {
                break;
            }
            self.advance();
            index += 1;
        }
    }

    fn get_char_type(character: &char) -> CharType {
        return if character.is_whitespace() {
            CharType::Whitespace
        } else if character.is_alphabetic() {
            CharType::Alphabetic
        } else if character.is_ascii_digit() {
            CharType::Numeric
        } else if APPLESOFT_OPERATORS.contains(&character.to_string().as_str())
            || UNIXSOFT_OPERATORS.contains(&character.to_string().as_str())
        {
            CharType::OperatorSymbol
        } else if UNIXSOFT_DELIMITERS.contains(&character) {
            CharType::Delimiter
        } else if character.clone() == '\"' {
            CharType::Quote
        } else {
            panic!("Invalid symbol detected");
        };
    }
    /// get all the symbol handlers that resolve symbols into tokens.
    /// Note: the order of these handlers matters as this is the order they will be evaluated in
    /// If any of the handlers returns a token the rest of the handlers will not be used. E.g. if the keyword handler finds that the current word is a keyword, the identifier handler wont be called on that same word
    /// TLDR; Order this list by precedence
    fn get_handlers(&self) -> Vec<TokenRecognizer> {
        return vec![
            Self::handle_newline,
            Self::handle_delimiter,
            Self::handle_string_literal,
            Self::handle_integer_literal,
            Self::handle_operator,
            Self::handle_keyword,
            Self::handle_identifier,
        ];
    }
    pub fn next_token(&mut self) -> Result<Option<Token>, LexerError> {
        self.consume_whitespace();

        let current = match self.current_char {
            Some(c) => c,
            None => {
                let eof = Ok(Some(Token {
                    kind: TokenKind::EOF,
                    position_flat: self.position_flat,
                    position_span: self.position_span.clone(),
                }));
                self.advance();
                return eof;
            }
        };
        let handlers = self.get_handlers();
        let mut token: Option<Token> = None;
        for handler in handlers {
            let result = handler(self, current);
            match result {
                Ok(found) => match found {
                    Some(t) => {
                        token = Some(t);
                        break;
                    }
                    _ => (),
                },
                Err(e) => return Err(e),
            }
        }

        self.advance();
        if let None = token {
            return Err(LexerError::InvalidChar(self.position_span.clone(), current));
        }

        return Ok(token);
    }
    pub fn tokenize(&mut self) -> Result<Vec<Token>, (Vec<Token>, Vec<LexerError>)> {
        let mut tokens = Vec::new();
        let mut errors = Vec::new();

        loop {
            match self.next_token() {
                Ok(t) => {
                    if let Some(t) = t {
                        let kind = t.kind.clone();
                        tokens.push(t);
                        if let TokenKind::EOF = kind {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                Err(e) => errors.push(e),
            }
        }

        if errors.len() > 0 {
            return Err((tokens, errors));
        }
        return Ok(tokens);
    }
    fn handle_delimiter(&mut self, character: char) -> Result<Option<Token>, LexerError> {
        let kind;
        if character == '(' {
            kind = TokenKind::ParenOpen;
        } else if character == ')' {
            kind = TokenKind::ParenClose;
        } else if character == '[' {
            kind = TokenKind::BrackOpen;
        } else if character == ']' {
            kind = TokenKind::BrackOpen;
        } else if character == ',' {
            kind = TokenKind::Comma;
        } else if character == ':' {
            kind = TokenKind::Colon;
        } else {
            return Ok(None);
        }

        return Ok(Some(Token {
            kind,
            position_flat: self.position_flat,
            position_span: self.position_span.clone(),
        }));
    }

    fn handle_newline(&mut self, character: char) -> Result<Option<Token>, LexerError> {
        if character != '\n' {
            return Ok(None);
        }
        let token = Some(Token {
            kind: TokenKind::Newline,
            position_flat: self.position_flat,
            position_span: self.position_span.clone(),
        });
        return Ok(token);
    }
    fn handle_string_literal(&mut self, character: char) -> Result<Option<Token>, LexerError> {
        if character != '\"' {
            return Ok(None);
        }

        let result = self.consume_string_literal()?;
        return Ok(Some(Token {
            kind: TokenKind::StringLiteral(result.1),
            position_flat: self.position_flat,
            position_span: result.0,
        }));
    }
    fn handle_integer_literal(&mut self, character: char) -> Result<Option<Token>, LexerError> {
        if !character.is_ascii_digit() {
            return Ok(None);
        }

        let word = self.consume_word();
        if let Some(tuple) = word {
            match tuple.1 {
                CharType::Numeric => (),
                _ => panic!("unexpected token: {:#?} of type {:#?}", tuple.2, tuple.1),
            }

            let token = Some(Token {
                kind: TokenKind::Number(tuple.2),
                position_flat: self.position_flat,
                position_span: tuple.0,
            });
            return Ok(token);
        }
        return Err(LexerError::UnexpectedEof);
    }
    fn handle_operator(&mut self, character: char) -> Result<Option<Token>, LexerError> {
        let word = self.peek_word();

        if let Some(t) = word {
            if !APPLESOFT_OPERATORS.contains(&t.2.as_str())
                && !UNIXSOFT_OPERATORS.contains(&t.2.as_str())
            {
                return Ok(None);
            }
            self.consume_word();

            return Ok(Some(Token {
                kind: TokenKind::Operator(t.2),
                position_flat: self.position_flat,
                position_span: t.0,
            }));
        }
        return Ok(None);
    }
    fn handle_keyword(&mut self, character: char) -> Result<Option<Token>, LexerError> {
        if !character.is_ascii_alphabetic() {
            return Ok(None);
        }

        let word = self.peek_word();

        if let Some(t) = word {
            if APPLESOFT_KEYWORDS.contains(&t.2.to_uppercase().as_str())
                || UNIXSOFT_KEYWORDS.contains(&t.2.to_uppercase().as_str())
            {
                self.consume_word();
                return Ok(Some(Token {
                    kind: TokenKind::Keyword(t.2),
                    position_flat: self.position_flat,
                    position_span: t.0,
                }));
            }
        }
        return Ok(None);
    }
    fn handle_identifier(&mut self, character: char) -> Result<Option<Token>, LexerError> {
        if !character.is_ascii_alphabetic() {
            return Ok(None);
        }
        println!(
            "Handling ascii alphabetic for identifier starting with: {}",
            character
        );

        let word = self.peek_word();

        if let Some(t) = word {
            self.consume_word();
            return Ok(Some(Token {
                kind: TokenKind::Identifier(t.2),
                position_flat: self.position_flat,
                position_span: t.0,
            }));
        }
        return Ok(None);
    }
}

pub const APPLESOFT_KEYWORDS: &'static [&'static str] = &[
    "END", "FOR", "NEXT", "DATA", "INPUT", "DEL", "DIM", "READ", "GR", "TEXT", "PR#", "IN#",
    "CALL", "PLOT", "HLIN", "VLIN", "HGR2", "HGR", "HCOLOR", "HPLOT", "DRAW", "XDRAW", "HTAB",
    "HOME", "ROT=", "SCALE=", "SHLOAD", "TRACE", "NOTRACE", "NORMAL", "INVERSE", "FLASH", "COLOR",
    "POP", "VTAB", "HIMEM:", "LOMEM:", "ONERR", "RESUME", "RECALL", "STORE", "SPEED=", "LET",
    "GOTO", "RUN", "IF", "RESTORE", "GOSUB", "RETURN", "REM", "STOP", "WAIT", "LOAD", "SAVE",
    "DEF", "POKE", "PRINT", "CONT", "LIST", "CLEAR", "GET", "NEW", "TO", "FN", "THEN", "AT",
    "STEP",
];

pub const APPLESOFT_FUNCTIONS: &'static [&'static str] = &[
    "SGN", "ABS", "USR", "FRE", "SCRN", "PDL", "POS", "SQR", "RND", "LOG", "EXP", "COS", "SIN",
    "TAN", "ATN", "PEEK", "LEN", "STR", "VAL", "ASC", "CHR", "LEFT", "RIGHT", "MID",
];
pub const APPLESOFT_OPERATORS: &'static [&'static str] =
    &["+", "-", "*", "/", "^", ">", "=", "<", "AND", "OR", "NOT"];
pub const UNIXSOFT_KEYWORDS: &'static [&'static str] = &[
    "TRUE", "FALSE", "//", "DATE", "DAY", "HOUR", "MINUTE", "SECOND", "TIME", "BEGIN", "FIN",
    "MENU", "POPTIONS", "CSCOPE", "ENUM", "PENUM",
];
pub const UNIXSOFT_FUNCTIONS: &'static [&'static str] = &["INT", "FLOAT", "BOOL"];
pub const UNIXSOFT_OPERATORS: &'static [&'static str] = &[">=", "<=", "!="];
pub const UNIXSOFT_DELIMITERS: [char; 6] = ['(', ')', '[', ']', ',', ':'];

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_script_tokens(input: &str, expected_tokens: Vec<TokenKind>, strip_eof: bool) {
        let mut lexer = Lexer::new(input.into());
        let result = lexer.tokenize();

        let mut tokens = None;
        let mut errors: Option<Vec<LexerError>> = None;
        if let Ok(t) = result {
            tokens = Some(t);
        } else if let Err(t) = result {
            tokens = Some(t.0);
            errors = Some(t.1)
        }
        assert!(
            errors.is_none(),
            "Lexer returned one or more errors: {:#?}\nTokens: {:#?}",
            errors,
            tokens,
        );

        let mut actual_tokens: Vec<TokenKind> =
            tokens.unwrap().iter().map(|t| t.kind.clone()).collect();
        if strip_eof {
            actual_tokens.no_eof();
        }
        assert_eq!(actual_tokens, expected_tokens);
    }
    #[test]
    fn test_all_keywords_uppercase() {
        let operators: Vec<String> = APPLESOFT_OPERATORS
            .iter()
            .map(|o| o.to_uppercase())
            .chain(UNIXSOFT_OPERATORS.iter().map(|o| o.to_uppercase()))
            .collect();
        assert_eq!(
            APPLESOFT_OPERATORS
                .iter()
                .map(|o| o.to_owned())
                .chain(UNIXSOFT_OPERATORS.iter().map(|o| o.to_owned()))
                .map(|o| o.to_owned())
                .collect::<Vec<String>>(),
            operators
        );
        let keywords: Vec<String> = APPLESOFT_KEYWORDS
            .iter()
            .map(|o| o.to_uppercase())
            .chain(UNIXSOFT_KEYWORDS.iter().map(|o| o.to_uppercase()))
            .collect();
        assert_eq!(
            APPLESOFT_KEYWORDS
                .iter()
                .map(|o| o.to_owned())
                .chain(UNIXSOFT_KEYWORDS.iter().map(|o| o.to_owned()))
                .map(|o| o.to_owned())
                .collect::<Vec<String>>(),
            keywords
        );
        let functions: Vec<String> = APPLESOFT_FUNCTIONS
            .iter()
            .map(|o| o.to_uppercase())
            .chain(UNIXSOFT_FUNCTIONS.iter().map(|o| o.to_uppercase()))
            .collect();
        assert_eq!(
            APPLESOFT_FUNCTIONS
                .iter()
                .map(|o| o.to_owned())
                .chain(UNIXSOFT_FUNCTIONS.iter().map(|o| o.to_owned()))
                .map(|o| o.to_owned())
                .collect::<Vec<String>>(),
            functions
        );
    }
    #[test]
    fn test_recognize_simple_keyword() {
        let input = "PRINT 5";
        let expected_tokens = vec![
            TokenKind::Keyword("PRINT".into()),
            TokenKind::Number("5".into()),
        ];
        assert_script_tokens(input, expected_tokens, true);
    }
    #[test]
    fn test_recognize_words() {
        let input = "10 PRINT5 10";
        let expected_tokens = vec![
            TokenKind::Number("10".into()),
            TokenKind::Identifier("PRINT5".into()),
            TokenKind::Number("10".into()),
        ];
        assert_script_tokens(input, expected_tokens, true);

        let input = "5PRINT 10";
        let expected_tokens = vec![
            TokenKind::Number("5".into()),
            TokenKind::Keyword("PRINT".into()),
            TokenKind::Number("10".into()),
        ];
        assert_script_tokens(input, expected_tokens, true);
    }
    #[test]
    fn test_newlines() {
        let input = "10 PRINT 5\n20 PRINT 10";
        let expected_tokens = vec![
            TokenKind::Number("10".into()),
            TokenKind::Keyword("PRINT".into()),
            TokenKind::Number("5".into()),
            TokenKind::Newline,
            TokenKind::Number("20".into()),
            TokenKind::Keyword("PRINT".into()),
            TokenKind::Number("10".into()),
        ];
        assert_script_tokens(input, expected_tokens, true);
    }
    #[test]
    fn test_string_simple_literals() {
        let input = "10 PRINT \"Hello, World\"";
        let expected_tokens = vec![
            TokenKind::Number("10".into()),
            TokenKind::Keyword("PRINT".into()),
            TokenKind::StringLiteral("\"Hello, World\"".into()),
        ];
        assert_script_tokens(input, expected_tokens, true);
    }
    #[test]
    fn test_string_escaped_literals() {
        let input = "10 PRINT \"Hello\\\"World\"";
        let expected_tokens = vec![
            TokenKind::Number("10".into()),
            TokenKind::Keyword("PRINT".into()),
            TokenKind::StringLiteral("\"Hello\\\"World\"".into()),
        ];
        assert_script_tokens(input, expected_tokens, true);

        let input = "10 PRINT \"Hello\\nWorld\"";
        let expected_tokens = vec![
            TokenKind::Number("10".into()),
            TokenKind::Keyword("PRINT".into()),
            TokenKind::StringLiteral("\"Hello\\nWorld\"".into()),
        ];
        assert_script_tokens(input, expected_tokens, true);
    }
    #[test]
    fn test_recognize_single_or_double_operators() {
        let input = "LET X = 10\nPRINT X + 5";
        let expected_tokens = vec![
            TokenKind::Keyword("LET".into()),
            TokenKind::Identifier("X".into()),
            TokenKind::Operator("=".into()),
            TokenKind::Number("10".into()),
            TokenKind::Newline,
            TokenKind::Keyword("PRINT".into()),
            TokenKind::Identifier("X".into()),
            TokenKind::Operator("+".into()),
            TokenKind::Number("5".into()),
        ];
        assert_script_tokens(input, expected_tokens.clone(), true);

        let input = "LET X = 10\nPRINT X+5";
        assert_script_tokens(input, expected_tokens, true);

        let input = "PRINT X <= 5";
        let expected_tokens = vec![
            TokenKind::Keyword("PRINT".into()),
            TokenKind::Identifier("X".into()),
            TokenKind::Operator("<=".into()),
            TokenKind::Number("5".into()),
        ];
        assert_script_tokens(input, expected_tokens, true);

        let input = "LET X = TRUE AND TRUE";
        let expected_tokens = vec![
            TokenKind::Keyword("LET".into()),
            TokenKind::Identifier("X".into()),
            TokenKind::Operator("=".into()),
            TokenKind::Keyword("TRUE".into()),
            TokenKind::Operator("AND".into()),
            TokenKind::Keyword("TRUE".into()),
        ];
        assert_script_tokens(input, expected_tokens, true);
    }
    #[test]
    fn test_delimiters() {
        let input = "DEF my_func x,y,z = x+y+z\nLET a = my_func(1,2,3)";
        let expected_tokens = vec![
            TokenKind::Keyword("DEF".into()),
            TokenKind::Identifier("my_func".into()),
            TokenKind::Identifier("x".into()),
            TokenKind::Comma,
            TokenKind::Identifier("y".into()),
            TokenKind::Comma,
            TokenKind::Identifier("z".into()),
            TokenKind::Operator("=".into()),
            TokenKind::Identifier("x".into()),
            TokenKind::Operator("+".into()),
            TokenKind::Identifier("y".into()),
            TokenKind::Operator("+".into()),
            TokenKind::Identifier("z".into()),
            TokenKind::Newline,
            TokenKind::Keyword("LET".into()),
            TokenKind::Identifier("a".into()),
            TokenKind::Operator("=".into()),
            TokenKind::Identifier("my_func".into()),
            TokenKind::ParenOpen,
            TokenKind::Number("1".into()),
            TokenKind::Comma,
            TokenKind::Number("2".into()),
            TokenKind::Comma,
            TokenKind::Number("3".into()),
            TokenKind::ParenClose,
        ];
    }
}
