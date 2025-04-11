use crate::{
    Lexer,
    token::Token,
    errors::ParseError
};
use crate::nfa::NFA;

#[derive(Debug)]
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
        let mut nfa = match self.parse_term() {
            Ok(nfa) => nfa,
            Err(ParseError::UnexpectedToken(Token::Union)) => NFA::epsilon(),
            Err(e) => return Err(e),
        };

        while let Some(Ok(token)) = self.lexer.peek() {
            match token {
                Token::Union => {
                    self.lexer.next();
                    let rhs = match self.parse_term() {
                        Ok(nfa) => nfa,
                        Err(ParseError::UnexpectedEOF) => NFA::epsilon(),
                        Err(e) => return Err(e),
                    };

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
            None => {
                //This is the original
                // return Err(ParseError::UnexpectedEOF) 
                //This was changed because empty strings are an edge case which should be epsilon transitions
                // e.g. "" is an epsilon transition
                //If things break its because there is infinite recursion since there are technically
                //An infinite number of empty strings in any given string
                return Ok(NFA::epsilon())
            },
        };

        match token {
            Token::Literal(c) => Ok(NFA::literal(c)),
            Token::LParen => {
                let nfa = match self.parse_expr() {
                    Ok(nfa) => nfa,
                    Err(ParseError::UnexpectedToken(Token::RParen)) => return Ok(NFA::epsilon()),
                    Err(e) => return Err(e),
                };
                
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_parsing() {
        let test_regex = "a|b";

        let lexer = Lexer::new(test_regex.to_string());
        let mut parser = Parser::new(lexer);

        match parser.parse_term() {
            Err(_) => panic!(),
            _ => (),
        };

        parser.lexer.peek();

        let third = match parser.lexer.peek() {
            Some(Ok(n)) => n,
            _ => panic!(),
        };

        let expected = Token::Union;

        assert_eq!(third, expected);


    }
}