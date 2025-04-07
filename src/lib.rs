pub mod lexer;
pub mod parser;
pub mod matcher;

use crate::{
    lexer::{
        Lexer,
        errors::ParseError,
    },
    parser::Parser,
    matcher::Matcher,
};


pub struct RRegex { 
    matcher: Matcher
}

impl RRegex {
    pub fn new(regex: String) -> Result<Self, ParseError> {
        let lexer = Lexer::new(regex);
        let mut parser = Parser::new(lexer);
        let nfa = parser.parse()?;
        let matcher = Matcher::new(nfa);

        Ok(RRegex { matcher })
    }

    pub fn matches(&self, input: &str) -> bool {
        self.matcher.matches(input)
    }
}