use crate::{
    Lexer,
    token::Token,
    errors::ParseError
};
use crate::nfa::NFA;

// Parser v2 lets see how this goes
// This time we are trying to translate the following grammar into code:
// Regex → Alternation
// Alternation → Concatenation ('|' Concatenation) *
// Concatenation → Term+
// Term → Factor Postfix?
//  Factor → Literal | '(' Regex ')' | ε
//  Postfix → '*' | '+' | '?'

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(lexer: &mut Lexer) -> Self {
        let tokens = match lexer.collect().expect("Unexpected None when Parsing") {
            Ok(vec) => vec,
            Err(e) => panic!("{e}"),
        };

        Parser {
            tokens
        }
    }

    pub fn parse(&mut self) -> Result<NFA, ParseError> {
        self.parse_alternation()
    }

    fn parse_alternation(&mut self) -> Result<NFA, ParseError> {
        let mut nfa = self.parse_concatenation()?;

        while self.consume_if(Token::Union) {
            let rhs = match self.peek() {
                Some(Token::Literal(_)) | 
                Some(Token::LParen) | 
                Some(Token::Union) => self.parse_concatenation()?,
                Some(_) |
                None => NFA::epsilon()
            }; 
            nfa = NFA::union(nfa, rhs);
        }

        Ok(nfa)
    }

    fn parse_concatenation(&mut self) -> Result<NFA, ParseError> {
        let mut nfa = self.parse_term()?;

        while self.peek().is_some() && 
              !self.peek_multiple(&[Token::Union, Token::RParen]) {
            let rhs = self.parse_term()?;
            nfa = NFA::concatenate(nfa, rhs);
        }

        Ok(nfa)
    }

    fn parse_term(&mut self) -> Result<NFA, ParseError> {
        let mut nfa = self.parse_factor()?;

        if let Some(postfix) = self.peek_postfix() {
            self.consume(); //Consume postfix
            match postfix {
                Token::Star => nfa = NFA::kleene_star(nfa),
                _ => unreachable!()
            }
        }

        Ok(nfa)
    }

    fn parse_factor(&mut self) -> Result<NFA, ParseError> {
        match self.peek() {
            Some(Token::LParen) => {
                self.consume_if(Token::LParen); //Consume LParen
                let nfa = self.parse_alternation()?;
                if !self.consume_if(Token::RParen) {
                    return Err(ParseError::MismatchedParentheses)
                }
                Ok(nfa)
            },
            Some(Token::Literal(c)) => {
                let c = *c;
                self.consume(); //Consume literal
                let nfa = NFA::literal(c);
                Ok(nfa)
            },
            Some(Token::Union) |
            Some(Token::RParen) |
            None => {
                Ok(NFA::epsilon())
            }
            Some(t) => Err(ParseError::UnexpectedToken(*t))
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.first()
    }

    fn consume(&mut self) {
        self.tokens.remove(0);
    }

    fn consume_if(&mut self, expected: Token) -> bool{
        if self.peek() == Some(&expected){
            self.consume();
            true
        } else {
            false
        }
    }

    fn peek_multiple(&self, tokens: &[Token]) -> bool {
        if let Some(current) = self.peek() {
            tokens.iter().any(|token| token == current)
        } else {
            false
        }
    }

    fn peek_postfix(&self) -> Option<Token> {
        match self.peek() {
            Some(Token::Star) => Some(Token::Star),
            _ => None,
        }
    }

}