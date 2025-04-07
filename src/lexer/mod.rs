use crate::lexer::token::Token;
use crate::lexer::errors::ParseError;

pub(crate) mod token;
pub mod errors;

enum LexerState{
    Empty,
    Peeked(Token),
    Consumed,
}

pub struct Lexer{
    input: String,
    state: LexerState,
}

impl Lexer{
    pub fn new(input: String) -> Self {
        Lexer {
            input: input,
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
            LexerState::Consumed => None,
        }
    }

    pub fn next(&mut self) -> Option<Result<Token, ParseError>> {
        match std::mem::replace(&mut self.state, LexerState::Empty) {
            LexerState::Empty => self.next_token(),
            LexerState::Peeked(t) => {
                self.state = LexerState::Consumed;
                Some(Ok(t))
            },
            LexerState::Consumed => None,
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
}

