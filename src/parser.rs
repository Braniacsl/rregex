use crate::{
    Lexer,
    token::Token,
    errors::ParseError
};
use crate::nfa::NFA;

// #[derive(Debug)]
// pub struct Parser {
//     lexer: Lexer
// }

// impl Parser {
//     pub fn new(lexer: Lexer) -> Self {
//         Parser {
//             lexer
//         }
//     }

//     pub fn parse(&mut self) -> Result<NFA, ParseError>{
//         self.parse_expr()
//     }

//     fn parse_expr(&mut self) -> Result<NFA, ParseError> {
//         let mut nfa = match self.parse_term() {
//             Ok(nfa) => nfa,
//             Err(ParseError::UnexpectedToken(Token::Union)) => NFA::epsilon(),
//             Err(e) => return Err(e),
//         };

//         while let Some(Ok(token)) = self.lexer.peek() {
//             match token {
//                 Token::Union => {
//                     self.lexer.next();
//                     let rhs = match self.parse_term() {
//                         Ok(nfa) => nfa,
//                         Err(ParseError::UnexpectedEOF) => NFA::epsilon(),
//                         Err(e) => return Err(e),
//                     };

//                     nfa = NFA::union(nfa, rhs);
//                 }
//                 _ => break,
//             }
//         }

//         Ok(nfa)
//     }

//     fn parse_term(&mut self) -> Result<NFA, ParseError> {
//         let mut nfa = self.parse_factor()?;

//         while let Some(Ok(token)) = self.lexer.peek() {
//             match token {
//                 Token::Literal(_) | Token::LParen => {
//                     let rhs = self.parse_term()?;
//                     nfa = NFA::concatenate(nfa, rhs);
//                 } 
//                 _ => break,
//             }
//         }

//         Ok(nfa)
//     }

//     fn parse_factor(&mut self) -> Result<NFA, ParseError> {
//         let token = match self.lexer.next() {
//             Some(Ok(token)) => token,
//             Some(Err(e)) => return Err(e),
//             None => {
//                 //This is the original
//                 // return Err(ParseError::UnexpectedEOF) 
//                 //This was changed because empty strings are an edge case which should be epsilon transitions
//                 // e.g. "" is an epsilon transition
//                 //If things break its because there is infinite recursion since there are technically
//                 //An infinite number of empty strings in any given string
//                 return Ok(NFA::epsilon())
//             },
//         };

//         match token {
//             Token::Literal(c) => Ok(NFA::literal(c)),
//             Token::LParen => {
//                 let nfa = match self.parse_expr() {
//                     Ok(nfa) => nfa,
//                     Err(ParseError::UnexpectedToken(Token::RParen)) => return Ok(NFA::epsilon()),
//                     Err(e) => return Err(e),
//                 };
                
//                 if let Some(Ok(Token::RParen)) = self.lexer.next() {
//                     Ok(nfa)
//                 }
//                 else {
//                     Err(ParseError::MismatchedParentheses)
//                 }
//             },
//             Token::Star => {
//                 let nfa = self.parse()?;
//                 Ok(NFA::kleene_star(nfa))
//             },
//             _ => Err(ParseError::UnexpectedToken(token)),
//         }
//     }
// }

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
            let rhs = self.parse_concatenation()?;
            nfa = NFA::union(nfa, rhs);
        }

        Ok(nfa)
    }

    fn parse_concatenation(&mut self) -> Result<NFA, ParseError> {
        let mut nfa = self.parse_term()?;

        while self.peek().is_some() && 
              !self.peek_multiple(&[Token::Union, Token::RParen]) {
            let rhs = self.parse_term()?;
            nfa = NFA::concatenate(rhs, nfa);
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
                self.consume(); //Consume LParen
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
            None => {
                Ok(NFA::epsilon())
            }
            Some(t) => Err(ParseError::UnexpectedToken(*t))
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.last()
    }

    fn consume(&mut self) {
        self.tokens.pop();
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let mut lexer = Lexer::new("a".to_string());
        let mut parser = Parser::new(&mut lexer);

        let nfa = parser.parse().unwrap();

        dbg!(nfa);
    }
}