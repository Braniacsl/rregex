use crate::lexer::{
    Lexer,
    token::Token,
    errors::ParseError
};
use crate::parser::nfa::NFA;

pub(crate) mod nfa;

pub struct Parser {
    lexer: Lexer
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        Parser {
            lexer
        }
    }

    pub fn parse(&mut self) -> Result<NFA, ParseError>{
        self.parse_expr()
    }

    fn parse_expr(&mut self) -> Result<NFA, ParseError> {
        let mut nfa = self.parse_term()?;

        while let Some(Ok(token)) = self.lexer.peek() {
            match token {
                Token::Union => {
                    self.lexer.next();
                    let rhs = self.parse_term()?;
                    nfa = NFA::union(nfa, rhs);
                }
                _ => break,
            }
        }

        Ok(nfa)
    }

    fn parse_term(&mut self) -> Result<NFA, ParseError> {
        let mut nfa = self.parse_factor()?;

        while let Some(Ok(token)) = self.lexer.peek() {
            match token {
                Token::Literal(_) | Token::LParen => {
                    let rhs = self.parse_term()?;
                    nfa = NFA::concatenate(nfa, rhs);
                } 
                _ => break,
            }
        }

        Ok(nfa)
    }

    fn parse_factor(&mut self) -> Result<NFA, ParseError> {
        let token = match self.lexer.next() {
            Some(Ok(token)) => token,
            Some(Err(e)) => return Err(e),
            None => return Err(ParseError::UnexpectedEOF),
        };

        match token {
            Token::Literal(c) => Ok(NFA::literal(c)),
            Token::LParen => {
                let nfa = self.parse_expr()?;
                
                if let Some(Ok(Token::RParen)) = self.lexer.next() {
                    Ok(nfa)
                }
                else {
                    Err(ParseError::MismatchedParentheses)
                }
            },
            Token::Star => {
                let nfa = self.parse()?;
                Ok(NFA::kleene_star(nfa))
            },
            _ => Err(ParseError::UnexpectedToken(token)),
        }
    }
}