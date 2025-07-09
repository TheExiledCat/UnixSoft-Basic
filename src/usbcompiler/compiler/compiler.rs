use std::{fs::read_to_string, path::PathBuf};

use crate::usbcompiler::{
    errors::error::CompilerError,
    tokenizer::lexer::{Lexer, Token},
};
pub struct Compiler {
    entry_point: PathBuf,
}

impl Compiler {
    pub fn new(entry_point: PathBuf) -> Self {
        return Self { entry_point };
    }
    pub fn compile(&mut self) -> Result<(), Vec<CompilerError>> {
        //tokenize input file
        let entry_content = read_to_string(&self.entry_point).unwrap();

        let mut lexer = Lexer::new(entry_content);
        let mut errors: Vec<CompilerError> = Vec::new();
        // entrypoint gets tokenized
        let tokens: Vec<Token>;
        match lexer.tokenize() {
            Ok(t) => {
                tokens = t;
            }
            Err(errs) => errors.extend(
                errs.1
                    .into_iter()
                    .map(CompilerError::TokenizerError)
                    .collect::<Vec<CompilerError>>(),
            ),
        }

        //parse tokens and create ast

        //if the ast contains Import nodes, tokenize those and inject them into the final ast.

        //generate assembly

        if errors.len() > 0 {
            return Err(errors);
        }
        return Ok(());
    }
}
