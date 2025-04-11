use crate::token::Token;
use crate::errors::ParseError;

#[derive(Debug)]
enum LexerState{
    Empty,
    Peeked(Token),
}

#[derive(Debug)]
pub struct Lexer{
    input: String,
    state: LexerState,
}

impl Lexer{
    pub fn new(input: String) -> Self {
        Lexer {
            input: input.chars().rev().collect::<String>(),
            state: LexerState::Empty,
        }
    }

    pub fn peek(&mut self) -> Option<Result<Token, ParseError>> {
        match &self.state {
            LexerState::Empty => {
                let token = self.next_token();

                self.state = match token {
                    Some(Ok(t)) => LexerState::Peeked(t),
                    Some(Err(e)) => return Some(Err(e)),
                    None => LexerState::Empty,
                };

                match &self.state {
                    LexerState::Peeked(t) => Some(Ok(*t)),
                    _ => None,
                }
            },
            LexerState::Peeked(t) => Some(Ok(*t)),
        }
    }

    pub fn next(&mut self) -> Option<Result<Token, ParseError>> {
        match std::mem::replace(&mut self.state, LexerState::Empty) {
            LexerState::Empty => self.next_token(),
            LexerState::Peeked(t) => {
                self.state = LexerState::Empty;
                Some(Ok(t))
            },
        }
    }

    pub fn next_token(&mut self) -> Option<Result<Token, ParseError>> {
        if self.input.is_empty() { return None }

        let cur_char = self.input.pop();

        match cur_char {
            Some(c) if c.is_alphabetic() => Some(Ok(Token::Literal(c))),
            Some('|') => Some(Ok(Token::Union)),
            Some('*') => Some(Ok(Token::Star)),
            Some('(') => Some(Ok(Token::LParen)),
            Some(')') => Some(Ok(Token::RParen)),
            None => None,
            c => Some(Err(ParseError::UnexpectedToken(Token::Unknown(c?)))),
        }
    }

    pub fn collect(&mut self) -> Option<Result<Vec<Token>, ParseError>> {
        let mut collected = Vec::new();

        while let Some(result) = self.next() {
            match result {
                Ok(token) => collected.push(token),
                Err(e) => return Some(Err(e)),
            }
        }

        Some(Ok(collected))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peek_next(){
        let regex = "a|b";
        let mut lexer = Lexer::new(regex.to_string());

        let result  = lexer.peek().expect("").unwrap();
        let expected = Token::Literal('a');

        assert_eq!(result, expected);

        let result = lexer.next().expect("").unwrap();
        
        assert_eq!(result, expected);
    }

    #[test]
    fn test_double_peek() {
        let regex = "a|b";
        let mut lexer = Lexer::new(regex.to_string());

        lexer.next().expect("").unwrap();
        dbg!(&lexer);
        lexer.peek().expect("").unwrap();
        dbg!(&lexer);
        lexer.next().expect("").unwrap();
        dbg!(&lexer);
        let peek_a = lexer.peek().expect("").unwrap();
        dbg!(&lexer);
        let expected = Token::Literal('b');

        assert_eq!(peek_a, expected);
    }

    #[test]
    fn test_lexer_collect() {
        let mut lexer = Lexer::new("a|b".to_string());

        let vec = lexer.collect().unwrap();

        dbg!(vec);
    }
}